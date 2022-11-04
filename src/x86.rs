use super::*;
use core::fmt::Debug;
use core::mem::transmute;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl f32x4 {
    #[inline]
    fn as_vec(self) -> __m128 {
        unsafe { transmute(self) }
    }
}

impl f64x2 {
    #[inline]
    fn as_vec(self) -> __m128d {
        unsafe { transmute(self) }
    }
}

impl f32x8 {
    #[inline]
    fn as_vec(self) -> __m256 {
        unsafe { transmute(self) }
    }
}

impl f64x4 {
    #[inline]
    fn as_vec(self) -> __m256d {
        unsafe { transmute(self) }
    }
}

#[cfg(feature = "nightly")]
impl f32x16 {
    #[inline]
    fn as_vec(self) -> __m512 {
        unsafe { transmute(self) }
    }
}

#[cfg(feature = "nightly")]
impl f64x8 {
    #[inline]
    fn as_vec(self) -> __m512d {
        unsafe { transmute(self) }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Sse2 {
    __private: (),
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Sse41 {
    __private: (),
    sse2: Sse2,
}

#[derive(Debug, Copy, Clone)]
pub struct Avx {
    __private: (),
}

#[derive(Debug, Copy, Clone)]
pub struct Avx2 {
    __private: (),
    avx: Avx,
}

#[derive(Debug, Copy, Clone)]
pub struct FmaAvx2 {
    __private: (),
    avx2: Avx2,
}

#[cfg(feature = "nightly")]
#[derive(Debug, Copy, Clone)]
pub struct Avx512f {
    __private: (),
    fma_avx2: FmaAvx2,
}

#[cfg(feature = "std")]
macro_rules! x86_feature_detected {
    ($tt: tt) => {
        is_x86_feature_detected!($tt)
    };
}

#[cfg(not(feature = "std"))]
macro_rules! x86_feature_detected {
    ($tt: tt) => {
        cfg!(target_arch = $tt)
    };
}

impl Sse2 {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self { __private: () }
    }
}

impl Sse41 {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self {
            __private: (),
            sse2: Sse2::new_unchecked(),
        }
    }
}

impl Avx {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self { __private: () }
    }
}

impl Avx2 {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self {
            __private: (),
            avx: Avx::new_unchecked(),
        }
    }
}

impl FmaAvx2 {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self {
            __private: (),
            avx2: Avx2::new_unchecked(),
        }
    }
}

#[cfg(feature = "nightly")]
impl Avx512f {
    #[inline]
    pub unsafe fn new_unchecked() -> Self {
        Self {
            __private: (),
            fma_avx2: FmaAvx2::new_unchecked(),
        }
    }
}

macro_rules! defer_impl {
    {
        $self: ident => $child: expr,
        $(
            fn $name:ident(self $(, $param: ident: $param_ty: ty)*) -> $ret_ty: ty;
        )*
    } => {
        $(
            #[inline]
            fn $name($self, $($param: $param_ty,)*) -> $ret_ty {
                $child.$name($($param,)*)
            }
        )*
    }
}

impl Seal for Sse2 {}
impl Seal for Sse41 {}
impl Seal for Avx {}
impl Seal for Avx2 {}
impl Seal for FmaAvx2 {}
#[cfg(feature = "nightly")]
impl Seal for Avx512f {}

#[rustfmt::skip]
impl Simd for Sse2 {
    type m32s = m32x4;
    type f32s = f32x4;
    type i32s = i32x4;
    type u32s = u32x4;

    type m64s = m64x2;
    type f64s = f64x2;
    type i64s = i64x2;
    type u64s = u64x2;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmpeq_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmplt_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmple_ps(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmpeq_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmplt_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmple_pd(a.as_vec(), b.as_vec())) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_or_pd(_mm_and_pd(mask, if_true), _mm_andnot_pd(mask, if_false)))
        }
    }
    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_or_pd(_mm_and_pd(mask, if_true), _mm_andnot_pd(mask, if_false)))
        }
    }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_max_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_sub_epi64(transmute(a), transmute(b))) } }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm_set1_epi64x(value as i64)) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "sse2")]
        unsafe fn vectorize<Op: WithSimd>(this: Sse2, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }
}

#[rustfmt::skip]
impl Simd for Sse41 {
    type m32s = m32x4;
    type f32s = f32x4;
    type i32s = i32x4;
    type u32s = u32x4;

    type m64s = m64x2;
    type f64s = f64x2;
    type i64s = i64x2;
    type u64s = u64x2;

    defer_impl! {
        self => self.sse2,

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

        fn f32s_splat(self, value: f32) -> Self::f32s;
        fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;

        fn f64s_splat(self, value: f64) -> Self::f64s;
        fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;

        fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;

        fn u32s_splat(self, value: u32) -> Self::u32s;
        fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
        fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;

        fn u64s_splat(self, value: u64) -> Self::u64s;
        fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
        fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
    }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_blendv_pd(if_false, if_true, mask))
        }
    }
    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_blendv_pd(if_false, if_true, mask))
        }
    }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "sse4.1")]
        unsafe fn vectorize<Op: WithSimd>(this: Sse41, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }
}

