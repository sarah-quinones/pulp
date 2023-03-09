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
//!         let (head, tail) = S::f64s_as_mut_simd(v);
//!
//!         let three = simd.f64s_splat(3.0);
//!         for x in head {
//!             *x = simd.f64s_mul(three, *x);
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
    clippy::zero_prefixed_literal,
    clippy::identity_op,
    clippy::too_many_arguments
)]
#![cfg_attr(feature = "nightly", feature(stdsimd), feature(avx512_target_feature))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use core::fmt::Debug;
use core::marker::PhantomData;
use core::slice::{from_raw_parts, from_raw_parts_mut};

use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
use seal::Seal;

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

impl<R, F: FnOnce() -> R> WithSimd for F {
    type Output = R;

    #[inline(always)]
    fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
        let _simd = &simd;
        self()
    }
}

#[rustfmt::skip]
pub trait Simd: Seal + Debug + Copy + Send + Sync + 'static {
    type m32s: Debug + Copy + Send + Sync + 'static;
    type f32s: Debug + Copy + Send + Sync + Pod + 'static;
    type i32s: Debug + Copy + Send + Sync + Pod + 'static;
    type u32s: Debug + Copy + Send + Sync + Pod + 'static;

    type m64s: Debug + Copy + Send + Sync + 'static;
    type f64s: Debug + Copy + Send + Sync + Pod + 'static;
    type i64s: Debug + Copy + Send + Sync + Pod + 'static;
    type u64s: Debug + Copy + Send + Sync + Pod + 'static;

    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output;

    #[inline] fn f32s_as_simd(slice: &[f32]) -> (&[Self::f32s], &[f32]) { unsafe { split_slice(slice) } }
    #[inline] fn f32s_as_mut_simd(slice: &mut [f32]) -> (&mut [Self::f32s], &mut [f32]) { unsafe { split_mut_slice(slice) } }
    #[inline] fn f64s_as_simd(slice: &[f64]) -> (&[Self::f64s], &[f64]) { unsafe { split_slice(slice) } }
    #[inline] fn f64s_as_mut_simd(slice: &mut [f64]) -> (&mut [Self::f64s], &mut [f64]) { unsafe { split_mut_slice(slice) } }

    fn m32s_not(self, a: Self::m32s) -> Self::m32s;
    fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;
    fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;
    fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s;

    fn m64s_not(self, a: Self::m64s) -> Self::m64s;
    fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;
    fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;
    fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s;

    fn u32s_not(self, a: Self::u32s) -> Self::u32s;
    fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;

    fn u64s_not(self, a: Self::u64s) -> Self::u64s;
    fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;

    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s;
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s;

    #[inline] fn i32s_not(self, a: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_not(self.i32s_transmute_u32s(a))) }
    #[inline] fn i32s_and(self, a: Self::i32s, b: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_and(self.i32s_transmute_u32s(a), self.i32s_transmute_u32s(b))) }
    #[inline] fn i32s_or(self, a: Self::i32s, b: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_or(self.i32s_transmute_u32s(a), self.i32s_transmute_u32s(b))) }
    #[inline] fn i32s_xor(self, a: Self::i32s, b: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_xor(self.i32s_transmute_u32s(a), self.i32s_transmute_u32s(b))) }

    #[inline] fn i64s_not(self, a: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_not(self.i64s_transmute_u64s(a))) }
    #[inline] fn i64s_and(self, a: Self::i64s, b: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_and(self.i64s_transmute_u64s(a), self.i64s_transmute_u64s(b))) }
    #[inline] fn i64s_or(self, a: Self::i64s, b: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_or(self.i64s_transmute_u64s(a), self.i64s_transmute_u64s(b))) }
    #[inline] fn i64s_xor(self, a: Self::i64s, b: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_xor(self.i64s_transmute_u64s(a), self.i64s_transmute_u64s(b))) }

    #[inline] fn f32s_not(self, a: Self::f32s) -> Self::f32s { self.u32s_transmute_f32s(self.u32s_not(self.f32s_transmute_u32s(a))) }
    #[inline] fn f32s_and(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { self.u32s_transmute_f32s(self.u32s_and(self.f32s_transmute_u32s(a), self.f32s_transmute_u32s(b))) }
    #[inline] fn f32s_or(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { self.u32s_transmute_f32s(self.u32s_or(self.f32s_transmute_u32s(a), self.f32s_transmute_u32s(b))) }
    #[inline] fn f32s_xor(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { self.u32s_transmute_f32s(self.u32s_xor(self.f32s_transmute_u32s(a), self.f32s_transmute_u32s(b))) }

    #[inline] fn f64s_not(self, a: Self::f64s) -> Self::f64s { self.u64s_transmute_f64s(self.u64s_not(self.f64s_transmute_u64s(a))) }
    #[inline] fn f64s_and(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { self.u64s_transmute_f64s(self.u64s_and(self.f64s_transmute_u64s(a), self.f64s_transmute_u64s(b))) }
    #[inline] fn f64s_or(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { self.u64s_transmute_f64s(self.u64s_or(self.f64s_transmute_u64s(a), self.f64s_transmute_u64s(b))) }
    #[inline] fn f64s_xor(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { self.u64s_transmute_f64s(self.u64s_xor(self.f64s_transmute_u64s(a), self.f64s_transmute_u64s(b))) }

    #[inline] fn m32s_select_i32s(self, mask: Self::m32s, if_true: Self::i32s, if_false: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.m32s_select_u32s(mask, self.i32s_transmute_u32s(if_true), self.i32s_transmute_u32s(if_false))) }
    #[inline] fn m32s_select_f32s(self, mask: Self::m32s, if_true: Self::f32s, if_false: Self::f32s) -> Self::f32s { self.u32s_transmute_f32s(self.m32s_select_u32s(mask, self.f32s_transmute_u32s(if_true), self.f32s_transmute_u32s(if_false))) }
    #[inline] fn m64s_select_i64s(self, mask: Self::m64s, if_true: Self::i64s, if_false: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.m64s_select_u64s(mask, self.i64s_transmute_u64s(if_true), self.i64s_transmute_u64s(if_false))) }
    #[inline] fn m64s_select_f64s(self, mask: Self::m64s, if_true: Self::f64s, if_false: Self::f64s) -> Self::f64s { self.u64s_transmute_f64s(self.m64s_select_u64s(mask, self.f64s_transmute_u64s(if_true), self.f64s_transmute_u64s(if_false))) }

    fn u32s_splat(self, value: u32) -> Self::u32s;
    fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
    fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;

    fn u64s_splat(self, value: u64) -> Self::u64s;
    fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;

    #[inline] fn i32s_splat(self, value: i32) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_splat(value as u32)) }
    #[inline] fn i32s_add(self, a: Self::i32s, b: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_add(self.i32s_transmute_u32s(a), self.i32s_transmute_u32s(b))) }
    #[inline] fn i32s_sub(self, a: Self::i32s, b: Self::i32s) -> Self::i32s { self.u32s_transmute_i32s(self.u32s_sub(self.i32s_transmute_u32s(a), self.i32s_transmute_u32s(b))) }

    #[inline] fn i64s_splat(self, value: i64) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_splat(value as u64)) }
    #[inline] fn i64s_add(self, a: Self::i64s, b: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_add(self.i64s_transmute_u64s(a), self.i64s_transmute_u64s(b))) }
    #[inline] fn i64s_sub(self, a: Self::i64s, b: Self::i64s) -> Self::i64s { self.u64s_transmute_i64s(self.u64s_sub(self.i64s_transmute_u64s(a), self.i64s_transmute_u64s(b))) }

    fn f32s_splat(self, value: f32) -> Self::f32s;
    #[inline] fn f32s_abs(self, a: Self::f32s) -> Self::f32s { self.f32s_and(self.f32s_not(self.f32s_splat(-0.0)), a) }
    #[inline] fn f32s_neg(self, a: Self::f32s) -> Self::f32s { self.f32s_xor(self.f32s_splat(-0.0), a) }
    fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { self.f32s_add(self.f32s_mul(a, b), c) }
    fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
    #[inline] fn f32s_greater_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { self.f32s_less_than(b, a) }
    #[inline] fn f32s_greater_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { self.f32s_less_than_or_equal(b, a) }
    fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
    fn f32s_reduce_sum(self, a: Self::f32s) -> f32;
    fn f32s_reduce_product(self, a: Self::f32s) -> f32;
    fn f32s_reduce_min(self, a: Self::f32s) -> f32;
    fn f32s_reduce_max(self, a: Self::f32s) -> f32;

    fn f64s_splat(self, value: f64) -> Self::f64s;
    #[inline] fn f64s_abs(self, a: Self::f64s) -> Self::f64s { self.f64s_and(self.f64s_not(self.f64s_splat(-0.0)), a) }
    #[inline] fn f64s_neg(self, a: Self::f64s) -> Self::f64s { self.f64s_xor(a, self.f64s_splat(-0.0)) }
    fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { self.f64s_add(self.f64s_mul(a, b), c) }
    fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
    #[inline] fn f64s_greater_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { self.f64s_less_than(b, a) }
    #[inline] fn f64s_greater_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { self.f64s_less_than_or_equal(b, a) }
    fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
    fn f64s_reduce_sum(self, a: Self::f64s) -> f64;
    fn f64s_reduce_product(self, a: Self::f64s) -> f64;
    fn f64s_reduce_min(self, a: Self::f64s) -> f64;
    fn f64s_reduce_max(self, a: Self::f64s) -> f64;

    #[inline] fn f32s_transmute_i32s(self, a: Self::f32s) -> Self::i32s { cast(a) }
    #[inline] fn f32s_transmute_u32s(self, a: Self::f32s) -> Self::u32s { cast(a) }
    #[inline] fn i32s_transmute_f32s(self, a: Self::i32s) -> Self::f32s { cast(a) }
    #[inline] fn i32s_transmute_u32s(self, a: Self::i32s) -> Self::u32s { cast(a) }
    #[inline] fn u32s_transmute_f32s(self, a: Self::u32s) -> Self::f32s { cast(a) }
    #[inline] fn u32s_transmute_i32s(self, a: Self::u32s) -> Self::i32s { cast(a) }

    #[inline] fn f64s_transmute_i64s(self, a: Self::f64s) -> Self::i64s { cast(a) }
    #[inline] fn f64s_transmute_u64s(self, a: Self::f64s) -> Self::u64s { cast(a) }
    #[inline] fn i64s_transmute_f64s(self, a: Self::i64s) -> Self::f64s { cast(a) }
    #[inline] fn i64s_transmute_u64s(self, a: Self::i64s) -> Self::u64s { cast(a) }
    #[inline] fn u64s_transmute_f64s(self, a: Self::u64s) -> Self::f64s { cast(a) }
    #[inline] fn u64s_transmute_i64s(self, a: Self::u64s) -> Self::i64s { cast(a) }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar {
    __private: (),
}

