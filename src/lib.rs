#![allow(non_camel_case_types)]
#![cfg_attr(feature = "nightly", feature(stdsimd), feature(avx512_target_feature))]

use core::{
    fmt::Debug,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use bytemuck::{NoUninit, Pod, Zeroable};
use seal::Seal;

mod seal {
    pub trait Seal {}
}

pub trait WithSimd {
    type Output;

    fn with_simd<S: Simd>(self, simd: S) -> Self::Output;
}

impl<F: FnOnce()> WithSimd for F {
    type Output = ();

    fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
        let _simd = &simd;
        self()
    }
}

#[rustfmt::skip]
pub trait Simd: Seal + Debug + Copy + Send + Sync + 'static {
    type m32s: Debug + Copy + Send + Sync + 'static;
    type f32s: Debug + Copy + Send + Sync + 'static;
    type i32s: Debug + Copy + Send + Sync + 'static;
    type u32s: Debug + Copy + Send + Sync + 'static;

    type m64s: Debug + Copy + Send + Sync + 'static;
    type f64s: Debug + Copy + Send + Sync + 'static;
    type i64s: Debug + Copy + Send + Sync + 'static;
    type u64s: Debug + Copy + Send + Sync + 'static;

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

    fn f32s_splat(self, value: f32) -> Self::f32s;
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

    fn f64s_splat(self, value: f64) -> Self::f64s;
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

    #[inline] fn f32s_transmute_i32s(self, a: Self::f32s) -> Self::i32s { unsafe { transmute(a) } }
    #[inline] fn f32s_transmute_u32s(self, a: Self::f32s) -> Self::u32s { unsafe { transmute(a) } }
    #[inline] fn i32s_transmute_f32s(self, a: Self::i32s) -> Self::f32s { unsafe { transmute(a) } }
    #[inline] fn i32s_transmute_u32s(self, a: Self::i32s) -> Self::u32s { unsafe { transmute(a) } }
    #[inline] fn u32s_transmute_f32s(self, a: Self::u32s) -> Self::f32s { unsafe { transmute(a) } }
    #[inline] fn u32s_transmute_i32s(self, a: Self::u32s) -> Self::i32s { unsafe { transmute(a) } }

    #[inline] fn f64s_transmute_i64s(self, a: Self::f64s) -> Self::i64s { unsafe { transmute(a) } }
    #[inline] fn f64s_transmute_u64s(self, a: Self::f64s) -> Self::u64s { unsafe { transmute(a) } }
    #[inline] fn i64s_transmute_f64s(self, a: Self::i64s) -> Self::f64s { unsafe { transmute(a) } }
    #[inline] fn i64s_transmute_u64s(self, a: Self::i64s) -> Self::u64s { unsafe { transmute(a) } }
    #[inline] fn u64s_transmute_f64s(self, a: Self::u64s) -> Self::f64s { unsafe { transmute(a) } }
    #[inline] fn u64s_transmute_i64s(self, a: Self::u64s) -> Self::i64s { unsafe { transmute(a) } }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar {
    __private: (),
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

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        op.with_simd(self)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
#[doc(hidden)]
pub struct m8(u8);
#[derive(Copy, Clone)]
#[repr(transparent)]
#[doc(hidden)]
pub struct m16(u16);
#[derive(Copy, Clone)]
#[repr(transparent)]
#[doc(hidden)]
pub struct m32(u32);
#[derive(Copy, Clone)]
#[repr(transparent)]
#[doc(hidden)]
pub struct m64(u64);

impl Debug for m8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.is_set())
    }
}
impl Debug for m16 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.is_set())
    }
}
impl Debug for m32 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.is_set())
    }
}
impl Debug for m64 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.is_set())
    }
}

impl m8 {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self(if value { u8::MAX } else { 0 })
    }

    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u8::MAX
    }

    #[inline]
    pub fn flip(self) -> Self {
        Self(!self.0)
    }
}
impl m16 {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self(if value { u16::MAX } else { 0 })
    }

    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u16::MAX
    }

    #[inline]
    pub fn flip(self) -> Self {
        Self(!self.0)
    }
}
impl m32 {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self(if value { u32::MAX } else { 0 })
    }

    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u32::MAX
    }

    #[inline]
    pub fn flip(self) -> Self {
        Self(!self.0)
    }
}
impl m64 {
    #[inline]
    pub fn new(value: bool) -> Self {
        Self(if value { u64::MAX } else { 0 })
    }

    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u64::MAX
    }

    #[inline]
    pub fn flip(self) -> Self {
        Self(!self.0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct f32x4(f32, f32, f32, f32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct f32x8(f32, f32, f32, f32, f32, f32, f32, f32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
#[doc(hidden)]
pub struct f32x16(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct i32x4(i32, i32, i32, i32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct i32x8(i32, i32, i32, i32, i32, i32, i32, i32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
#[doc(hidden)]
pub struct i32x16(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct u32x4(u32, u32, u32, u32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct u32x8(u32, u32, u32, u32, u32, u32, u32, u32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
#[doc(hidden)]
pub struct u32x16(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct m32x4(m32, m32, m32, m32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct m32x8(m32, m32, m32, m32, m32, m32, m32, m32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct f64x2(f64, f64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct f64x4(f64, f64, f64, f64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct f64x8(f64, f64, f64, f64, f64, f64, f64, f64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct i64x2(i64, i64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct i64x4(i64, i64, i64, i64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct i64x8(i64, i64, i64, i64, i64, i64, i64, i64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct u64x2(u64, u64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct u64x4(u64, u64, u64, u64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct u64x8(u64, u64, u64, u64, u64, u64, u64, u64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct m64x2(m64, m64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[doc(hidden)]
pub struct m64x4(m64, m64, m64, m64);

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

unsafe impl NoUninit for m32x4 {}
unsafe impl NoUninit for m32x8 {}
unsafe impl NoUninit for m64x2 {}
unsafe impl NoUninit for m64x4 {}

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

#[inline]
unsafe fn transmute<T, U>(src: T) -> U {
    assert_eq!(core::mem::size_of::<T>(), core::mem::size_of::<U>());
    core::mem::transmute_copy(&src)
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Arch {
    Scalar(crate::Scalar),
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl Arch {
    #[inline]
    pub fn new() -> Self {
        Self::Scalar(crate::Scalar::new())
    }

    #[inline]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            Arch::Scalar(simd) => simd.vectorize(op),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::Arch;