#[rustfmt::skip]
impl Simd for Avx {
    type m32s = m32x8;
    type f32s = f32x8;
    type i32s = i32x8;
    type u32s = u32x8;

    type m64s = m64x4;
    type f64s = f64x4;
    type i64s = i64x4;
    type u64s = u64x4;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm256_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm256_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __m256d = transmute(mask);
            let if_true: __m256d = transmute(if_true);
            let if_false: __m256d = transmute(if_false);

            transmute(_mm256_blendv_pd(if_false, if_true, mask))
        }
    }
    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __m256d = transmute(mask);
            let if_true: __m256d = transmute(if_true);
            let if_false: __m256d = transmute(if_false);

            transmute(_mm256_blendv_pd(if_false, if_true, mask))
        }
    }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "avx")]
        unsafe fn vectorize<Op: WithSimd>(this: Avx, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_max_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        unsafe {
            let a: [__m128i; 2] = transmute(a);
            let b: [__m128i; 2] = transmute(b);
            let c: [__m128i; 2] = [_mm_add_epi32(a[0], b[0]), _mm_add_epi32(a[1], b[1])];
            transmute(c)
        }
    }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s {
        unsafe {
            let a: [__m128i; 2] = transmute(a);
            let b: [__m128i; 2] = transmute(b);
            let c: [__m128i; 2] = [_mm_sub_epi32(a[0], b[0]), _mm_sub_epi32(a[1], b[1])];
            transmute(c)
        }
    }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        unsafe {
            let a: [__m128i; 2] = transmute(a);
            let b: [__m128i; 2] = transmute(b);
            let c: [__m128i; 2] = [_mm_add_epi64(a[0], b[0]), _mm_add_epi64(a[1], b[1])];
            transmute(c)
        }
    }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s {
        unsafe {
            let a: [__m128i; 2] = transmute(a);
            let b: [__m128i; 2] = transmute(b);
            let c: [__m128i; 2] = [_mm_sub_epi64(a[0], b[0]), _mm_sub_epi64(a[1], b[1])];
            transmute(c)
        }
    }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm256_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm256_set1_epi64x(value as i64)) } }
}

#[rustfmt::skip]
impl Simd for Avx2 {
    type m32s = m32x8;
    type f32s = f32x8;
    type i32s = i32x8;
    type u32s = u32x8;

    type m64s = m64x4;
    type f64s = f64x4;
    type i64s = i64x4;
    type u64s = u64x4;

    defer_impl! {
        self => self.avx,
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

        fn f32s_splat(self, value: f32) -> Self::f32s;
        fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;

        fn f64s_splat(self, value: f64) -> Self::f64s;
        fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;

        fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s;
        fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s;

        fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;

        fn u32s_splat(self, value: u32) -> Self::u32s;
        fn u64s_splat(self, value: u64) -> Self::u64s;
    }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_sub_epi64(transmute(a), transmute(b))) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "avx2")]
        unsafe fn vectorize<Op: WithSimd>(this: Avx2, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }
}

#[rustfmt::skip]
impl Simd for FmaAvx2 {
    type m32s = m32x8;
    type f32s = f32x8;
    type i32s = i32x8;
    type u32s = u32x8;

    type m64s = m64x4;
    type f64s = f64x4;
    type i64s = i64x4;
    type u64s = u64x4;

    defer_impl! {
        self => self.avx2,
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

        fn f32s_splat(self, value: f32) -> Self::f32s;
        fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;
        fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s;

        fn f64s_splat(self, value: f64) -> Self::f64s;
        fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;
        fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s;

        fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s;
        fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s;

        fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s;
        fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;
        fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s;

        fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
        fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s;
        fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
        fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s;
        fn u32s_splat(self, value: u32) -> Self::u32s;
        fn u64s_splat(self, value: u64) -> Self::u64s;
    }

    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_fmadd_pd(a.as_vec(), b.as_vec(), c.as_vec())) } }
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_fmadd_ps(a.as_vec(), b.as_vec(), c.as_vec())) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "fma,avx2")]
        unsafe fn vectorize<Op: WithSimd>(this: FmaAvx2, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
#[cfg(feature = "nightly")]
pub struct m32x16(pub __mmask16);

#[derive(Copy, Clone)]
#[repr(transparent)]
#[cfg(feature = "nightly")]
pub struct m64x8(pub __mmask8);

#[cfg(feature = "nightly")]
impl Debug for m64x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("m64x8")
            .field(&((self.0 >> 0x0) & 1 == 1))
            .field(&((self.0 >> 0x1) & 1 == 1))
            .field(&((self.0 >> 0x2) & 1 == 1))
            .field(&((self.0 >> 0x3) & 1 == 1))
            .field(&((self.0 >> 0x4) & 1 == 1))
            .field(&((self.0 >> 0x5) & 1 == 1))
            .field(&((self.0 >> 0x6) & 1 == 1))
            .field(&((self.0 >> 0x7) & 1 == 1))
            .finish()
    }
}