impl Default for Scalar {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Scalar {
    #[inline]
    pub fn new() -> Self {
        Self { __private: () }
    }
}

impl Seal for Scalar {}
#[rustfmt::skip]
impl Simd for Scalar {
    #[inline] fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output { op.with_simd(self) }

    type m32s = bool;
    type f32s = f32;
    type i32s = i32;
    type u32s = u32;

    type m64s = bool;
    type f64s = f64;
    type i64s = i64;
    type u64s = u64;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { !a }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { a & b }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { a | b }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { a ^ b }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { value }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a + b }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a - b }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a * b }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a / b }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { a == b }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { a < b }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { a <= b }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { !a }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { a & b }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { a | b }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { a ^ b }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { value }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a + b }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a - b }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a * b }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a / b }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { a == b }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { a < b }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { a <= b }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { !a }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { a & b }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { a | b }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { a ^ b }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { !a }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { a & b }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { a | b }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { a ^ b }

    #[inline] fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s { if mask { if_true } else { if_false } }
    #[inline] fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s { if mask { if_true } else { if_false } }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a.min(b) }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { a.max(b) }

    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a.min(b) }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { a.max(b) }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { a.wrapping_add(b) }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { a.wrapping_sub(b) }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { a.wrapping_add(b) }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { a.wrapping_sub(b) }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { value }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { value }

    #[inline] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 { a }
    #[inline] fn f32s_reduce_product(self, a: Self::f32s) -> f32 { a }
    #[inline] fn f32s_reduce_min(self, a: Self::f32s) -> f32 { a }
    #[inline] fn f32s_reduce_max(self, a: Self::f32s) -> f32 { a }
    #[inline] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 { a }
    #[inline] fn f64s_reduce_product(self, a: Self::f64s) -> f64 { a }
    #[inline] fn f64s_reduce_min(self, a: Self::f64s) -> f64 { a }
    #[inline] fn f64s_reduce_max(self, a: Self::f64s) -> f64 { a }
}

