//! `pulp` is a safe abstraction over SIMD instructions, that allows you to write a function once
//! and dispatch to equivalent vectorized versions based on the features detected at runtime.
//!
//! # Autovectorization example
//!
//! ```
//! use pulp::Arch;
//!
//! let mut v = (0..1000).map(|i| i as f64).collect::<Vec<_>>();
//! let arch = Arch::new();
//!
//! arch.dispatch(|| {
//!     for x in &mut v {
//!         *x *= 2.0;
//!     }
//! });
//!
//! for (i, x) in v.into_iter().enumerate() {
//!     assert_eq!(x, 2.0 * i as f64);
//! }
//! ```
//!
//! # Manual vectorization example
//!
//! ```
//! use pulp::{Arch, Simd, WithSimd};
//!
//! struct TimesThree<'a>(&'a mut [f64]);
//! impl<'a> WithSimd for TimesThree<'a> {
//!     type Output = ();
//!
//!     #[inline(always)]
//!     fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
//!         let v = self.0;
//!         let (head, tail) = S::as_mut_simd_f64s(v);
//!
//!         let three = simd.splat_f64s(3.0);
//!         for x in head {
//!             *x = simd.mul_f64s(three, *x);
//!         }
//!
//!         for x in tail {
//!             *x = *x * 3.0;
//!         }
//!     }
//! }
//!
//! let mut v = (0..1000).map(|i| i as f64).collect::<Vec<_>>();
//! let arch = Arch::new();
//!
//! arch.dispatch(TimesThree(&mut v));
//!
//! for (i, x) in v.into_iter().enumerate() {
//!     assert_eq!(x, 3.0 * i as f64);
//! }
//! ```

#![allow(
    non_camel_case_types,
    unknown_lints,
    clippy::zero_prefixed_literal,
    clippy::identity_op,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::missing_transmute_annotations
)]
#![cfg_attr(
    all(feature = "nightly", any(target_arch = "x86", target_arch = "x86_64")),
    feature(stdarch_x86_avx512),
    feature(avx512_target_feature)
)]
#![cfg_attr(
    all(feature = "nightly", any(target_arch = "aarch64")),
    feature(stdarch_neon_i8mm),
    feature(stdarch_neon_sm4),
    feature(stdarch_neon_ftts),
    feature(stdarch_neon_fcma),
    feature(stdarch_neon_dotprod)
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::slice::{from_raw_parts, from_raw_parts_mut};
use num_complex::Complex;
use seal::Seal;

/// Requires the first non-lifetime generic parameter, as well as the function's
/// first input parameter to be the SIMD type.
/// Also currently requires that all the lifetimes be explicitly specified.
#[cfg(feature = "macro")]
#[cfg_attr(docsrs, doc(cfg(feature = "macro")))]
pub use pulp_macro::with_simd;

pub type c32 = Complex<f32>;
pub type c64 = Complex<f64>;

#[derive(Debug, Copy, Clone)]
pub struct MemMask<T> {
    mask: T,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    load: Option<unsafe extern "C" fn()>,
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    store: Option<unsafe extern "C" fn()>,
}

impl<T> From<T> for MemMask<T> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            mask: value,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            load: None,
            #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
            store: None,
        }
    }
}

impl<T: Copy> MemMask<T> {
    #[inline]
    pub fn mask(self) -> T {
        self.mask
    }
}

mod seal {
    pub trait Seal {}
}

pub trait NullaryFnOnce {
    type Output;

    fn call(self) -> Self::Output;
}

impl<R, F: FnOnce() -> R> NullaryFnOnce for F {
    type Output = R;

    #[inline(always)]
    fn call(self) -> Self::Output {
        self()
    }
}

pub trait WithSimd {
    type Output;

    fn with_simd<S: Simd>(self, simd: S) -> Self::Output;
}

impl<F: NullaryFnOnce> WithSimd for F {
    type Output = F::Output;

    #[inline(always)]
    fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
        let _simd = &simd;
        self.call()
    }
}

// a0,0 ... a0,m-1
// ...
// an-1,0 ... an-1,m-1
#[inline(always)]
unsafe fn interleave_fallback<Unit: Pod, Reg: Pod, AosReg>(x: AosReg) -> AosReg {
    assert!(size_of::<AosReg>() % size_of::<Reg>() == 0);
    assert!(size_of::<Reg>() % size_of::<Unit>() == 0);
    assert!(!core::mem::needs_drop::<AosReg>());

    if const { size_of::<AosReg>() == size_of::<Reg>() } {
        x
    } else {
        let mut y = core::ptr::read(&x);

        let n = const { size_of::<AosReg>() / size_of::<Reg>() };
        let m = const { size_of::<Reg>() / size_of::<Unit>() };

        unsafe {
            let y = (&mut y) as *mut _ as *mut Unit;
            let x = (&x) as *const _ as *const Unit;
            for j in 0..m {
                for i in 0..n {
                    *y.add(i + n * j) = *x.add(j + i * m);
                }
            }
        }

        y
    }
}

#[inline(always)]
unsafe fn deinterleave_fallback<Unit: Pod, Reg: Pod, SoaReg>(y: SoaReg) -> SoaReg {
    assert!(size_of::<SoaReg>() % size_of::<Reg>() == 0);
    assert!(size_of::<Reg>() % size_of::<Unit>() == 0);
    assert!(!core::mem::needs_drop::<SoaReg>());

    if const { size_of::<SoaReg>() == size_of::<Reg>() } {
        y
    } else {
        let mut x = core::ptr::read(&y);

        let n = const { size_of::<SoaReg>() / size_of::<Reg>() };
        let m = const { size_of::<Reg>() / size_of::<Unit>() };

        unsafe {
            let y = (&y) as *const _ as *const Unit;
            let x = (&mut x) as *mut _ as *mut Unit;
            for j in 0..m {
                for i in 0..n {
                    *x.add(j + i * m) = *y.add(i + n * j);
                }
            }
        }

        x
    }
}

/// Types that allow \[de\]interleaving.
///
/// # Safety
/// Instances of this type passed to simd \[de\]interleave functions must be `Pod`.
pub unsafe trait Interleave {}
unsafe impl<T: Pod> Interleave for T {}

