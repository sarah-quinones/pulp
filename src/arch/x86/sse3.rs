use super::*;

pub unsafe trait Sse3Token: Sse2Token {}

pub trait Sse3: Sse3Token {
    /// Alternatively add and subtract packed single-precision (32-bit)
    /// floating-point elements in `a` to/from packed elements in `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_addsub_ps)
    #[doc(alias = "_mm_addsub_ps")]
    #[inline(always)]
    fn f32x4_addsub(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_addsub_ps(a.0, b.0)) }
    }

    /// Alternatively add and subtract packed double-precision (64-bit)
    /// floating-point elements in `a` to/from packed elements in `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_addsub_pd)
    #[doc(alias = "_mm_addsub_pd")]
    #[inline(always)]
    fn f64x2_addsub(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_addsub_pd(a.0, b.0)) }
    }

    /// Horizontally adds adjacent pairs of single-precision (32-bit)
    /// floating-point elements in `a` and `b`, and pack the results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_ps)
    #[doc(alias = "_mm_hadd_ps")]
    #[inline(always)]
    fn f32x4_horizontal_add(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_hadd_ps(a.0, b.0)) }
    }

    /// Horizontally adds adjacent pairs of double-precision (64-bit)
    /// floating-point elements in `a` and `b`, and pack the results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_pd)
    #[doc(alias = "_mm_hadd_pd")]
    #[inline(always)]
    fn f64x2_horizontal_add(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_hadd_pd(a.0, b.0)) }
    }

    /// Horizontally subtract adjacent pairs of single-precision (32-bit)
    /// floating-point elements in `a` and `b`, and pack the results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_ps)
    #[doc(alias = "_mm_hsub_ps")]
    #[inline(always)]
    fn f32x4_hsub(self, a: f32x4, b: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_hsub_ps(a.0, b.0)) }
    }

    /// Horizontally subtract adjacent pairs of double-precision (64-bit)
    /// floating-point elements in `a` and `b`, and pack the results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_pd)
    #[doc(alias = "_mm_hsub_pd")]
    #[inline(always)]
    fn f64x2_hsub(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_hsub_pd(a.0, b.0)) }
    }

    /// Duplicate the low double-precision (64-bit) floating-point element
    /// from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_movedup_pd)
    #[doc(alias = "_mm_movedup_pd")]
    #[inline(always)]
    fn f64x2_duplicate(self, a: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_movedup_pd(a.0)) }
    }

    /// Duplicate odd-indexed single-precision (32-bit) floating-point elements
    /// from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_movehdup_ps)
    #[doc(alias = "_mm_movehdup_ps")]
    #[inline(always)]
    fn f32x4_duplicate_high(self, a: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_movehdup_ps(a.0)) }
    }

    /// Duplicate even-indexed single-precision (32-bit) floating-point elements
    /// from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_movehdup_ps)
    #[doc(alias = "_mm_moveldup_ps")]
    #[inline(always)]
    fn f32x4_duplicate_low(self, a: f32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_moveldup_ps(a.0)) }
    }
}

impl<T: Sse3Token> Sse3 for T {}
