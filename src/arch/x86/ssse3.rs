use super::*;

pub unsafe trait Ssse3Token: Sse3Token {}

pub trait Ssse3: Ssse3Token {
    /// Computes the absolute value of packed 8-bit signed integers in `a` and
    /// return the unsigned results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_abs_epi8)
    #[doc(alias = "_mm_abs_epi8")]
    #[inline(always)]
    fn i8x16_abs(self, a: i8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_abs_epi8(a.0)) }
    }

    /// Computes the absolute value of each of the packed 16-bit signed integers in
    /// `a` and return the 16-bit unsigned integer
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_abs_epi16)
    #[doc(alias = "_mm_abs_epi16")]
    #[inline(always)]
    fn i16x8_abs(self, a: i16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_abs_epi16(a.0)) }
    }

    /// Computes the absolute value of each of the packed 32-bit signed integers in
    /// `a` and return the 32-bit unsigned integer
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_abs_epi32)
    #[doc(alias = "_mm_abs_epi32")]
    #[inline(always)]
    fn i32x4_abs(self, a: i32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_abs_epi32(a.0)) }
    }

    /// Shuffles bytes from `a` according to the content of `b`.
    ///
    /// The last 4 bits of each byte of `b` are used as addresses
    /// into the 16 bytes of `a`.
    ///
    /// In addition, if the highest significant bit of a byte of `b`
    /// is set, the respective destination byte is set to 0.
    ///
    /// Picturing `a` and `b` as `[u8; 16]`, `_mm_shuffle_epi8` is
    /// logically equivalent to:
    ///
    /// ```
    /// fn mm_shuffle_epi8(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    ///     let mut r = [0u8; 16];
    ///     for i in 0..16 {
    ///         // if the most significant bit of b is set,
    ///         // then the destination byte is set to 0.
    ///         if b[i] & 0x80 == 0u8 {
    ///             r[i] = a[(b[i] % 16) as usize];
    ///         }
    ///     }
    ///     r
    /// }
    /// ```
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shuffle_epi8)
    #[doc(alias = "_mm_shuffle_epi8")]
    #[inline(always)]
    fn u8x16_shuffle_dyn(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_shuffle_epi8(a.0, b.0)) }
    }

    /// Horizontally adds the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x u16]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_epi16)
    #[doc(alias = "_mm_hadd_epi16")]
    #[inline(always)]
    fn u16x8_horizontal_add(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_hadd_epi16(a.0, b.0)) }
    }

    /// Horizontally adds the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x i16]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_epi16)
    #[doc(alias = "_mm_hadd_epi16")]
    #[inline(always)]
    fn i16x8_horizontal_add(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_hadd_epi16(a.0, b.0)) }
    }

    /// Horizontally adds the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[4 x u32]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_epi32)
    #[doc(alias = "_mm_hadd_epi32")]
    #[inline(always)]
    fn u32x4_horizontal_add(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_hadd_epi32(a.0, b.0)) }
    }

    /// Horizontally adds the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[4 x i32]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadd_epi32)
    #[doc(alias = "_mm_hadd_epi32")]
    #[inline(always)]
    fn i32x4_horizontal_add(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_hadd_epi32(a.0, b.0)) }
    }

    /// Horizontally subtracts the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x u16]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_epi16)
    #[doc(alias = "_mm_hsub_epi16")]
    #[inline(always)]
    fn u16x8_horizontal_sub(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_hsub_epi16(a.0, b.0)) }
    }

    /// Horizontally subtracts the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x i16]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_epi16)
    #[doc(alias = "_mm_hsub_epi16")]
    #[inline(always)]
    fn i16x8_horizontal_sub(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_hsub_epi16(a.0, b.0)) }
    }

    /// Horizontally subtracts the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[4 x u32]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_epi32)
    #[doc(alias = "_mm_hsub_epi32")]
    #[inline(always)]
    fn u32x4_horizontal_sub(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_hsub_epi32(a.0, b.0)) }
    }

    /// Horizontally subtracts the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[4 x i32]`, with wrapping on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsub_epi32)
    #[doc(alias = "_mm_hsub_epi32")]
    #[inline(always)]
    fn i32x4_horizontal_sub(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_hsub_epi32(a.0, b.0)) }
    }

    /// Horizontally adds the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x i16]`, with saturation on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hadds_epi16)
    #[doc(alias = "_mm_hadds_epi16")]
    #[inline(always)]
    fn i16x8_horizontal_saturating_add(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_hadds_epi16(a.0, b.0)) }
    }

    /// Horizontally subtracts the adjacent pairs of values contained in 2 packed
    /// 128-bit vectors of `[8 x i16]`, with saturation on overflow.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_hsubs_epi16)
    #[doc(alias = "_mm_hsubs_epi16")]
    #[inline(always)]
    fn i16x8_horizontal_saturating_sub(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_hsubs_epi16(a.0, b.0)) }
    }

    /// Multiplies corresponding pairs of packed 8-bit unsigned integer
    /// values contained in the first source operand and packed 8-bit signed
    /// integer values contained in the second source operand, add pairs of
    /// contiguous products with signed saturation, and writes the 16-bit sums to
    /// the corresponding bits in the destination.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_maddubs_epi16)
    #[doc(alias = "_mm_maddubs_epi16")]
    #[inline(always)]
    fn u8x16_mul_saturating_add_adjacent(self, a: u8x16, b: i8x16) -> i16x8 {
        unsafe { i16x8(arch::_mm_maddubs_epi16(a.0, b.0)) }
    }

    /// Multiplies packed 16-bit signed integer values, truncate the 32-bit
    /// product to the 18 most significant bits by right-shifting, round the
    /// truncated value by adding 1, and write bits `[16:1]` to the destination.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mulhrs_epi16)
    #[doc(alias = "_mm_mulhrs_epi16")]
    #[inline(always)]
    fn i16x8_mul_high_round_scale(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_mulhrs_epi16(a.0, b.0)) }
    }

    /// Negates packed 8-bit integers in `a` when the corresponding signed 8-bit
    /// integer in `b` is negative, and returns the result.
    /// Elements in result are zeroed out when the corresponding element in `b`
    /// is zero.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sign_epi8)
    #[doc(alias = "_mm_sign_epi8")]
    #[inline(always)]
    fn i8x16_copy_sign(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_sign_epi8(a.0, b.0)) }
    }

    /// Negates packed 16-bit integers in `a` when the corresponding signed 16-bit
    /// integer in `b` is negative, and returns the results.
    /// Elements in result are zeroed out when the corresponding element in `b`
    /// is zero.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sign_epi16)
    #[doc(alias = "_mm_sign_epi16")]
    #[inline(always)]
    fn i16x8_copy_sign(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_sign_epi16(a.0, b.0)) }
    }

    /// Negates packed 32-bit integers in `a` when the corresponding signed 32-bit
    /// integer in `b` is negative, and returns the results.
    /// Element in result are zeroed out when the corresponding element in `b`
    /// is zero.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sign_epi32)
    #[doc(alias = "_mm_sign_epi32")]
    #[inline(always)]
    fn i32x4_copy_sign(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_sign_epi32(a.0, b.0)) }
    }
}

impl<T: Ssse3Token> Ssse3 for T {}