pub trait Simd: Seal + Debug + Copy + Send + Sync + 'static {
    const IS_SCALAR: bool = false;

    const U64_LANES: usize = size_of::<Self::u64s>() / size_of::<u64>();
    const I64_LANES: usize = size_of::<Self::i64s>() / size_of::<i64>();
    const F64_LANES: usize = size_of::<Self::f64s>() / size_of::<f64>();
    const C64_LANES: usize = size_of::<Self::c64s>() / size_of::<c64>();

    const U32_LANES: usize = size_of::<Self::u32s>() / size_of::<u32>();
    const I32_LANES: usize = size_of::<Self::i32s>() / size_of::<i32>();
    const F32_LANES: usize = size_of::<Self::f32s>() / size_of::<f32>();
    const C32_LANES: usize = size_of::<Self::c32s>() / size_of::<c32>();

    const REGISTER_COUNT: usize;

    type m32s: Debug + Copy + Send + Sync + Zeroable + NoUninit + 'static;
    type f32s: Debug + Copy + Send + Sync + Pod + 'static;
    type c32s: Debug + Copy + Send + Sync + Pod + 'static;
    type i32s: Debug + Copy + Send + Sync + Pod + 'static;
    type u32s: Debug + Copy + Send + Sync + Pod + 'static;

    type m64s: Debug + Copy + Send + Sync + Zeroable + NoUninit + 'static;
    type f64s: Debug + Copy + Send + Sync + Pod + 'static;
    type c64s: Debug + Copy + Send + Sync + Pod + 'static;
    type i64s: Debug + Copy + Send + Sync + Pod + 'static;
    type u64s: Debug + Copy + Send + Sync + Pod + 'static;

    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output;

    #[inline(always)]
    fn as_simd_f32s(slice: &[f32]) -> (&[Self::f32s], &[f32]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_f32s(slice: &mut [f32]) -> (&mut [Self::f32s], &mut [f32]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_f32s(
        slice: &mut [MaybeUninit<f32>],
    ) -> (&mut [MaybeUninit<Self::f32s>], &mut [MaybeUninit<f32>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_c32s(slice: &[c32]) -> (&[Self::c32s], &[c32]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_c32s(slice: &mut [c32]) -> (&mut [Self::c32s], &mut [c32]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_c32s(
        slice: &mut [MaybeUninit<c32>],
    ) -> (&mut [MaybeUninit<Self::c32s>], &mut [MaybeUninit<c32>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_i32s(slice: &[i32]) -> (&[Self::i32s], &[i32]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_i32s(slice: &mut [i32]) -> (&mut [Self::i32s], &mut [i32]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_i32s(
        slice: &mut [MaybeUninit<i32>],
    ) -> (&mut [MaybeUninit<Self::i32s>], &mut [MaybeUninit<i32>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_u32s(slice: &[u32]) -> (&[Self::u32s], &[u32]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_u32s(slice: &mut [u32]) -> (&mut [Self::u32s], &mut [u32]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_u32s(
        slice: &mut [MaybeUninit<u32>],
    ) -> (&mut [MaybeUninit<Self::u32s>], &mut [MaybeUninit<u32>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_f64s(slice: &[f64]) -> (&[Self::f64s], &[f64]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_f64s(slice: &mut [f64]) -> (&mut [Self::f64s], &mut [f64]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_f64s(
        slice: &mut [MaybeUninit<f64>],
    ) -> (&mut [MaybeUninit<Self::f64s>], &mut [MaybeUninit<f64>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_c64s(slice: &[c64]) -> (&[Self::c64s], &[c64]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_c64s(slice: &mut [c64]) -> (&mut [Self::c64s], &mut [c64]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_c64s(
        slice: &mut [MaybeUninit<c64>],
    ) -> (&mut [MaybeUninit<Self::c64s>], &mut [MaybeUninit<c64>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_i64s(slice: &[i64]) -> (&[Self::i64s], &[i64]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_i64s(slice: &mut [i64]) -> (&mut [Self::i64s], &mut [i64]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_i64s(
        slice: &mut [MaybeUninit<i64>],
    ) -> (&mut [MaybeUninit<Self::i64s>], &mut [MaybeUninit<i64>]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_simd_u64s(slice: &[u64]) -> (&[Self::u64s], &[u64]) {
        unsafe { split_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_simd_u64s(slice: &mut [u64]) -> (&mut [Self::u64s], &mut [u64]) {
        unsafe { split_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_simd_u64s(
        slice: &mut [MaybeUninit<u64>],
    ) -> (&mut [MaybeUninit<Self::u64s>], &mut [MaybeUninit<u64>]) {
        unsafe { split_mut_slice(slice) }
    }

    #[inline(always)]
    fn as_rsimd_f32s(slice: &[f32]) -> (&[f32], &[Self::f32s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_f32s(slice: &mut [f32]) -> (&mut [f32], &mut [Self::f32s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_f32s(
        slice: &mut [MaybeUninit<f32>],
    ) -> (&mut [MaybeUninit<f32>], &mut [MaybeUninit<Self::f32s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_c32s(slice: &[c32]) -> (&[c32], &[Self::c32s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_c32s(slice: &mut [c32]) -> (&mut [c32], &mut [Self::c32s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_c32s(
        slice: &mut [MaybeUninit<c32>],
    ) -> (&mut [MaybeUninit<c32>], &mut [MaybeUninit<Self::c32s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_i32s(slice: &[i32]) -> (&[i32], &[Self::i32s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_i32s(slice: &mut [i32]) -> (&mut [i32], &mut [Self::i32s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_i32s(
        slice: &mut [MaybeUninit<i32>],
    ) -> (&mut [MaybeUninit<i32>], &mut [MaybeUninit<Self::i32s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_u32s(slice: &[u32]) -> (&[u32], &[Self::u32s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_u32s(slice: &mut [u32]) -> (&mut [u32], &mut [Self::u32s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_u32s(
        slice: &mut [MaybeUninit<u32>],
    ) -> (&mut [MaybeUninit<u32>], &mut [MaybeUninit<Self::u32s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_f64s(slice: &[f64]) -> (&[f64], &[Self::f64s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_f64s(slice: &mut [f64]) -> (&mut [f64], &mut [Self::f64s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_f64s(
        slice: &mut [MaybeUninit<f64>],
    ) -> (&mut [MaybeUninit<f64>], &mut [MaybeUninit<Self::f64s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_c64s(slice: &[c64]) -> (&[c64], &[Self::c64s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_c64s(slice: &mut [c64]) -> (&mut [c64], &mut [Self::c64s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_c64s(
        slice: &mut [MaybeUninit<c64>],
    ) -> (&mut [MaybeUninit<c64>], &mut [MaybeUninit<Self::c64s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_i64s(slice: &[i64]) -> (&[i64], &[Self::i64s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_i64s(slice: &mut [i64]) -> (&mut [i64], &mut [Self::i64s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_i64s(
        slice: &mut [MaybeUninit<i64>],
    ) -> (&mut [MaybeUninit<i64>], &mut [MaybeUninit<Self::i64s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_rsimd_u64s(slice: &[u64]) -> (&[u64], &[Self::u64s]) {
        unsafe { rsplit_slice(slice) }
    }
    #[inline(always)]
    fn as_mut_rsimd_u64s(slice: &mut [u64]) -> (&mut [u64], &mut [Self::u64s]) {
        unsafe { rsplit_mut_slice(slice) }
    }
    #[inline(always)]
    fn as_uninit_mut_rsimd_u64s(
        slice: &mut [MaybeUninit<u64>],
    ) -> (&mut [MaybeUninit<u64>], &mut [MaybeUninit<Self::u64s>]) {
        unsafe { rsplit_mut_slice(slice) }
    }

    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    #[inline(always)]
    unsafe fn mask_load_ptr_i32s(self, mask: MemMask<Self::m32s>, ptr: *const i32) -> Self::i32s {
        self.transmute_i32s_u32s(self.mask_load_ptr_u32s(mask, ptr as *const u32))
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    #[inline(always)]
    unsafe fn mask_load_ptr_f32s(self, mask: MemMask<Self::m32s>, ptr: *const f32) -> Self::f32s {
        self.transmute_f32s_u32s(self.mask_load_ptr_u32s(mask, ptr as *const u32))
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    unsafe fn mask_load_ptr_u32s(self, mask: MemMask<Self::m32s>, ptr: *const u32) -> Self::u32s;
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    unsafe fn mask_load_ptr_c32s(self, mask: MemMask<Self::m32s>, ptr: *const c32) -> Self::c32s;
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    #[inline(always)]
    unsafe fn mask_store_ptr_i32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut i32,
        values: Self::i32s,
    ) {
        self.mask_store_ptr_u32s(mask, ptr as *mut u32, self.transmute_u32s_i32s(values));
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    #[inline(always)]
    unsafe fn mask_store_ptr_f32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut f32,
        values: Self::f32s,
    ) {
        self.mask_store_ptr_u32s(mask, ptr as *mut u32, self.transmute_u32s_f32s(values));
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    unsafe fn mask_store_ptr_u32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut u32,
        values: Self::u32s,
    );
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    unsafe fn mask_store_ptr_c32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut c32,
        values: Self::c32s,
    );

    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    #[inline(always)]
    unsafe fn mask_load_ptr_i64s(self, mask: MemMask<Self::m64s>, ptr: *const i64) -> Self::i64s {
        self.transmute_i64s_u64s(self.mask_load_ptr_u64s(mask, ptr as *const u64))
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    #[inline(always)]
    unsafe fn mask_load_ptr_f64s(self, mask: MemMask<Self::m64s>, ptr: *const f64) -> Self::f64s {
        self.transmute_f64s_u64s(self.mask_load_ptr_u64s(mask, ptr as *const u64))
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    unsafe fn mask_load_ptr_u64s(self, mask: MemMask<Self::m64s>, ptr: *const u64) -> Self::u64s;
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::read`].
    unsafe fn mask_load_ptr_c64s(self, mask: MemMask<Self::m64s>, ptr: *const c64) -> Self::c64s;
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    #[inline(always)]
    unsafe fn mask_store_ptr_i64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut i64,
        values: Self::i64s,
    ) {
        self.mask_store_ptr_u64s(mask, ptr as *mut u64, self.transmute_u64s_i64s(values));
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    #[inline(always)]
    unsafe fn mask_store_ptr_f64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut f64,
        values: Self::f64s,
    ) {
        self.mask_store_ptr_u64s(mask, ptr as *mut u64, self.transmute_u64s_f64s(values));
    }
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    unsafe fn mask_store_ptr_u64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut u64,
        values: Self::u64s,
    );
    /// # Safety
    ///
    /// Addresses corresponding to enabled lanes in the mask have the same restrictions as
    /// [`core::ptr::write`].
    unsafe fn mask_store_ptr_c64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut c64,
        values: Self::c64s,
    );

    fn partial_load_u32s(self, slice: &[u32]) -> Self::u32s;
    fn partial_store_u32s(self, slice: &mut [u32], values: Self::u32s);
    fn partial_load_u64s(self, slice: &[u64]) -> Self::u64s;
    fn partial_store_u64s(self, slice: &mut [u64], values: Self::u64s);

    #[inline(always)]
    fn deinterleave_shfl_f64s<T: Interleave>(self, values: T) -> T {
        unsafe { deinterleave_fallback::<f64, Self::f64s, T>(values) }
    }
    #[inline(always)]
    fn interleave_shfl_f64s<T: Interleave>(self, values: T) -> T {
        unsafe { interleave_fallback::<f64, Self::f64s, T>(values) }
    }
    #[inline(always)]
    fn deinterleave_shfl_f32s<T: Interleave>(self, values: T) -> T {
        unsafe { deinterleave_fallback::<f32, Self::f32s, T>(values) }
    }
    #[inline(always)]
    fn interleave_shfl_f32s<T: Interleave>(self, values: T) -> T {
        unsafe { interleave_fallback::<f32, Self::f32s, T>(values) }
    }

    #[inline(always)]
    fn partial_load_i32s(self, slice: &[i32]) -> Self::i32s {
        cast(self.partial_load_u32s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_i32s(self, slice: &mut [i32], values: Self::i32s) {
        self.partial_store_u32s(bytemuck::cast_slice_mut(slice), cast(values))
    }
    #[inline(always)]
    fn partial_load_i64s(self, slice: &[i64]) -> Self::i64s {
        cast(self.partial_load_u64s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_i64s(self, slice: &mut [i64], values: Self::i64s) {
        self.partial_store_u64s(bytemuck::cast_slice_mut(slice), cast(values))
    }

    #[inline(always)]
    fn partial_load_f32s(self, slice: &[f32]) -> Self::f32s {
        cast(self.partial_load_u32s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_f32s(self, slice: &mut [f32], values: Self::f32s) {
        self.partial_store_u32s(bytemuck::cast_slice_mut(slice), cast(values))
    }
    #[inline(always)]
    fn partial_load_f64s(self, slice: &[f64]) -> Self::f64s {
        cast(self.partial_load_u64s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_f64s(self, slice: &mut [f64], values: Self::f64s) {
        self.partial_store_u64s(bytemuck::cast_slice_mut(slice), cast(values))
    }

    #[inline(always)]
    fn partial_load_c32s(self, slice: &[c32]) -> Self::c32s {
        cast(self.partial_load_f64s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_c32s(self, slice: &mut [c32], values: Self::c32s) {
        self.partial_store_f64s(bytemuck::cast_slice_mut(slice), cast(values))
    }
    #[inline(always)]
    fn partial_load_c64s(self, slice: &[c64]) -> Self::c64s {
        cast(self.partial_load_f64s(bytemuck::cast_slice(slice)))
    }
    #[inline(always)]
    fn partial_store_c64s(self, slice: &mut [c64], values: Self::c64s) {
        self.partial_store_f64s(bytemuck::cast_slice_mut(slice), cast(values))
    }

    #[inline(always)]
    fn first_true_m32s(self, mask: Self::m32s) -> usize {
        if const { size_of::<Self::m32s>() == size_of::<Self::u32s>() } {
            let mask: Self::u32s = bytemuck::cast(mask);
            let slice = bytemuck::cast_slice::<Self::u32s, u32>(core::slice::from_ref(&mask));
            let mut i = 0;
            for &x in slice.iter() {
                if x != 0 {
                    break;
                }
                i += 1;
            }
            i
        } else if const { size_of::<Self::m32s>() == size_of::<u8>() } {
            let mask: u8 = bytemuck::cast(mask);
            mask.leading_zeros() as usize
        } else if const { size_of::<Self::m32s>() == size_of::<u16>() } {
            let mask: u16 = bytemuck::cast(mask);
            mask.leading_zeros() as usize
        } else {
            panic!()
        }
    }
    #[inline(always)]
    fn first_true_m64s(self, mask: Self::m64s) -> usize {
        if const { size_of::<Self::m64s>() == size_of::<Self::u64s>() } {
            let mask: Self::u64s = bytemuck::cast(mask);
            let slice = bytemuck::cast_slice::<Self::u64s, u64>(core::slice::from_ref(&mask));
            let mut i = 0;
            for &x in slice.iter() {
                if x != 0 {
                    break;
                }
                i += 1;
            }
            i
        } else if const { size_of::<Self::m64s>() == size_of::<u8>() } {
            let mask: u8 = bytemuck::cast(mask);
            mask.leading_zeros() as usize
        } else if const { size_of::<Self::m64s>() == size_of::<u16>() } {
            let mask: u16 = bytemuck::cast(mask);
            mask.leading_zeros() as usize
        } else {
            panic!()
        }
    }

    #[inline(always)]
    fn mask_between_m64s(self, start: u64, end: u64) -> MemMask<Self::m64s> {
        let iota: Self::u64s =
            const { unsafe { core::mem::transmute_copy(&<u64 as Iota64>::IOTA) } };
        self.and_m64s(
            self.greater_than_or_equal_u64s(iota, self.splat_u64s(start)),
            self.less_than_u64s(iota, self.splat_u64s(end)),
        )
        .into()
    }

    #[inline(always)]
    fn mask_between_m32s(self, start: u32, end: u32) -> MemMask<Self::m32s> {
        let iota: Self::u32s =
            const { unsafe { core::mem::transmute_copy(&<u32 as Iota32>::IOTA) } };
        self.and_m32s(
            self.greater_than_or_equal_u32s(iota, self.splat_u32s(start)),
            self.less_than_u32s(iota, self.splat_u32s(end)),
        )
        .into()
    }

    fn not_m32s(self, a: Self::m32s) -> Self::m32s;
    fn and_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;
    fn or_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;
    fn xor_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;

    fn not_m64s(self, a: Self::m64s) -> Self::m64s;
    fn and_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;
    fn or_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;
    fn xor_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;

    fn not_u32s(self, a: Self::u32s) -> Self::u32s;
    fn and_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn or_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn xor_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;

    fn not_u64s(self, a: Self::u64s) -> Self::u64s;
    fn and_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn or_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn xor_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;

    fn select_u32s_m32s(
        self,
        mask: Self::m32s,
        if_true: Self::u32s,
        if_false: Self::u32s,
    ) -> Self::u32s;
    fn select_u64s_m64s(
        self,
        mask: Self::m64s,
        if_true: Self::u64s,
        if_false: Self::u64s,
    ) -> Self::u64s;

    #[inline]
    fn not_i32s(self, a: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(self.not_u32s(self.transmute_u32s_i32s(a)))
    }
    #[inline]
    fn and_i32s(self, a: Self::i32s, b: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(
            self.and_u32s(self.transmute_u32s_i32s(a), self.transmute_u32s_i32s(b)),
        )
    }
    #[inline]
    fn or_i32s(self, a: Self::i32s, b: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(
            self.or_u32s(self.transmute_u32s_i32s(a), self.transmute_u32s_i32s(b)),
        )
    }
    #[inline]
    fn xor_i32s(self, a: Self::i32s, b: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(
            self.xor_u32s(self.transmute_u32s_i32s(a), self.transmute_u32s_i32s(b)),
        )
    }

    #[inline]
    fn not_i64s(self, a: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(self.not_u64s(self.transmute_u64s_i64s(a)))
    }
    #[inline]
    fn and_i64s(self, a: Self::i64s, b: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(
            self.and_u64s(self.transmute_u64s_i64s(a), self.transmute_u64s_i64s(b)),
        )
    }
    #[inline]
    fn or_i64s(self, a: Self::i64s, b: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(
            self.or_u64s(self.transmute_u64s_i64s(a), self.transmute_u64s_i64s(b)),
        )
    }
    #[inline]
    fn xor_i64s(self, a: Self::i64s, b: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(
            self.xor_u64s(self.transmute_u64s_i64s(a), self.transmute_u64s_i64s(b)),
        )
    }

    #[inline]
    fn not_f32s(self, a: Self::f32s) -> Self::f32s {
        self.transmute_f32s_u32s(self.not_u32s(self.transmute_u32s_f32s(a)))
    }
    #[inline]
    fn and_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        self.transmute_f32s_u32s(
            self.and_u32s(self.transmute_u32s_f32s(a), self.transmute_u32s_f32s(b)),
        )
    }
    #[inline]
    fn or_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        self.transmute_f32s_u32s(
            self.or_u32s(self.transmute_u32s_f32s(a), self.transmute_u32s_f32s(b)),
        )
    }
    #[inline]
    fn xor_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        self.transmute_f32s_u32s(
            self.xor_u32s(self.transmute_u32s_f32s(a), self.transmute_u32s_f32s(b)),
        )
    }

    #[inline]
    fn not_f64s(self, a: Self::f64s) -> Self::f64s {
        self.transmute_f64s_u64s(self.not_u64s(self.transmute_u64s_f64s(a)))
    }
    #[inline]
    fn and_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        self.transmute_f64s_u64s(
            self.and_u64s(self.transmute_u64s_f64s(a), self.transmute_u64s_f64s(b)),
        )
    }
    #[inline]
    fn or_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        self.transmute_f64s_u64s(
            self.or_u64s(self.transmute_u64s_f64s(a), self.transmute_u64s_f64s(b)),
        )
    }
    #[inline]
    fn xor_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        self.transmute_f64s_u64s(
            self.xor_u64s(self.transmute_u64s_f64s(a), self.transmute_u64s_f64s(b)),
        )
    }

    #[inline]
    fn select_i32s_m32s(
        self,
        mask: Self::m32s,
        if_true: Self::i32s,
        if_false: Self::i32s,
    ) -> Self::i32s {
        self.transmute_i32s_u32s(self.select_u32s_m32s(
            mask,
            self.transmute_u32s_i32s(if_true),
            self.transmute_u32s_i32s(if_false),
        ))
    }
    #[inline]
    fn select_f32s_m32s(
        self,
        mask: Self::m32s,
        if_true: Self::f32s,
        if_false: Self::f32s,
    ) -> Self::f32s {
        self.transmute_f32s_u32s(self.select_u32s_m32s(
            mask,
            self.transmute_u32s_f32s(if_true),
            self.transmute_u32s_f32s(if_false),
        ))
    }
    #[inline]
    fn select_i64s_m64s(
        self,
        mask: Self::m64s,
        if_true: Self::i64s,
        if_false: Self::i64s,
    ) -> Self::i64s {
        self.transmute_i64s_u64s(self.select_u64s_m64s(
            mask,
            self.transmute_u64s_i64s(if_true),
            self.transmute_u64s_i64s(if_false),
        ))
    }
    #[inline]
    fn select_f64s_m64s(
        self,
        mask: Self::m64s,
        if_true: Self::f64s,
        if_false: Self::f64s,
    ) -> Self::f64s {
        self.transmute_f64s_u64s(self.select_u64s_m64s(
            mask,
            self.transmute_u64s_f64s(if_true),
            self.transmute_u64s_f64s(if_false),
        ))
    }

    fn splat_u32s(self, value: u32) -> Self::u32s;
    fn add_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn sub_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn less_than_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s;
    fn greater_than_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s;
    fn less_than_or_equal_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s;
    fn greater_than_or_equal_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s;
    fn wrapping_dyn_shl_u32s(self, a: Self::u32s, amount: Self::u32s) -> Self::u32s;
    fn wrapping_dyn_shr_u32s(self, a: Self::u32s, amount: Self::u32s) -> Self::u32s;
    fn widening_mul_u32s(self, a: Self::u32s, b: Self::u32s) -> (Self::u32s, Self::u32s);

    fn splat_u64s(self, value: u64) -> Self::u64s;
    fn add_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn sub_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn less_than_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s;
    fn greater_than_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s;
    fn less_than_or_equal_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s;
    fn greater_than_or_equal_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s;

    #[inline]
    fn splat_i32s(self, value: i32) -> Self::i32s {
        self.transmute_i32s_u32s(self.splat_u32s(value as u32))
    }
    #[inline]
    fn add_i32s(self, a: Self::i32s, b: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(
            self.add_u32s(self.transmute_u32s_i32s(a), self.transmute_u32s_i32s(b)),
        )
    }
    #[inline]
    fn sub_i32s(self, a: Self::i32s, b: Self::i32s) -> Self::i32s {
        self.transmute_i32s_u32s(
            self.sub_u32s(self.transmute_u32s_i32s(a), self.transmute_u32s_i32s(b)),
        )
    }

    #[inline]
    fn splat_i64s(self, value: i64) -> Self::i64s {
        self.transmute_i64s_u64s(self.splat_u64s(value as u64))
    }
    #[inline]
    fn add_i64s(self, a: Self::i64s, b: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(
            self.add_u64s(self.transmute_u64s_i64s(a), self.transmute_u64s_i64s(b)),
        )
    }
    #[inline]
    fn sub_i64s(self, a: Self::i64s, b: Self::i64s) -> Self::i64s {
        self.transmute_i64s_u64s(
            self.sub_u64s(self.transmute_u64s_i64s(a), self.transmute_u64s_i64s(b)),
        )
    }

    fn splat_f32s(self, value: f32) -> Self::f32s;
    #[inline]
    fn abs_f32s(self, a: Self::f32s) -> Self::f32s {
        self.and_f32s(self.not_f32s(self.splat_f32s(-0.0)), a)
    }
    #[inline]
    fn neg_f32s(self, a: Self::f32s) -> Self::f32s {
        self.xor_f32s(self.splat_f32s(-0.0), a)
    }
    fn add_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn sub_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn mul_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn div_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    #[inline]
    fn mul_add_e_f32s(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s {
        self.add_f32s(self.mul_f32s(a, b), c)
    }
    #[inline]
    fn f32_scalar_mul_add_e(self, a: f32, b: f32, c: f32) -> f32 {
        a * b + c
    }

    fn mul_add_f32s(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s;
    #[inline]
    fn f32_scalar_mul_add(self, a: f32, b: f32, c: f32) -> f32 {
        #[cfg(feature = "std")]
        {
            f32::mul_add(a, b, c)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fmaf(a, b, c)
        }
    }
    fn equal_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    fn less_than_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    fn less_than_or_equal_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    #[inline]
    fn greater_than_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s {
        self.less_than_f32s(b, a)
    }
    #[inline]
    fn greater_than_or_equal_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s {
        self.less_than_or_equal_f32s(b, a)
    }
    fn min_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn max_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn reduce_sum_f32s(self, a: Self::f32s) -> f32;
    fn reduce_product_f32s(self, a: Self::f32s) -> f32;
    fn reduce_min_f32s(self, a: Self::f32s) -> f32;
    fn reduce_max_f32s(self, a: Self::f32s) -> f32;

    fn splat_c32s(self, value: c32) -> Self::c32s;
    fn conj_c32s(self, a: Self::c32s) -> Self::c32s;
    fn neg_c32s(self, a: Self::c32s) -> Self::c32s;
    fn swap_re_im_c32s(self, a: Self::c32s) -> Self::c32s;
    fn add_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s;
    fn sub_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s;
    /// Computes `a * b`
    #[inline]
    fn mul_e_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        self.mul_c32s(a, b)
    }
    #[inline]
    fn c32_scalar_mul_e(self, a: c32, b: c32) -> c32 {
        a * b
    }
    fn mul_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s;
    #[inline]
    fn c32_scalar_mul(self, a: c32, b: c32) -> c32 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f32_scalar_mul_add(a_re, b_re, -a_im * b_im);
        let im = self.f32_scalar_mul_add(a_re, b_im, a_im * b_re);

        c32 { re, im }
    }
    /// Computes `conj(a) * b`
    #[inline]
    fn conj_mul_e_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        self.conj_mul_c32s(a, b)
    }
    #[inline]
    fn c32_scalar_conj_mul_e(self, a: c32, b: c32) -> c32 {
        a.conj() * b
    }
    fn conj_mul_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s;
    #[inline]
    fn c32_scalar_conj_mul(self, a: c32, b: c32) -> c32 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f32_scalar_mul_add(a_re, b_re, a_im * b_im);
        let im = self.f32_scalar_mul_add(a_re, b_im, -a_im * b_re);

        c32 { re, im }
    }

    /// Computes `a * b + c`
    #[inline]
    fn mul_add_e_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.mul_add_c32s(a, b, c)
    }
    #[inline]
    fn c32_scalar_mul_add_e(self, a: c32, b: c32, c: c32) -> c32 {
        a * b + c
    }
    fn mul_add_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s;
    #[inline]
    fn c32_scalar_mul_add(self, a: c32, b: c32, c: c32) -> c32 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f32_scalar_mul_add(a_re, b_re, self.f32_scalar_mul_add(-a_im, b_im, c.re));
        let im = self.f32_scalar_mul_add(a_re, b_im, self.f32_scalar_mul_add(a_im, b_re, c.im));

        c32 { re, im }
    }

    /// Computes `conj(a) * b + c`
    #[inline]
    fn conj_mul_add_e_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.conj_mul_add_c32s(a, b, c)
    }
    #[inline]
    fn c32_scalar_conj_mul_add_e(self, a: c32, b: c32, c: c32) -> c32 {
        a.conj() * b + c
    }
    fn conj_mul_add_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s;
    #[inline]
    fn c32_scalar_conj_mul_add(self, a: c32, b: c32, c: c32) -> c32 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f32_scalar_mul_add(a_re, b_re, self.f32_scalar_mul_add(a_im, b_im, c.re));
        let im = self.f32_scalar_mul_add(a_re, b_im, self.f32_scalar_mul_add(-a_im, b_re, c.im));

        c32 { re, im }
    }

    /// Contains the square of the norm in both the real and imaginary components.
    fn abs2_c32s(self, a: Self::c32s) -> Self::c32s;
    /// Contains the max norm in both the real and imaginary components.
    fn abs_max_c32s(self, a: Self::c32s) -> Self::c32s;
    /// Contains the max norm in both the real and imaginary components.
    fn abs_max_c64s(self, a: Self::c64s) -> Self::c64s;

    fn reduce_sum_c32s(self, a: Self::c32s) -> c32;

    fn splat_f64s(self, value: f64) -> Self::f64s;
    #[inline]
    fn abs_f64s(self, a: Self::f64s) -> Self::f64s {
        self.and_f64s(self.not_f64s(self.splat_f64s(-0.0)), a)
    }
    #[inline]
    fn neg_f64s(self, a: Self::f64s) -> Self::f64s {
        self.xor_f64s(a, self.splat_f64s(-0.0))
    }
    fn add_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn sub_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn mul_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn div_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    #[inline]
    fn mul_add_e_f64s(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s {
        self.add_f64s(self.mul_f64s(a, b), c)
    }
    #[inline]
    fn f64_scalar_mul_add_e(self, a: f64, b: f64, c: f64) -> f64 {
        a * b + c
    }
    fn mul_add_f64s(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s;
    #[inline]
    fn f64_scalar_mul_add(self, a: f64, b: f64, c: f64) -> f64 {
        #[cfg(feature = "std")]
        {
            f64::mul_add(a, b, c)
        }
        #[cfg(not(feature = "std"))]
        {
            libm::fma(a, b, c)
        }
    }
    fn equal_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    fn less_than_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    fn less_than_or_equal_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    #[inline]
    fn greater_than_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s {
        self.less_than_f64s(b, a)
    }
    #[inline]
    fn greater_than_or_equal_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s {
        self.less_than_or_equal_f64s(b, a)
    }
    fn min_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn max_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn reduce_sum_f64s(self, a: Self::f64s) -> f64;
    fn reduce_product_f64s(self, a: Self::f64s) -> f64;
    fn reduce_min_f64s(self, a: Self::f64s) -> f64;
    fn reduce_max_f64s(self, a: Self::f64s) -> f64;

    fn reduce_min_c32s(self, a: Self::c32s) -> c32;
    fn reduce_max_c32s(self, a: Self::c32s) -> c32;
    fn reduce_min_c64s(self, a: Self::c64s) -> c64;
    fn reduce_max_c64s(self, a: Self::c64s) -> c64;

    fn splat_c64s(self, value: c64) -> Self::c64s;
    fn conj_c64s(self, a: Self::c64s) -> Self::c64s;
    fn neg_c64s(self, a: Self::c64s) -> Self::c64s;
    fn swap_re_im_c64s(self, a: Self::c64s) -> Self::c64s;
    fn add_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s;
    fn sub_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s;
    /// Computes `a * b`
    fn mul_e_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        self.mul_c64s(a, b)
    }
    #[inline]
    fn c64_scalar_mul_e(self, a: c64, b: c64) -> c64 {
        a * b
    }
    fn mul_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s;
    #[inline]
    fn c64_scalar_mul(self, a: c64, b: c64) -> c64 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f64_scalar_mul_add(a_re, b_re, -a_im * b_im);
        let im = self.f64_scalar_mul_add(a_re, b_im, a_im * b_re);

        c64 { re, im }
    }
    /// Computes `conj(a) * b`
    #[inline]
    fn conj_mul_e_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        self.conj_mul_c64s(a, b)
    }
    #[inline]
    fn c64_scalar_conj_mul_e(self, a: c64, b: c64) -> c64 {
        a.conj() * b
    }
    fn conj_mul_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s;
    #[inline]
    fn c64_scalar_conj_mul(self, a: c64, b: c64) -> c64 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f64_scalar_mul_add(a_re, b_re, a_im * b_im);
        let im = self.f64_scalar_mul_add(a_re, b_im, -a_im * b_re);

        c64 { re, im }
    }

    /// Computes `a * b + c`
    #[inline]
    fn mul_add_e_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.mul_add_c64s(a, b, c)
    }
    #[inline]
    fn c64_scalar_mul_add_e(self, a: c64, b: c64, c: c64) -> c64 {
        a * b + c
    }
    fn mul_add_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s;
    #[inline]
    fn c64_scalar_mul_add(self, a: c64, b: c64, c: c64) -> c64 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f64_scalar_mul_add(a_re, b_re, self.f64_scalar_mul_add(-a_im, b_im, c.re));
        let im = self.f64_scalar_mul_add(a_re, b_im, self.f64_scalar_mul_add(a_im, b_re, c.im));

        c64 { re, im }
    }

    /// Computes `conj(a) * b + c`
    #[inline]
    fn conj_mul_add_e_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.conj_mul_add_c64s(a, b, c)
    }
    #[inline]
    fn c64_scalar_conj_mul_add_e(self, a: c64, b: c64, c: c64) -> c64 {
        a.conj() * b + c
    }
    fn conj_mul_add_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s;
    #[inline]
    fn c64_scalar_conj_mul_add(self, a: c64, b: c64, c: c64) -> c64 {
        let a_re = a.re;
        let a_im = a.im;
        let b_re = b.re;
        let b_im = b.im;

        let re = self.f64_scalar_mul_add(a_re, b_re, self.f64_scalar_mul_add(a_im, b_im, c.re));
        let im = self.f64_scalar_mul_add(a_re, b_im, self.f64_scalar_mul_add(-a_im, b_re, c.im));

        c64 { re, im }
    }

    /// Contains the square of the norm in both the real and imaginary components.
    fn abs2_c64s(self, a: Self::c64s) -> Self::c64s;
    fn reduce_sum_c64s(self, a: Self::c64s) -> c64;

    #[inline]
    fn transmute_i32s_f32s(self, a: Self::f32s) -> Self::i32s {
        cast(a)
    }
    #[inline]
    fn transmute_u32s_f32s(self, a: Self::f32s) -> Self::u32s {
        cast(a)
    }
    #[inline]
    fn transmute_f32s_i32s(self, a: Self::i32s) -> Self::f32s {
        cast(a)
    }
    #[inline]
    fn transmute_u32s_i32s(self, a: Self::i32s) -> Self::u32s {
        cast(a)
    }
    #[inline]
    fn transmute_f32s_u32s(self, a: Self::u32s) -> Self::f32s {
        cast(a)
    }
    #[inline]
    fn transmute_i32s_u32s(self, a: Self::u32s) -> Self::i32s {
        cast(a)
    }

    #[inline]
    fn transmute_i64s_f64s(self, a: Self::f64s) -> Self::i64s {
        cast(a)
    }
    #[inline]
    fn transmute_u64s_f64s(self, a: Self::f64s) -> Self::u64s {
        cast(a)
    }
    #[inline]
    fn transmute_f64s_i64s(self, a: Self::i64s) -> Self::f64s {
        cast(a)
    }
    #[inline]
    fn transmute_u64s_i64s(self, a: Self::i64s) -> Self::u64s {
        cast(a)
    }
    #[inline]
    fn transmute_f64s_u64s(self, a: Self::u64s) -> Self::f64s {
        cast(a)
    }
    #[inline]
    fn transmute_i64s_u64s(self, a: Self::u64s) -> Self::i64s {
        cast(a)
    }

    #[inline(always)]
    fn rotate_right_i32s(self, a: Self::i32s, amount: usize) -> Self::i32s {
        cast(self.rotate_right_u32s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_right_f32s(self, a: Self::f32s, amount: usize) -> Self::f32s {
        cast(self.rotate_right_u32s(cast(a), amount))
    }
    fn rotate_right_u32s(self, a: Self::u32s, amount: usize) -> Self::u32s;
    fn rotate_right_c32s(self, a: Self::c32s, amount: usize) -> Self::c32s;

    #[inline(always)]
    fn rotate_right_i64s(self, a: Self::i64s, amount: usize) -> Self::i64s {
        cast(self.rotate_right_u64s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_right_f64s(self, a: Self::f64s, amount: usize) -> Self::f64s {
        cast(self.rotate_right_u64s(cast(a), amount))
    }
    fn rotate_right_u64s(self, a: Self::u64s, amount: usize) -> Self::u64s;
    fn rotate_right_c64s(self, a: Self::c64s, amount: usize) -> Self::c64s;

    #[inline(always)]
    fn rotate_left_i32s(self, a: Self::i32s, amount: usize) -> Self::i32s {
        cast(self.rotate_left_u32s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_left_f32s(self, a: Self::f32s, amount: usize) -> Self::f32s {
        cast(self.rotate_left_u32s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_left_u32s(self, a: Self::u32s, amount: usize) -> Self::u32s {
        self.rotate_right_u32s(a, amount.wrapping_neg())
    }
    #[inline(always)]
    fn rotate_left_c32s(self, a: Self::c32s, amount: usize) -> Self::c32s {
        self.rotate_right_c32s(a, amount.wrapping_neg())
    }

    #[inline(always)]
    fn rotate_left_i64s(self, a: Self::i64s, amount: usize) -> Self::i64s {
        cast(self.rotate_left_u64s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_left_f64s(self, a: Self::f64s, amount: usize) -> Self::f64s {
        cast(self.rotate_left_u64s(cast(a), amount))
    }
    #[inline(always)]
    fn rotate_left_u64s(self, a: Self::u64s, amount: usize) -> Self::u64s {
        self.rotate_right_u64s(a, amount.wrapping_neg())
    }
    #[inline(always)]
    fn rotate_left_c64s(self, a: Self::c64s, amount: usize) -> Self::c64s {
        self.rotate_right_c64s(a, amount.wrapping_neg())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar;

impl Default for Scalar {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Scalar {
    #[inline]
    pub fn new() -> Self {
        Self
    }
}

impl Seal for Scalar {}
impl Simd for Scalar {
    const IS_SCALAR: bool = true;
    const REGISTER_COUNT: usize = 16;

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        op.with_simd(self)
    }

    type m32s = bool;
    type f32s = f32;
    type c32s = c32;
    type i32s = i32;
    type u32s = u32;

    type m64s = bool;
    type f64s = f64;
    type c64s = c64;
    type i64s = i64;
    type u64s = u64;

    #[inline]
    fn not_m32s(self, a: Self::m32s) -> Self::m32s {
        !a
    }
    #[inline]
    fn and_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s {
        a & b
    }
    #[inline]
    fn or_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s {
        a | b
    }
    #[inline]
    fn xor_m32s(self, a: Self::m32s, b: Self::m32s) -> Self::m32s {
        a ^ b
    }

    #[inline]
    fn splat_f32s(self, value: f32) -> Self::f32s {
        value
    }
    #[inline]
    fn add_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a + b
    }
    #[inline]
    fn sub_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a - b
    }
    #[inline]
    fn mul_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a * b
    }
    #[inline]
    fn div_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a / b
    }
    #[inline]
    fn equal_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s {
        a == b
    }
    #[inline]
    fn less_than_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s {
        a < b
    }
    #[inline]
    fn less_than_or_equal_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::m32s {
        a <= b
    }

    #[inline]
    fn splat_c32s(self, value: c32) -> Self::c32s {
        value
    }
    #[inline]
    fn add_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        a + b
    }
    #[inline]
    fn sub_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        a - b
    }
    #[inline]
    fn mul_e_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        a * b
    }

    #[inline]
    fn splat_c64s(self, value: c64) -> Self::c64s {
        value
    }
    #[inline]
    fn add_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        a + b
    }
    #[inline]
    fn sub_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        a - b
    }
    #[inline]
    fn mul_e_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        a * b
    }

    #[inline]
    fn not_m64s(self, a: Self::m64s) -> Self::m64s {
        !a
    }
    #[inline]
    fn and_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s {
        a & b
    }
    #[inline]
    fn or_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s {
        a | b
    }
    #[inline]
    fn xor_m64s(self, a: Self::m64s, b: Self::m64s) -> Self::m64s {
        a ^ b
    }

    #[inline]
    fn splat_f64s(self, value: f64) -> Self::f64s {
        value
    }
    #[inline]
    fn add_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a + b
    }
    #[inline]
    fn sub_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a - b
    }
    #[inline]
    fn mul_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a * b
    }
    #[inline]
    fn div_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a / b
    }
    #[inline]
    fn equal_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s {
        a == b
    }
    #[inline]
    fn less_than_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s {
        a < b
    }
    #[inline]
    fn less_than_or_equal_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::m64s {
        a <= b
    }

    #[inline]
    fn not_u32s(self, a: Self::u32s) -> Self::u32s {
        !a
    }
    #[inline]
    fn and_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        a & b
    }
    #[inline]
    fn or_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        a | b
    }
    #[inline]
    fn xor_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        a ^ b
    }

    #[inline]
    fn not_u64s(self, a: Self::u64s) -> Self::u64s {
        !a
    }
    #[inline]
    fn and_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        a & b
    }
    #[inline]
    fn or_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        a | b
    }
    #[inline]
    fn xor_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        a ^ b
    }

    #[inline]
    fn select_u32s_m32s(
        self,
        mask: Self::m32s,
        if_true: Self::u32s,
        if_false: Self::u32s,
    ) -> Self::u32s {
        if mask {
            if_true
        } else {
            if_false
        }
    }
    #[inline]
    fn select_u64s_m64s(
        self,
        mask: Self::m64s,
        if_true: Self::u64s,
        if_false: Self::u64s,
    ) -> Self::u64s {
        if mask {
            if_true
        } else {
            if_false
        }
    }

    #[inline]
    fn min_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a.min(b)
    }
    #[inline]
    fn max_f32s(self, a: Self::f32s, b: Self::f32s) -> Self::f32s {
        a.max(b)
    }

    #[inline]
    fn min_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a.min(b)
    }
    #[inline]
    fn max_f64s(self, a: Self::f64s, b: Self::f64s) -> Self::f64s {
        a.max(b)
    }

    #[inline]
    fn add_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        a.wrapping_add(b)
    }
    #[inline]
    fn sub_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        a.wrapping_sub(b)
    }
    #[inline]
    fn add_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        a.wrapping_add(b)
    }
    #[inline]
    fn sub_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        a.wrapping_sub(b)
    }

    #[inline]
    fn splat_u32s(self, value: u32) -> Self::u32s {
        value
    }
    #[inline]
    fn splat_u64s(self, value: u64) -> Self::u64s {
        value
    }

    #[inline]
    fn reduce_sum_f32s(self, a: Self::f32s) -> f32 {
        a
    }
    #[inline]
    fn reduce_product_f32s(self, a: Self::f32s) -> f32 {
        a
    }
    #[inline]
    fn reduce_min_f32s(self, a: Self::f32s) -> f32 {
        a
    }
    #[inline]
    fn reduce_max_f32s(self, a: Self::f32s) -> f32 {
        a
    }
    #[inline]
    fn reduce_sum_f64s(self, a: Self::f64s) -> f64 {
        a
    }
    #[inline]
    fn reduce_product_f64s(self, a: Self::f64s) -> f64 {
        a
    }
    #[inline]
    fn reduce_min_f64s(self, a: Self::f64s) -> f64 {
        a
    }
    #[inline]
    fn reduce_max_f64s(self, a: Self::f64s) -> f64 {
        a
    }

    #[inline]
    fn mul_add_f32s(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s {
        self.f32_scalar_mul_add(a, b, c)
    }
    #[inline]
    fn mul_add_f64s(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s {
        self.f64_scalar_mul_add(a, b, c)
    }

    #[inline]
    fn abs2_c32s(self, a: Self::c32s) -> Self::c32s {
        let norm2 = a.re * a.re + a.im * a.im;
        c32::new(norm2, norm2)
    }
    #[inline]
    fn abs2_c64s(self, a: Self::c64s) -> Self::c64s {
        let norm2 = a.re * a.re + a.im * a.im;
        c64::new(norm2, norm2)
    }

    #[inline]
    fn partial_load_u32s(self, slice: &[u32]) -> Self::u32s {
        if let Some((head, _)) = slice.split_first() {
            *head
        } else {
            0
        }
    }

    #[inline]
    fn partial_store_u32s(self, slice: &mut [u32], values: Self::u32s) {
        if let Some((head, _)) = slice.split_first_mut() {
            *head = values;
        }
    }

    #[inline]
    fn partial_load_u64s(self, slice: &[u64]) -> Self::u64s {
        if let Some((head, _)) = slice.split_first() {
            *head
        } else {
            0
        }
    }

    #[inline]
    fn partial_store_u64s(self, slice: &mut [u64], values: Self::u64s) {
        if let Some((head, _)) = slice.split_first_mut() {
            *head = values;
        }
    }

    #[inline]
    fn partial_load_c64s(self, slice: &[c64]) -> Self::c64s {
        if let Some((head, _)) = slice.split_first() {
            *head
        } else {
            c64 { re: 0.0, im: 0.0 }
        }
    }

    #[inline]
    fn partial_store_c64s(self, slice: &mut [c64], values: Self::c64s) {
        if let Some((head, _)) = slice.split_first_mut() {
            *head = values;
        }
    }

    #[inline]
    fn conj_mul_e_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        a.conj() * b
    }

    #[inline]
    fn mul_add_e_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        a * b + c
    }

    #[inline]
    fn conj_mul_add_e_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        a.conj() * b + c
    }

    #[inline]
    fn conj_mul_e_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        a.conj() * b
    }

    #[inline]
    fn mul_add_e_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        a * b + c
    }

    #[inline]
    fn conj_mul_add_e_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        a.conj() * b + c
    }

    #[inline]
    fn conj_c32s(self, a: Self::c32s) -> Self::c32s {
        a.conj()
    }

    #[inline]
    fn conj_c64s(self, a: Self::c64s) -> Self::c64s {
        a.conj()
    }

    #[inline]
    fn neg_c32s(self, a: Self::c32s) -> Self::c32s {
        -a
    }

    #[inline]
    fn swap_re_im_c32s(self, a: Self::c32s) -> Self::c32s {
        c32 { re: a.im, im: a.re }
    }

    #[inline]
    fn reduce_sum_c32s(self, a: Self::c32s) -> c32 {
        a
    }

    #[inline]
    fn neg_c64s(self, a: Self::c64s) -> Self::c64s {
        -a
    }

    fn swap_re_im_c64s(self, a: Self::c64s) -> Self::c64s {
        c64 { re: a.im, im: a.re }
    }

    #[inline]
    fn reduce_sum_c64s(self, a: Self::c64s) -> c64 {
        a
    }

    #[inline]
    fn wrapping_dyn_shl_u32s(self, a: Self::u32s, amount: Self::u32s) -> Self::u32s {
        a.wrapping_shl(amount)
    }
    #[inline]
    fn wrapping_dyn_shr_u32s(self, a: Self::u32s, amount: Self::u32s) -> Self::u32s {
        a.wrapping_shr(amount)
    }

    #[inline]
    fn widening_mul_u32s(self, a: Self::u32s, b: Self::u32s) -> (Self::u32s, Self::u32s) {
        let c = a as u64 * b as u64;
        let lo = c as u32;
        let hi = (c >> 32) as u32;
        (lo, hi)
    }

    #[inline]
    fn less_than_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s {
        a < b
    }

    #[inline]
    fn greater_than_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s {
        a > b
    }

    #[inline]
    fn less_than_or_equal_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s {
        a <= b
    }

    #[inline]
    fn greater_than_or_equal_u32s(self, a: Self::u32s, b: Self::u32s) -> Self::m32s {
        a >= b
    }

    #[inline]
    fn mul_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        self.c32_scalar_mul(a, b)
    }

    #[inline]
    fn conj_mul_c32s(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        self.c32_scalar_conj_mul(a, b)
    }

    #[inline]
    fn mul_add_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.c32_scalar_mul_add(a, b, c)
    }

    #[inline]
    fn conj_mul_add_c32s(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.c32_scalar_conj_mul_add(a, b, c)
    }

    #[inline]
    fn mul_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        self.c64_scalar_mul(a, b)
    }

    #[inline]
    fn conj_mul_c64s(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        self.c64_scalar_conj_mul(a, b)
    }

    #[inline]
    fn mul_add_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.c64_scalar_mul_add(a, b, c)
    }

    #[inline]
    fn conj_mul_add_c64s(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.c64_scalar_conj_mul_add(a, b, c)
    }

    #[inline(always)]
    unsafe fn mask_load_ptr_u32s(self, mask: MemMask<Self::m32s>, ptr: *const u32) -> Self::u32s {
        if mask.mask {
            *ptr
        } else {
            0
        }
    }

    #[inline(always)]
    unsafe fn mask_load_ptr_c32s(self, mask: MemMask<Self::m32s>, ptr: *const c32) -> Self::c32s {
        if mask.mask {
            *ptr
        } else {
            core::mem::zeroed()
        }
    }

    #[inline(always)]
    unsafe fn mask_store_ptr_u32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut u32,
        values: Self::u32s,
    ) {
        if mask.mask {
            *ptr = values
        }
    }

    #[inline(always)]
    unsafe fn mask_store_ptr_c32s(
        self,
        mask: MemMask<Self::m32s>,
        ptr: *mut c32,
        values: Self::c32s,
    ) {
        if mask.mask {
            *ptr = values
        }
    }

    #[inline(always)]
    unsafe fn mask_load_ptr_u64s(self, mask: MemMask<Self::m64s>, ptr: *const u64) -> Self::u64s {
        if mask.mask {
            *ptr
        } else {
            0
        }
    }

    #[inline(always)]
    unsafe fn mask_load_ptr_c64s(self, mask: MemMask<Self::m64s>, ptr: *const c64) -> Self::c64s {
        if mask.mask {
            *ptr
        } else {
            core::mem::zeroed()
        }
    }

    #[inline(always)]
    unsafe fn mask_store_ptr_u64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut u64,
        values: Self::u64s,
    ) {
        if mask.mask {
            *ptr = values
        }
    }

    #[inline(always)]
    unsafe fn mask_store_ptr_c64s(
        self,
        mask: MemMask<Self::m64s>,
        ptr: *mut c64,
        values: Self::c64s,
    ) {
        if mask.mask {
            *ptr = values
        }
    }

    #[inline(always)]
    fn less_than_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s {
        a < b
    }

    #[inline(always)]
    fn greater_than_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s {
        a > b
    }

    #[inline(always)]
    fn less_than_or_equal_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s {
        a <= b
    }

    #[inline(always)]
    fn greater_than_or_equal_u64s(self, a: Self::u64s, b: Self::u64s) -> Self::m64s {
        a >= b
    }

    #[inline(always)]
    fn rotate_right_u32s(self, a: Self::u32s, _amount: usize) -> Self::u32s {
        a
    }

    #[inline(always)]
    fn rotate_right_c32s(self, a: Self::c32s, _amount: usize) -> Self::c32s {
        a
    }

    #[inline(always)]
    fn rotate_right_u64s(self, a: Self::u64s, _amount: usize) -> Self::u64s {
        a
    }

    #[inline(always)]
    fn rotate_right_c64s(self, a: Self::c64s, _amount: usize) -> Self::c64s {
        a
    }

    #[inline(always)]
    fn abs_max_c32s(self, a: Self::c32s) -> Self::c32s {
        let re = if a.re > a.im { a.re } else { a.im };
        let im = re;
        Complex { re, im }
    }

    #[inline(always)]
    fn abs_max_c64s(self, a: Self::c64s) -> Self::c64s {
        let re = if a.re > a.im { a.re } else { a.im };
        let im = re;
        Complex { re, im }
    }

    #[inline(always)]
    fn first_true_m32s(self, mask: Self::m32s) -> usize {
        if mask {
            0
        } else {
            1
        }
    }

    #[inline(always)]
    fn first_true_m64s(self, mask: Self::m64s) -> usize {
        if mask {
            0
        } else {
            1
        }
    }

    #[inline(always)]
    fn reduce_min_c32s(self, a: Self::c32s) -> c32 {
        a
    }
    #[inline(always)]
    fn reduce_max_c32s(self, a: Self::c32s) -> c32 {
        a
    }
    #[inline(always)]
    fn reduce_min_c64s(self, a: Self::c64s) -> c64 {
        a
    }
    #[inline(always)]
    fn reduce_max_c64s(self, a: Self::c64s) -> c64 {
        a
    }
}

#[inline(always)]
unsafe fn split_slice<T, U>(slice: &[T]) -> (&[U], &[T]) {
    assert_eq!(core::mem::size_of::<U>() % core::mem::size_of::<T>(), 0);
    assert_eq!(core::mem::align_of::<U>(), core::mem::align_of::<T>());

    let chunk_size = core::mem::size_of::<U>() / core::mem::size_of::<T>();

    let len = slice.len();
    let data = slice.as_ptr();

    let div = len / chunk_size;
    let rem = len % chunk_size;
    (
        from_raw_parts(data as *const U, div),
        from_raw_parts(data.add(len - rem), rem),
    )
}

#[inline(always)]
unsafe fn split_mut_slice<T, U>(slice: &mut [T]) -> (&mut [U], &mut [T]) {
    assert_eq!(core::mem::size_of::<U>() % core::mem::size_of::<T>(), 0);
    assert_eq!(core::mem::align_of::<U>(), core::mem::align_of::<T>());

    let chunk_size = core::mem::size_of::<U>() / core::mem::size_of::<T>();

    let len = slice.len();
    let data = slice.as_mut_ptr();

    let div = len / chunk_size;
    let rem = len % chunk_size;
    (
        from_raw_parts_mut(data as *mut U, div),
        from_raw_parts_mut(data.add(len - rem), rem),
    )
}

#[inline(always)]
unsafe fn rsplit_slice<T, U>(slice: &[T]) -> (&[T], &[U]) {
    assert_eq!(core::mem::size_of::<U>() % core::mem::size_of::<T>(), 0);
    assert_eq!(core::mem::align_of::<U>(), core::mem::align_of::<T>());

    let chunk_size = core::mem::size_of::<U>() / core::mem::size_of::<T>();

    let len = slice.len();
    let data = slice.as_ptr();

    let div = len / chunk_size;
    let rem = len % chunk_size;
    (
        from_raw_parts(data, rem),
        from_raw_parts(data.add(rem) as *const U, div),
    )
}

#[inline(always)]
unsafe fn rsplit_mut_slice<T, U>(slice: &mut [T]) -> (&mut [T], &mut [U]) {
    assert_eq!(core::mem::size_of::<U>() % core::mem::size_of::<T>(), 0);
    assert_eq!(core::mem::align_of::<U>(), core::mem::align_of::<T>());

    let chunk_size = core::mem::size_of::<U>() / core::mem::size_of::<T>();

    let len = slice.len();
    let data = slice.as_mut_ptr();

    let div = len / chunk_size;
    let rem = len % chunk_size;
    (
        from_raw_parts_mut(data, rem),
        from_raw_parts_mut(data.add(rem) as *mut U, div),
    )
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
#[repr(u8)]
enum ArchInner {
    Scalar = 0,
    // improves codegen for some reason
    #[allow(dead_code)]
    Dummy = u8::MAX - 1,
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")))]
impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        Self::Scalar
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            ArchInner::Scalar => crate::Scalar::new().vectorize(op),
            ArchInner::Dummy => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[cfg(target_arch = "aarch64")]
use aarch64::ArchInner;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use x86::ArchInner;

impl Arch {
    #[inline(always)]
    fn __static_available() -> &'static ::core::sync::atomic::AtomicU8 {
        static AVAILABLE: ::core::sync::atomic::AtomicU8 =
            ::core::sync::atomic::AtomicU8::new(u8::MAX);
        &AVAILABLE
    }

    #[inline(never)]
    fn __detect_is_available() -> u8 {
        let out = unsafe {
            core::mem::transmute(Self {
                inner: ArchInner::new(),
            })
        };
        Self::__static_available().store(out, ::core::sync::atomic::Ordering::Relaxed);
        out
    }

    #[inline(always)]
    pub fn new() -> Self {
        let mut available =
            Self::__static_available().load(::core::sync::atomic::Ordering::Relaxed);
        if available == u8::MAX {
            available = Self::__detect_is_available();
        }

        unsafe { core::mem::transmute(available) }
    }
    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        self.inner.dispatch(op)
    }
}

impl Default for Arch {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ScalarArch {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Arch {
    inner: ArchInner,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use x86::ScalarArchInner;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
enum ScalarArchInner {
    Scalar = 0,
    // improves codegen for some reason
    #[allow(dead_code)]
    Dummy = u8::MAX - 1,
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl ScalarArchInner {
    #[inline]
    pub fn new() -> Self {
        Self::Scalar
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            ScalarArchInner::Scalar => crate::Scalar::new().vectorize(op),
            ScalarArchInner::Dummy => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ScalarArch {
    inner: ScalarArchInner,
}

impl ScalarArch {
    #[inline(always)]
    fn __static_available() -> &'static ::core::sync::atomic::AtomicU8 {
        static AVAILABLE: ::core::sync::atomic::AtomicU8 =
            ::core::sync::atomic::AtomicU8::new(u8::MAX);
        &AVAILABLE
    }

    #[inline(never)]
    fn __detect_is_available() -> u8 {
        let out = unsafe {
            core::mem::transmute(Self {
                inner: ScalarArchInner::new(),
            })
        };
        Self::__static_available().store(out, ::core::sync::atomic::Ordering::Relaxed);
        out
    }

    #[inline(always)]
    pub fn new() -> Self {
        let mut available =
            Self::__static_available().load(::core::sync::atomic::Ordering::Relaxed);
        if available == u8::MAX {
            available = Self::__detect_is_available();
        }

        unsafe { core::mem::transmute(available) }
    }
    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        self.inner.dispatch(op)
    }
}

#[doc(hidden)]
pub struct CheckSameSize<T, U>(PhantomData<(T, U)>);
impl<T, U> CheckSameSize<T, U> {
    pub const VALID: () = {
        assert!(core::mem::size_of::<T>() == core::mem::size_of::<U>());
    };
}

#[doc(hidden)]
pub struct CheckSizeLessThanOrEqual<T, U>(PhantomData<(T, U)>);
impl<T, U> CheckSizeLessThanOrEqual<T, U> {
    pub const VALID: () = {
        assert!(core::mem::size_of::<T>() <= core::mem::size_of::<U>());
    };
}

#[macro_export]
macro_rules! static_assert_same_size {
    ($t: ty, $u: ty) => {
        let _ = $crate::CheckSameSize::<$t, $u>::VALID;
    };
}
#[macro_export]
macro_rules! static_assert_size_less_than_or_equal {
    ($t: ty, $u: ty) => {
        let _ = $crate::CheckSizeLessThanOrEqual::<$t, $u>::VALID;
    };
}

/// Safe transmute function.
///
/// This function asserts at compile time that the two types have the same size.
#[inline(always)]
pub fn cast<T: NoUninit, U: AnyBitPattern>(value: T) -> U {
    static_assert_same_size!(T, U);
    let value = core::mem::ManuallyDrop::new(value);
    let ptr = &value as *const core::mem::ManuallyDrop<T> as *const U;
    unsafe { ptr.read_unaligned() }
}

/// Safe lossy transmute function, where the destination type may be smaller than the source type.
///
/// This property is checked at compile time.
#[inline(always)]
pub fn cast_lossy<T: NoUninit, U: AnyBitPattern>(value: T) -> U {
    static_assert_size_less_than_or_equal!(U, T);
    let value = core::mem::ManuallyDrop::new(value);
    let ptr = &value as *const core::mem::ManuallyDrop<T> as *const U;
    unsafe { ptr.read_unaligned() }
}

/// Splits a slice into chunks of equal size (known at compile time).
///
/// Returns the chunks and the remaining section of the input slice.
#[inline(always)]
pub fn as_arrays<const N: usize, T>(slice: &[T]) -> (&[[T; N]], &[T]) {
    let n = slice.len();
    let mid_div_n = n / N;
    let mid = mid_div_n * N;
    let ptr = slice.as_ptr();
    unsafe {
        (
            from_raw_parts(ptr as *const [T; N], mid_div_n),
            from_raw_parts(ptr.add(mid), n - mid),
        )
    }
}

/// Splits a slice into chunks of equal size (known at compile time).
///
/// Returns the chunks and the remaining section of the input slice.
#[inline(always)]
pub fn as_arrays_mut<const N: usize, T>(slice: &mut [T]) -> (&mut [[T; N]], &mut [T]) {
    let n = slice.len();
    let mid_div_n = n / N;
    let mid = mid_div_n * N;
    let ptr = slice.as_mut_ptr();
    unsafe {
        (
            from_raw_parts_mut(ptr as *mut [T; N], mid_div_n),
            from_raw_parts_mut(ptr.add(mid), n - mid),
        )
    }
}

/// Platform dependent intrinsics.
#[doc(hidden)]
pub mod core_arch;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(docsrs, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
/// Low level x86 API.
pub mod x86;

#[cfg(target_arch = "aarch64")]
#[cfg_attr(docsrs, doc(cfg(target_arch = "aarch64")))]
/// Low level aarch64 API.
pub mod aarch64;

/// Mask type with 8 bits. Its bit either all ones or all zeros.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m8(u8);
/// Mask type with 16 bits. Its bit either all ones or all zeros.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m16(u16);
/// Mask type with 32 bits. Its bit either all ones or all zeros.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m32(u32);
/// Mask type with 64 bits. Its bit either all ones or all zeros.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m64(u64);

/// Bitmask type for 8 elements, used for mask operations on AVX512.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct b8(pub u8);
/// Bitmask type for 16 elements, used for mask operations on AVX512.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct b16(pub u16);
/// Bitmask type for 32 elements, used for mask operations on AVX512.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct b32(pub u32);
/// Bitmask type for 64 elements, used for mask operations on AVX512.
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct b64(pub u64);

impl core::ops::Not for b8 {
    type Output = b8;
    #[inline(always)]
    fn not(self) -> Self::Output {
        b8(!self.0)
    }
}
impl core::ops::BitAnd for b8 {
    type Output = b8;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        b8(self.0 & rhs.0)
    }
}
impl core::ops::BitOr for b8 {
    type Output = b8;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        b8(self.0 | rhs.0)
    }
}
impl core::ops::BitXor for b8 {
    type Output = b8;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        b8(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for b16 {
    type Output = b16;
    #[inline(always)]
    fn not(self) -> Self::Output {
        b16(!self.0)
    }
}
impl core::ops::BitAnd for b16 {
    type Output = b16;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        b16(self.0 & rhs.0)
    }
}
impl core::ops::BitOr for b16 {
    type Output = b16;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        b16(self.0 | rhs.0)
    }
}
impl core::ops::BitXor for b16 {
    type Output = b16;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        b16(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for b32 {
    type Output = b32;
    #[inline(always)]
    fn not(self) -> Self::Output {
        b32(!self.0)
    }
}
impl core::ops::BitAnd for b32 {
    type Output = b32;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        b32(self.0 & rhs.0)
    }
}
impl core::ops::BitOr for b32 {
    type Output = b32;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        b32(self.0 | rhs.0)
    }
}
impl core::ops::BitXor for b32 {
    type Output = b32;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        b32(self.0 ^ rhs.0)
    }
}

impl core::ops::Not for b64 {
    type Output = b64;
    #[inline(always)]
    fn not(self) -> Self::Output {
        b64(!self.0)
    }
}
impl core::ops::BitAnd for b64 {
    type Output = b64;
    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        b64(self.0 & rhs.0)
    }
}
impl core::ops::BitOr for b64 {
    type Output = b64;
    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        b64(self.0 | rhs.0)
    }
}
impl core::ops::BitXor for b64 {
    type Output = b64;
    #[inline(always)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        b64(self.0 ^ rhs.0)
    }
}

impl Debug for b8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug)]
        struct b8(bool, bool, bool, bool, bool, bool, bool, bool);
        b8(
            ((self.0 >> 0) & 1) == 1,
            ((self.0 >> 1) & 1) == 1,
            ((self.0 >> 2) & 1) == 1,
            ((self.0 >> 3) & 1) == 1,
            ((self.0 >> 4) & 1) == 1,
            ((self.0 >> 5) & 1) == 1,
            ((self.0 >> 6) & 1) == 1,
            ((self.0 >> 7) & 1) == 1,
        )
        .fmt(f)
    }
}
impl Debug for b16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug)]
        struct b16(
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        );
        b16(
            ((self.0 >> 00) & 1) == 1,
            ((self.0 >> 01) & 1) == 1,
            ((self.0 >> 02) & 1) == 1,
            ((self.0 >> 03) & 1) == 1,
            ((self.0 >> 04) & 1) == 1,
            ((self.0 >> 05) & 1) == 1,
            ((self.0 >> 06) & 1) == 1,
            ((self.0 >> 07) & 1) == 1,
            ((self.0 >> 08) & 1) == 1,
            ((self.0 >> 09) & 1) == 1,
            ((self.0 >> 10) & 1) == 1,
            ((self.0 >> 11) & 1) == 1,
            ((self.0 >> 12) & 1) == 1,
            ((self.0 >> 13) & 1) == 1,
            ((self.0 >> 14) & 1) == 1,
            ((self.0 >> 15) & 1) == 1,
        )
        .fmt(f)
    }
}
impl Debug for b32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug)]
        struct b32(
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        );
        b32(
            ((self.0 >> 00) & 1) == 1,
            ((self.0 >> 01) & 1) == 1,
            ((self.0 >> 02) & 1) == 1,
            ((self.0 >> 03) & 1) == 1,
            ((self.0 >> 04) & 1) == 1,
            ((self.0 >> 05) & 1) == 1,
            ((self.0 >> 06) & 1) == 1,
            ((self.0 >> 07) & 1) == 1,
            ((self.0 >> 08) & 1) == 1,
            ((self.0 >> 09) & 1) == 1,
            ((self.0 >> 10) & 1) == 1,
            ((self.0 >> 11) & 1) == 1,
            ((self.0 >> 12) & 1) == 1,
            ((self.0 >> 13) & 1) == 1,
            ((self.0 >> 14) & 1) == 1,
            ((self.0 >> 15) & 1) == 1,
            ((self.0 >> 16) & 1) == 1,
            ((self.0 >> 17) & 1) == 1,
            ((self.0 >> 18) & 1) == 1,
            ((self.0 >> 19) & 1) == 1,
            ((self.0 >> 20) & 1) == 1,
            ((self.0 >> 21) & 1) == 1,
            ((self.0 >> 22) & 1) == 1,
            ((self.0 >> 23) & 1) == 1,
            ((self.0 >> 24) & 1) == 1,
            ((self.0 >> 25) & 1) == 1,
            ((self.0 >> 26) & 1) == 1,
            ((self.0 >> 27) & 1) == 1,
            ((self.0 >> 28) & 1) == 1,
            ((self.0 >> 29) & 1) == 1,
            ((self.0 >> 30) & 1) == 1,
            ((self.0 >> 31) & 1) == 1,
        )
        .fmt(f)
    }
}
impl Debug for b64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(dead_code)]
        #[derive(Copy, Clone, Debug)]
        struct b64(
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
            bool,
        );
        b64(
            ((self.0 >> 00) & 1) == 1,
            ((self.0 >> 01) & 1) == 1,
            ((self.0 >> 02) & 1) == 1,
            ((self.0 >> 03) & 1) == 1,
            ((self.0 >> 04) & 1) == 1,
            ((self.0 >> 05) & 1) == 1,
            ((self.0 >> 06) & 1) == 1,
            ((self.0 >> 07) & 1) == 1,
            ((self.0 >> 08) & 1) == 1,
            ((self.0 >> 09) & 1) == 1,
            ((self.0 >> 10) & 1) == 1,
            ((self.0 >> 11) & 1) == 1,
            ((self.0 >> 12) & 1) == 1,
            ((self.0 >> 13) & 1) == 1,
            ((self.0 >> 14) & 1) == 1,
            ((self.0 >> 15) & 1) == 1,
            ((self.0 >> 16) & 1) == 1,
            ((self.0 >> 17) & 1) == 1,
            ((self.0 >> 18) & 1) == 1,
            ((self.0 >> 19) & 1) == 1,
            ((self.0 >> 20) & 1) == 1,
            ((self.0 >> 21) & 1) == 1,
            ((self.0 >> 22) & 1) == 1,
            ((self.0 >> 23) & 1) == 1,
            ((self.0 >> 24) & 1) == 1,
            ((self.0 >> 25) & 1) == 1,
            ((self.0 >> 26) & 1) == 1,
            ((self.0 >> 27) & 1) == 1,
            ((self.0 >> 28) & 1) == 1,
            ((self.0 >> 29) & 1) == 1,
            ((self.0 >> 30) & 1) == 1,
            ((self.0 >> 31) & 1) == 1,
            ((self.0 >> 32) & 1) == 1,
            ((self.0 >> 33) & 1) == 1,
            ((self.0 >> 34) & 1) == 1,
            ((self.0 >> 35) & 1) == 1,
            ((self.0 >> 36) & 1) == 1,
            ((self.0 >> 37) & 1) == 1,
            ((self.0 >> 38) & 1) == 1,
            ((self.0 >> 39) & 1) == 1,
            ((self.0 >> 40) & 1) == 1,
            ((self.0 >> 41) & 1) == 1,
            ((self.0 >> 42) & 1) == 1,
            ((self.0 >> 43) & 1) == 1,
            ((self.0 >> 44) & 1) == 1,
            ((self.0 >> 45) & 1) == 1,
            ((self.0 >> 46) & 1) == 1,
            ((self.0 >> 47) & 1) == 1,
            ((self.0 >> 48) & 1) == 1,
            ((self.0 >> 49) & 1) == 1,
            ((self.0 >> 50) & 1) == 1,
            ((self.0 >> 51) & 1) == 1,
            ((self.0 >> 52) & 1) == 1,
            ((self.0 >> 53) & 1) == 1,
            ((self.0 >> 54) & 1) == 1,
            ((self.0 >> 55) & 1) == 1,
            ((self.0 >> 56) & 1) == 1,
            ((self.0 >> 57) & 1) == 1,
            ((self.0 >> 58) & 1) == 1,
            ((self.0 >> 59) & 1) == 1,
            ((self.0 >> 60) & 1) == 1,
            ((self.0 >> 61) & 1) == 1,
            ((self.0 >> 62) & 1) == 1,
            ((self.0 >> 63) & 1) == 1,
        )
        .fmt(f)
    }
}

impl Debug for m8 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m16 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m32 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m64 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}

impl m8 {
    /// Returns a mask with all bits set one, if `flag` is true, otherwise returns a mask with all
    /// bits set to zero.
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u8::MAX } else { 0 })
    }

    /// Returns `false` if the mask bits are all zero, otherwise returns `true`.
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
}
impl m16 {
    /// Returns a mask with all bits set one, if `flag` is true, otherwise returns a mask with all
    /// bits set to zero.
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u16::MAX } else { 0 })
    }

    /// Returns `false` if the mask bits are all zero, otherwise returns `true`.
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
}
impl m32 {
    /// Returns a mask with all bits set one, if `flag` is true, otherwise returns a mask with all
    /// bits set to zero.
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u32::MAX } else { 0 })
    }

    /// Returns `false` if the mask bits are all zero, otherwise returns `true`.
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
}
impl m64 {
    /// Returns a mask with all bits set one, if `flag` is true, otherwise returns a mask with all
    /// bits set to zero.
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u64::MAX } else { 0 })
    }

    /// Returns `false` if the mask bits are all zero, otherwise returns `true`.
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
}

/// A 128-bit SIMD vector with 16 elements of type [`i8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i8x16(
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
);
/// A 256-bit SIMD vector with 32 elements of type [`i8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i8x32(
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
);
/// A 512-bit SIMD vector with 64 elements of type [`i8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i8x64(
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
    pub i8,
);

/// A 128-bit SIMD vector with 16 elements of type [`u8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u8x16(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);
/// A 256-bit SIMD vector with 32 elements of type [`u8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u8x32(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);
/// A 512-bit SIMD vector with 64 elements of type [`u8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u8x64(
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
    pub u8,
);

/// A 128-bit SIMD vector with 16 elements of type [`m8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m8x16(
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
);
/// A 256-bit SIMD vector with 32 elements of type [`m8`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m8x32(
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
    pub m8,
);

/// A 128-bit SIMD vector with 8 elements of type [`i16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i16x8(
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
);
/// A 256-bit SIMD vector with 16 elements of type [`i16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i16x16(
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
);
/// A 512-bit SIMD vector with 32 elements of type [`i16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i16x32(
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
    pub i16,
);

/// A 128-bit SIMD vector with 8 elements of type [`u16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u16x8(
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
);
/// A 256-bit SIMD vector with 16 elements of type [`u16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u16x16(
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
);
/// A 512-bit SIMD vector with 32 elements of type [`u16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u16x32(
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
    pub u16,
);

/// A 128-bit SIMD vector with 8 elements of type [`m16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m16x8(
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
);
/// A 256-bit SIMD vector with 16 elements of type [`m16`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m16x16(
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
    pub m16,
);

/// A 128-bit SIMD vector with 4 elements of type [`f32`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);
/// A 256-bit SIMD vector with 8 elements of type [`f32`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f32x8(
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
);
/// A 512-bit SIMD vector with 16 elements of type [`f32`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f32x16(
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
);

/// A 128-bit SIMD vector with 4 elements of type [`i32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);
/// A 256-bit SIMD vector with 8 elements of type [`i32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i32x8(
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
);
/// A 512-bit SIMD vector with 16 elements of type [`i32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i32x16(
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
    pub i32,
);

/// A 128-bit SIMD vector with 4 elements of type [`u32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);
/// A 256-bit SIMD vector with 8 elements of type [`u32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u32x8(
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
);
/// A 512-bit SIMD vector with 16 elements of type [`u32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u32x16(
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
    pub u32,
);

/// A 128-bit SIMD vector with 4 elements of type [`m32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m32x4(pub m32, pub m32, pub m32, pub m32);
/// A 256-bit SIMD vector with 8 elements of type [`m32`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m32x8(
    pub m32,
    pub m32,
    pub m32,
    pub m32,
    pub m32,
    pub m32,
    pub m32,
    pub m32,
);

/// A 128-bit SIMD vector with 2 elements of type [`f64`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f64x2(pub f64, pub f64);
/// A 256-bit SIMD vector with 4 elements of type [`f64`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f64x4(pub f64, pub f64, pub f64, pub f64);
/// A 512-bit SIMD vector with 8 elements of type [`f64`].
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct f64x8(
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
);

/// A 128-bit SIMD vector with 2 elements of type [`i64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i64x2(pub i64, pub i64);
/// A 256-bit SIMD vector with 4 elements of type [`i64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i64x4(pub i64, pub i64, pub i64, pub i64);
/// A 512-bit SIMD vector with 8 elements of type [`i64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct i64x8(
    pub i64,
    pub i64,
    pub i64,
    pub i64,
    pub i64,
    pub i64,
    pub i64,
    pub i64,
);

/// A 128-bit SIMD vector with 2 elements of type [`u64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u64x2(pub u64, pub u64);
/// A 256-bit SIMD vector with 4 elements of type [`u64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u64x4(pub u64, pub u64, pub u64, pub u64);
/// A 512-bit SIMD vector with 8 elements of type [`u64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct u64x8(
    pub u64,
    pub u64,
    pub u64,
    pub u64,
    pub u64,
    pub u64,
    pub u64,
    pub u64,
);

/// A 128-bit SIMD vector with 2 elements of type [`m64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m64x2(pub m64, pub m64);
/// A 256-bit SIMD vector with 4 elements of type [`m64`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct m64x4(pub m64, pub m64, pub m64, pub m64);

unsafe impl Zeroable for m8 {}
unsafe impl Zeroable for m16 {}
unsafe impl Zeroable for m32 {}
unsafe impl Zeroable for m64 {}
unsafe impl NoUninit for m8 {}
unsafe impl NoUninit for m16 {}
unsafe impl NoUninit for m32 {}
unsafe impl NoUninit for m64 {}

unsafe impl Zeroable for b8 {}
unsafe impl Pod for b8 {}
unsafe impl Zeroable for b16 {}
unsafe impl Pod for b16 {}
unsafe impl Zeroable for b32 {}
unsafe impl Pod for b32 {}
unsafe impl Zeroable for b64 {}
unsafe impl Pod for b64 {}

unsafe impl Zeroable for i8x16 {}
unsafe impl Zeroable for i8x32 {}
unsafe impl Zeroable for i8x64 {}
unsafe impl Pod for i8x16 {}
unsafe impl Pod for i8x32 {}
unsafe impl Pod for i8x64 {}
unsafe impl Zeroable for u8x16 {}
unsafe impl Zeroable for u8x32 {}
unsafe impl Zeroable for u8x64 {}
unsafe impl Pod for u8x16 {}
unsafe impl Pod for u8x32 {}
unsafe impl Pod for u8x64 {}
unsafe impl Zeroable for m8x16 {}
unsafe impl Zeroable for m8x32 {}
unsafe impl NoUninit for m8x16 {}
unsafe impl NoUninit for m8x32 {}

unsafe impl Zeroable for i16x8 {}
unsafe impl Zeroable for i16x16 {}
unsafe impl Zeroable for i16x32 {}
unsafe impl Pod for i16x8 {}
unsafe impl Pod for i16x16 {}
unsafe impl Pod for i16x32 {}
unsafe impl Zeroable for u16x8 {}
unsafe impl Zeroable for u16x16 {}
unsafe impl Zeroable for u16x32 {}
unsafe impl Pod for u16x8 {}
unsafe impl Pod for u16x16 {}
unsafe impl Pod for u16x32 {}
unsafe impl Zeroable for m16x8 {}
unsafe impl Zeroable for m16x16 {}
unsafe impl NoUninit for m16x8 {}
unsafe impl NoUninit for m16x16 {}

unsafe impl Zeroable for f32x4 {}
unsafe impl Zeroable for f32x8 {}
unsafe impl Zeroable for f32x16 {}
unsafe impl Pod for f32x4 {}
unsafe impl Pod for f32x8 {}
unsafe impl Pod for f32x16 {}
unsafe impl Zeroable for i32x4 {}
unsafe impl Zeroable for i32x8 {}
unsafe impl Zeroable for i32x16 {}
unsafe impl Pod for i32x4 {}
unsafe impl Pod for i32x8 {}
unsafe impl Pod for i32x16 {}
unsafe impl Zeroable for u32x4 {}
unsafe impl Zeroable for u32x8 {}
unsafe impl Zeroable for u32x16 {}
unsafe impl Pod for u32x4 {}
unsafe impl Pod for u32x8 {}
unsafe impl Pod for u32x16 {}
unsafe impl Zeroable for m32x4 {}
unsafe impl Zeroable for m32x8 {}
unsafe impl NoUninit for m32x4 {}
unsafe impl NoUninit for m32x8 {}

unsafe impl Zeroable for f64x2 {}
unsafe impl Zeroable for f64x4 {}
unsafe impl Zeroable for f64x8 {}
unsafe impl Pod for f64x2 {}
unsafe impl Pod for f64x4 {}
unsafe impl Pod for f64x8 {}
unsafe impl Zeroable for i64x2 {}
unsafe impl Zeroable for i64x4 {}
unsafe impl Zeroable for i64x8 {}
unsafe impl Pod for i64x2 {}
unsafe impl Pod for i64x4 {}
unsafe impl Pod for i64x8 {}
unsafe impl Zeroable for u64x2 {}
unsafe impl Zeroable for u64x4 {}
unsafe impl Zeroable for u64x8 {}
unsafe impl Pod for u64x2 {}
unsafe impl Pod for u64x4 {}
unsafe impl Pod for u64x8 {}
unsafe impl Zeroable for m64x2 {}
unsafe impl Zeroable for m64x4 {}
unsafe impl NoUninit for m64x2 {}
unsafe impl NoUninit for m64x4 {}

pub trait Iota32: Sized {
    const IOTA: [core::mem::MaybeUninit<Self>; 32];
}
pub trait Iota64: Sized {
    const IOTA: [core::mem::MaybeUninit<Self>; 32];
}

impl<T> Iota32 for T {
    const IOTA: [core::mem::MaybeUninit<Self>; 32] = {
        let mut iota = [const { core::mem::MaybeUninit::uninit() }; 32];
        let mut i = 0;
        while i < 32 {
            let v = (&mut iota[i]) as *mut _ as *mut u32;

            let mut j = 0;
            while j < size_of::<T>() / size_of::<u32>() {
                unsafe { *v.add(j) = i as u32 };
                j += 1;
            }

            i += 1;
        }
        iota
    };
}
impl<T> Iota64 for T {
    const IOTA: [core::mem::MaybeUninit<Self>; 32] = {
        let mut iota = [const { core::mem::MaybeUninit::uninit() }; 32];
        let mut i = 0;
        while i < 32 {
            let v = (&mut iota[i]) as *mut _ as *mut u64;

            let mut j = 0;
            while j < size_of::<T>() / size_of::<u64>() {
                unsafe { *v.add(j) = i as u64 };
                j += 1;
            }

            i += 1;
        }
        iota
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interleave() {
        #[cfg(target_arch = "x86_64")]
        if let Some(simd) = x86::V3::try_new() {
            {
                let src = [f64x4(0.0, 0.1, 1.0, 1.1), f64x4(2.0, 2.1, 3.0, 3.1)];
                let dst = unsafe { deinterleave_fallback::<f64, f64x4, [f64x4; 2]>(src) };
                assert_eq!(dst[1], simd.add_f64x4(dst[0], simd.splat_f64x4(0.1)));
                assert_eq!(src, unsafe {
                    interleave_fallback::<f64, f64x4, [f64x4; 2]>(dst)
                });
            }
            {
                let src = [
                    f64x4(0.0, 0.1, 0.2, 0.3),
                    f64x4(1.0, 1.1, 1.2, 1.3),
                    f64x4(2.0, 2.1, 2.2, 2.3),
                    f64x4(3.0, 3.1, 3.2, 3.3),
                ];
                let dst = unsafe { deinterleave_fallback::<f64, f64x4, [f64x4; 4]>(src) };
                assert_eq!(dst[1], simd.add_f64x4(dst[0], simd.splat_f64x4(0.1)));
                assert_eq!(dst[2], simd.add_f64x4(dst[0], simd.splat_f64x4(0.2)));
                assert_eq!(dst[3], simd.add_f64x4(dst[0], simd.splat_f64x4(0.3)));
                assert_eq!(src, unsafe {
                    interleave_fallback::<f64, f64x4, [f64x4; 4]>(dst)
                });
            }
        }
    }
}