#[cfg(feature = "nightly")]
impl Debug for m32x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("m32x16")
            .field(&((self.0 >> 0x0) & 1 == 1))
            .field(&((self.0 >> 0x1) & 1 == 1))
            .field(&((self.0 >> 0x2) & 1 == 1))
            .field(&((self.0 >> 0x3) & 1 == 1))
            .field(&((self.0 >> 0x4) & 1 == 1))
            .field(&((self.0 >> 0x5) & 1 == 1))
            .field(&((self.0 >> 0x6) & 1 == 1))
            .field(&((self.0 >> 0x7) & 1 == 1))
            .field(&((self.0 >> 0x8) & 1 == 1))
            .field(&((self.0 >> 0x9) & 1 == 1))
            .field(&((self.0 >> 0xa) & 1 == 1))
            .field(&((self.0 >> 0xb) & 1 == 1))
            .field(&((self.0 >> 0xc) & 1 == 1))
            .field(&((self.0 >> 0xd) & 1 == 1))
            .field(&((self.0 >> 0xe) & 1 == 1))
            .field(&((self.0 >> 0xf) & 1 == 1))
            .finish()
    }
}

#[cfg(feature = "nightly")]
#[rustfmt::skip]
impl Simd for Avx512f {
    type m32s = m32x16;
    type f32s = f32x16;
    type i32s = i32x16;
    type u32s = u32x16;

    type m64s = m64x8;
    type f64s = f64x8;
    type i64s = i64x8;
    type u64s = u64x8;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { m32x16(!a.0) }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 & b.0) }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 | b.0) }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 ^ b.0) }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { m64x8(!a.0) }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 & b.0) }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 | b.0) }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 ^ b.0) }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_xor_si512(_mm512_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_and_si512(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_or_si512(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_xor_si512(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_xor_si512(_mm512_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_and_si512(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_or_si512(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_xor_si512(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm512_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm512_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_fmadd_pd(a.as_vec(), b.as_vec(), c.as_vec())) } }
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_fmadd_ps(a.as_vec(), b.as_vec(), c.as_vec())) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __mmask16 = mask.0;
            let if_true: __m512 = transmute(if_true);
            let if_false: __m512 = transmute(if_false);

            transmute(_mm512_mask_blend_ps(mask, if_false, if_true))
        }
    }

    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __mmask8 = mask.0;
            let if_true: __m512d = transmute(if_true);
            let if_false: __m512d = transmute(if_false);

            transmute(_mm512_mask_blend_pd(mask, if_false, if_true))
        }
    }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_max_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_sub_epi64(transmute(a), transmute(b))) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        #[target_feature(enable = "avx512f")]
        unsafe fn vectorize<Op: WithSimd>(this: Avx512f, op: Op) -> Op::Output {
            op.with_simd(this)
        }
        unsafe { vectorize(self, op) }
    }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm512_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm512_set1_epi64(value as i64)) } }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchInner {
    #[cfg(feature = "nightly")]
    Avx512f(Avx512f),
    FmaAvx2(FmaAvx2),
    Avx(Avx),
    Sse41(Sse41),
    Sse2(Sse2),
    Scalar(crate::Scalar),
}

impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            #[cfg(feature = "nightly")]
            {
                if x86_feature_detected!("avx512f") {
                    return Self::Avx512f(Avx512f::new_unchecked());
                }
            }
            if x86_feature_detected!("fma") && x86_feature_detected!("avx2") {
                Self::FmaAvx2(FmaAvx2::new_unchecked())
            } else if x86_feature_detected!("avx") {
                Self::Avx(Avx::new_unchecked())
            } else if x86_feature_detected!("sse4.1") {
                Self::Sse41(Sse41::new_unchecked())
            } else if x86_feature_detected!("sse2") {
                Self::Sse2(Sse2::new_unchecked())
            } else {
                Self::Scalar(crate::Scalar::new())
            }
        }
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            #[cfg(feature = "nightly")]
            ArchInner::Avx512f(simd) => simd.vectorize(op),
            ArchInner::FmaAvx2(simd) => simd.vectorize(op),
            ArchInner::Avx(simd) => simd.vectorize(op),
            ArchInner::Sse41(simd) => simd.vectorize(op),
            ArchInner::Sse2(simd) => simd.vectorize(op),
            ArchInner::Scalar(simd) => simd.vectorize(op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn times_two() {
        let n = 1312;
        let mut v = (0..n).map(|i| i as f64).collect::<Vec<_>>();
        let arch = Arch::new();

        struct TimesThree<'a>(&'a mut [f64]);
        impl<'a> WithSimd for TimesThree<'a> {
            type Output = ();

            #[inline(always)]
            fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
                let v = self.0;
                let (head, tail) = S::f64s_as_mut_simd(v);

                let three = simd.f64s_splat(3.0);
                for x in head {
                    *x = simd.f64s_mul(three, *x);
                }

                for x in tail {
                    *x = *x * 3.0;
                }
            }
        }

        arch.dispatch(|| {
            for x in &mut v {
                *x *= 2.0;
            }
        });

        arch.dispatch(TimesThree(&mut v));

        for (i, x) in v.into_iter().enumerate() {
            assert_eq!(x, 6.0 * i as f64);
        }
    }
}