#[allow(unused_macros)]
macro_rules! autovectorize_impl {
    ($ty: ty, #[target_feature(enable = $tt: tt)]) => {
        #[rustfmt::skip]
        impl Simd for $ty {
            #[inline] fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
                #[target_feature(enable = $tt)]
                unsafe fn vectorize<Op: WithSimd>(this: $ty, op: Op) -> Op::Output {
                    op.with_simd(this)
                }
                unsafe { vectorize(self, op) }
            }

            type m32s = m32x4;
            type f32s = f32x4;
            type i32s = i32x4;
            type u32s = u32x4;

            type m64s = m64x2;
            type f64s = f64x2;
            type i64s = i64x2;
            type u64s = u64x2;

            #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { m32x4(a.0.flip(), a.1.flip(), a.2.flip(), a.3.flip()) }
            #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x4(m32(a.0.0 & b.0.0), m32(a.1.0 & b.1.0), m32(a.2.0 & b.2.0), m32(a.3.0 & b.3.0)) }
            #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x4(m32(a.0.0 | b.0.0), m32(a.1.0 | b.1.0), m32(a.2.0 | b.2.0), m32(a.3.0 | b.3.0)) }
            #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x4(m32(a.0.0 ^ b.0.0), m32(a.1.0 ^ b.1.0), m32(a.2.0 ^ b.2.0), m32(a.3.0 ^ b.3.0)) }

            #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { f32x4(value, value, value, value) }
            #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3) }
            #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3) }
            #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0 * b.0, a.1 * b.1, a.2 * b.2, a.3 * b.3) }
            #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0 / b.0, a.1 / b.1, a.2 / b.2, a.3 / b.3) }
            #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { m32x4(m32::new(a.0 == b.0), m32::new(a.1 == b.1), m32::new(a.2 == b.2), m32::new(a.3 == b.3)) }
            #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { m32x4(m32::new(a.0 < b.0), m32::new(a.1 < b.1), m32::new(a.2 < b.2), m32::new(a.3 < b.3)) }
            #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { m32x4(m32::new(a.0 <= b.0), m32::new(a.1 <= b.1), m32::new(a.2 <= b.2), m32::new(a.3 <= b.3)) }

            #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { m64x2(a.0.flip(), a.1.flip()) }
            #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x2(m64(a.0.0 & b.0.0), m64(a.1.0 & b.1.0)) }
            #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x2(m64(a.0.0 | b.0.0), m64(a.1.0 | b.1.0)) }
            #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x2(m64(a.0.0 ^ b.0.0), m64(a.1.0 ^ b.1.0)) }

            #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { f64x2(value, value) }
            #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0 + b.0, a.1 + b.1) }
            #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0 - b.0, a.1 - b.1) }
            #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0 * b.0, a.1 * b.1) }
            #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0 / b.0, a.1 / b.1) }
            #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { m64x2(m64::new(a.0 == b.0), m64::new(a.1 == b.1)) }
            #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { m64x2(m64::new(a.0 < b.0), m64::new(a.1 < b.1)) }
            #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { m64x2(m64::new(a.0 <= b.0), m64::new(a.1 <= b.1)) }

            #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { u32x4(!a.0, !a.1, !a.2, !a.3) }
            #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { u32x4(a.0 & b.0, a.1 & b.1, a.2 & b.2, a.3 & b.3) }
            #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { u32x4(a.0 | b.0, a.1 | b.1, a.2 | b.2, a.3 | b.3) }
            #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { u32x4(a.0 ^ b.0, a.1 ^ b.1, a.2 ^ b.2, a.3 ^ b.3) }

            #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { u64x2(!a.0, !a.1) }
            #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { u64x2(a.0 & b.0, a.1 & b.1) }
            #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { u64x2(a.0 | b.0, a.1 | b.1) }
            #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { u64x2(a.0 ^ b.0, a.1 ^ b.1) }

            #[inline] fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s { let mask: u32x4 = unsafe { ::core::mem::transmute(mask) }; self.u32s_or(self.u32s_and(mask, if_true), self.u32s_and(self.u32s_not(mask), if_false)) }
            #[inline] fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s { let mask: u64x2 = unsafe { ::core::mem::transmute(mask) }; self.u64s_or(self.u64s_and(mask, if_true), self.u64s_and(self.u64s_not(mask), if_false)) }

            #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0.min(b.0), a.1.min(b.1), a.2.min(b.2), a.3.min(b.3)) }
            #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { f32x4(a.0.max(b.0), a.1.max(b.1), a.2.max(b.2), a.3.max(b.3)) }

            #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0.min(b.0), a.1.min(b.1)) }
            #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { f64x2(a.0.max(b.0), a.1.max(b.1)) }

            #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { u32x4(a.0.wrapping_add(b.0), a.1.wrapping_add(b.1), a.2.wrapping_add(b.2), a.3.wrapping_add(b.3)) }
            #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { u32x4(a.0.wrapping_sub(b.0), a.1.wrapping_sub(b.1), a.2.wrapping_sub(b.2), a.3.wrapping_sub(b.3)) }
            #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { u64x2(a.0.wrapping_add(b.0), a.1.wrapping_add(b.1)) }
            #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { u64x2(a.0.wrapping_sub(b.0), a.1.wrapping_sub(b.1)) }

            #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { u32x4(value, value, value, value) }
            #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { u64x2(value, value) }

            #[inline] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 { (a.0 + a.2) + (a.1 + a.3) }
            #[inline] fn f32s_reduce_product(self, a: Self::f32s) -> f32 { (a.0 * a.2) * (a.1 * a.3) }
            #[inline] fn f32s_reduce_min(self, a: Self::f32s) -> f32 { a.0.min(a.2).min(a.1.min(a.3)) }
            #[inline] fn f32s_reduce_max(self, a: Self::f32s) -> f32 { a.0.max(a.2).max(a.1.max(a.3)) }
            #[inline] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 { a.0 + a.1 }
            #[inline] fn f64s_reduce_product(self, a: Self::f64s) -> f64 { a.0 * a.1 }
            #[inline] fn f64s_reduce_min(self, a: Self::f64s) -> f64 { a.0.min(a.1) }
            #[inline] fn f64s_reduce_max(self, a: Self::f64s) -> f64 { a.0.max(a.1) }
        }
    };
}

#[inline]
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

#[inline]
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

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
enum ArchInner {
    Scalar(crate::Scalar),
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        Self::Scalar(crate::Scalar::new())
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            ArchInner::Scalar(simd) => simd.vectorize(op),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod __x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use __x86::ArchInner;

#[cfg(target_arch = "aarch64")]
mod __aarch64;
#[cfg(target_arch = "aarch64")]
use __aarch64::ArchInner;

impl Arch {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: ArchInner::new(),
        }
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

#[derive(Debug, Clone, Copy)]
pub struct Arch {
    inner: ArchInner,
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
/// High level x86 API.
pub mod x86;
