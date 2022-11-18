use super::*;
use crate::arch::internal_simd_type;
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

// https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels
internal_simd_type!(pub Baseline, "sse", "sse2", "fxsr");
internal_simd_type!(pub V2, "sse", "sse2", "fxsr", "sse3", "ssse3", "sse4.1", "sse4.2", "popcnt");
internal_simd_type!(
    pub V3, "sse", "sse2", "fxsr", "sse3", "ssse3", "sse4.1", "sse4.2", "popcnt", "avx", "avx2",
    "bmi1", "bmi2", "fma", "lzcnt"
);
#[cfg(feature = "nightly")]
internal_simd_type!(
    pub V4, "sse", "sse2", "fxsr", "sse3", "ssse3", "sse4.1", "sse4.2", "popcnt", "avx", "avx2",
    "bmi1", "bmi2", "fma", "lzcnt", "avx512f", "avx512bw", "avx512cd", "avx512dq", "avx512vl"
);

impl Seal for Baseline {}
impl Seal for V2 {}
impl Seal for V3 {}
#[cfg(feature = "nightly")]
impl Seal for V4 {}

#[rustfmt::skip]
impl Simd for Baseline {
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
        self.vectorize(#[inline(always)] || op.with_simd(self))
    }

    #[inline(always)] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            // a0 a1 a2 a3
            let a: __m128 = transmute(a);
            // a2 a3 a2 a3
            let hi = _mm_movehl_ps(a, a);

            // a0+a2 a1+a3 _ _
            let r0 = _mm_add_ps(a, hi);
            // a1+a3 a2+a1 _ _
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);

            let r = _mm_add_ss(r0, r0_shuffled);

            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)] fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_mul_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_mul_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)] fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_min_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_min_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)] fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_max_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_max_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_add_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)] fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_mul_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)] fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_min_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)] fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_max_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
}

#[rustfmt::skip]
impl Simd for V2 {
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
        self.vectorize(#[inline(always)] || op.with_simd(self))
    }

    #[inline(always)] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 { unsafe { Baseline::f32s_reduce_sum(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f32s_reduce_product(self, a: Self::f32s) -> f32 { unsafe { Baseline::f32s_reduce_product(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f32s_reduce_min(self, a: Self::f32s) -> f32 { unsafe { Baseline::f32s_reduce_min(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f32s_reduce_max(self, a: Self::f32s) -> f32 { unsafe { Baseline::f32s_reduce_max(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 { unsafe { Baseline::f64s_reduce_sum(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f64s_reduce_product(self, a: Self::f64s) -> f64 { unsafe { Baseline::f64s_reduce_product(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f64s_reduce_min(self, a: Self::f64s) -> f64 { unsafe { Baseline::f64s_reduce_min(Baseline::new_unchecked(), a) } }
    #[inline(always)] fn f64s_reduce_max(self, a: Self::f64s) -> f64 { unsafe { Baseline::f64s_reduce_max(Baseline::new_unchecked(), a) } }
}

#[rustfmt::skip]
impl Simd for V3 {
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

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_max_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm256_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm256_set1_epi64x(value as i64)) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_sub_epi64(transmute(a), transmute(b))) } }

    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_fmadd_pd(a.as_vec(), b.as_vec(), c.as_vec())) } }
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_fmadd_ps(a.as_vec(), b.as_vec(), c.as_vec())) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        self.vectorize(#[inline(always)] || op.with_simd(self))
    }

    #[inline(always)] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_add_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_mul_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_product(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_min_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_min(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_max_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_max(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_add_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_mul_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_product(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_min_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_min(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_max_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_max(transmute(r))
        }
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
impl Simd for V4 {
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
        self.vectorize(#[inline(always)] || op.with_simd(self))
    }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm512_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm512_set1_epi64(value as i64)) } }

    #[inline(always)] fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_add_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_mul_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_product(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_min_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_min(transmute(r))
        }
    }

    #[inline(always)] fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_max_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_max(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_add_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_mul_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_product(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_min_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_min(transmute(r))
        }
    }

    #[inline(always)] fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_max_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_max(transmute(r))
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchInner {
    #[cfg(feature = "nightly")]
    V4(V4),
    V3(V3),
    V2(V2),
    Baseline(Baseline),
    Scalar(crate::Scalar),
}

impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        #[cfg(feature = "nightly")]
        if let Some(simd) = V4::try_new() {
            return Self::V4(simd);
        }
        if let Some(simd) = V3::try_new() {
            return Self::V3(simd);
        }
        if let Some(simd) = V2::try_new() {
            return Self::V2(simd);
        }
        if let Some(simd) = Baseline::try_new() {
            return Self::Baseline(simd);
        }
        Self::Scalar(crate::Scalar::new())
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            #[cfg(feature = "nightly")]
            ArchInner::V4(simd) => Simd::vectorize(simd, op),
            ArchInner::V3(simd) => Simd::vectorize(simd, op),
            ArchInner::V2(simd) => Simd::vectorize(simd, op),
            ArchInner::Baseline(simd) => Simd::vectorize(simd, op),
            ArchInner::Scalar(simd) => Simd::vectorize(simd, op),
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
