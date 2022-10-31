use super::*;

pub unsafe trait SseToken: Copy {}

pub trait Sse: SseToken {
    /// Constructs a `f32x4` from four `f32` values.
    #[inline(always)]
    fn f32x4_new(self, e0: f32, e1: f32, e2: f32, e3: f32) -> f32x4 {
        unsafe { f32x4(arch::_mm_setr_ps(e0, e1, e2, e3)) }
    }

    /// Constructs a `f32x4` with all elements set to `a`.
    #[inline(always)]
    fn f32x4_splat(self, a: f32) -> f32x4 {
        unsafe { f32x4(arch::_mm_set1_ps(a)) }
    }

    /// Loads four `f32` values from memory into a `f32x4`.
    #[inline(always)]
    fn f32x4_load(self, addr: &[f32; 4]) -> f32x4 {
        unsafe { f32x4(arch::_mm_loadu_ps(addr as *const f32)) }
    }

    /// Stores four `f32` values from a `f32x4` into memory.
    #[inline(always)]
    fn f32x4_store(self, addr: &mut [f32; 4], a: f32x4) {
        unsafe { arch::_mm_storeu_ps(addr as *mut f32, a.0) }
    }

    /// Computes `a + b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_ps)
    #[doc(alias = "_mm_add_ps")]
    #[inline(always)]
    fn f32x4_add(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_add_ps(a.0, b.0)) }
    }

    /// Computes `a - b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_ps)
    #[doc(alias = "_mm_sub_ps")]
    #[inline(always)]
    fn f32x4_sub(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_sub_ps(a.0, b.0)) }
    }

    /// Computes `a * b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_ps)
    #[doc(alias = "_mm_mul_ps")]
    #[inline(always)]
    fn f32x4_mul(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_mul_ps(a.0, b.0)) }
    }

    /// Computes `a / b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_div_ps)
    #[doc(alias = "_mm_div_ps")]
    #[inline(always)]
    fn f32x4_div(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_div_ps(a.0, b.0)) }
    }

    /// Computes `sqrt(a)` for each element in `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sqrt_ps)
    #[doc(alias = "_mm_sqrt_ps")]
    #[inline(always)]
    fn f32x4_sqrt(self, a: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_sqrt_ps(a.0)) }
    }

    /// Computes the approximate reciprocal of `a` for each element in `a`.
    ///
    /// The maximum relative error for this approximation is less than `1.5*2^-12`
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rcp_ps)
    #[doc(alias = "_mm_rcp_ps")]
    #[inline(always)]
    fn f32x4_rcp(self, a: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_rcp_ps(a.0)) }
    }

    /// Computes the approximate reciprocal square root of `a` for each element in `a`.
    ///
    /// The maximum relative error for this approximation is less than `1.5*2^-12`
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_rsqrt_ps)
    #[doc(alias = "_mm_rsqrt_ps")]
    #[inline(always)]
    fn f32x4_rsqrt(self, a: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_rsqrt_ps(a.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise minimum.
    ///
    /// # Warning
    ///
    /// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) minimum value
    /// when inputs are NaN or signed-zero values.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_ps)
    #[doc(alias = "_mm_min_ps")]
    #[inline(always)]
    fn f32x4_min(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_min_ps(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise maximum.
    ///
    /// # Warning
    ///
    /// Does not follow the IEEE Standard for Floating-Point Arithmetic (IEEE 754) maximum value
    /// when inputs are NaN or signed-zero values.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_ps)
    #[doc(alias = "_mm_max_ps")]
    #[inline(always)]
    fn f32x4_max(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_max_ps(a.0, b.0)) }
    }

    /// Computes `a & b` for each bit in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_ps)
    #[doc(alias = "_mm_and_ps")]
    #[inline(always)]
    fn f32x4_and(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_and_ps(a.0, b.0)) }
    }

    /// Computes `!a & b` for each bit in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_ps)
    #[doc(alias = "_mm_andnot_ps")]
    #[inline(always)]
    fn f32x4_andnot(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_andnot_ps(a.0, b.0)) }
    }

    /// Computes `a | b` for each bit in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_ps)
    #[doc(alias = "_mm_or_ps")]
    #[inline(always)]
    fn f32x4_or(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_or_ps(a.0, b.0)) }
    }

    /// Computes `a ^ b` for each bit in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_ps)
    #[doc(alias = "_mm_xor_ps")]
    #[inline(always)]
    fn f32x4_xor(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_xor_ps(a.0, b.0)) }
    }

    /// Compares `a` and `b` for equality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_ps)
    #[doc(alias = "_mm_cmpeq_ps")]
    #[inline(always)]
    fn f32x4_cmp_eq(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpeq_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmplt_ps)
    #[doc(alias = "_mm_cmplt_ps")]
    #[inline(always)]
    fn f32x4_cmp_lt(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmplt_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for less-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmple_ps)
    #[doc(alias = "_mm_cmple_ps")]
    #[inline(always)]
    fn f32x4_cmp_le(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmple_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpgt_ps)
    #[doc(alias = "_mm_cmpgt_ps")]
    #[inline(always)]
    fn f32x4_cmp_gt(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpgt_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpge_ps)
    #[doc(alias = "_mm_cmpge_ps")]
    #[inline(always)]
    fn f32x4_cmp_ge(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpge_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for inequality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_ps)
    #[doc(alias = "_mm_cmpeq_ps")]
    #[inline(always)]
    fn f32x4_cmp_neq(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpneq_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not-less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnlt_ps)
    #[doc(alias = "_mm_cmpnlt_ps")]
    #[inline(always)]
    fn f32x4_cmp_not_lt(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpnlt_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not-less-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnle_ps)
    #[doc(alias = "_mm_cmpnle_ps")]
    #[inline(always)]
    fn f32x4_cmp_not_le(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpnle_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpngt_ps)
    #[doc(alias = "_mm_cmpngt_ps")]
    #[inline(always)]
    fn f32x4_cmp_not_gt(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpngt_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnge_ps)
    #[doc(alias = "_mm_cmpnge_ps")]
    #[inline(always)]
    fn f32x4_cmp_not_ge(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpnge_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for the "ordered" relation, for each element in `a` and `b`.
    ///
    /// Two elements are ordered if neither of them is a NaN.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpord_ps)
    #[doc(alias = "_mm_cmpord_ps")]
    #[inline(always)]
    fn f32x4_cmp_ord(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpord_ps(a.0, b.0))) }
    }

    /// Compares `a` and `b` for the "unordered" relation, for each element in `a` and `b`.
    ///
    /// Two elements are unordered if at least one of them is a NaN.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpunord_ps)
    #[doc(alias = "_mm_cmpunord_ps")]
    #[inline(always)]
    fn f32x4_cmp_unord(self, a: f32x4, b: f32x4) -> m32x4 {
        unsafe { m32x4(transmute(arch::_mm_cmpunord_ps(a.0, b.0))) }
    }

    /// Shuffles packed single-precision (32-bit) floating-point elements in `a` and `b` using
    /// `SHUFFLE`.
    ///
    /// The lower half of result takes values from `a` and the higher half from
    /// `b`. Mask is split to 2 control bits each to index the element from inputs.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shuffle_ps)
    #[doc(alias = "_mm_shuffle_ps")]
    #[inline(always)]
    fn f32x4_shuffle<const SHUFFLE: i32>(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_shuffle_ps::<SHUFFLE>(a.0, b.0)) }
    }

    /// Unpacks and interleave single-precision (32-bit) floating-point elements from the higher
    /// half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_ps)
    #[doc(alias = "_mm_unpackhi_ps")]
    #[inline(always)]
    fn f32x4_unpack_high(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_unpackhi_ps(a.0, b.0)) }
    }

    /// Unpacks and interleave single-precision (32-bit) floating-point elements from the lower half
    /// of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_ps)
    #[doc(alias = "_mm_unpacklo_ps")]
    #[inline(always)]
    fn f32x4_unpack_low(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_unpacklo_ps(a.0, b.0)) }
    }

    /// Combine higher half of `a` and `b`. The higher half of `b` occupies the
    /// lower half of result.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_movehl_ps)
    #[doc(alias = "_mm_movehl_ps")]
    #[inline(always)]
    fn f32x4_movehl(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_movehl_ps(a.0, b.0)) }
    }

    /// Combine lower half of `a` and `b`. The lower half of `b` occupies the
    /// higher half of result.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_movelh_ps)
    #[doc(alias = "_mm_movelh_ps")]
    #[inline(always)]
    fn f32x4_movelh(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_movelh_ps(a.0, b.0)) }
    }
}

impl<T: SseToken> Sse for T {}
