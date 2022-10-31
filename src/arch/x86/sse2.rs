use super::*;

pub unsafe trait Sse2Token: SseToken {}

pub trait Sse2: Sse2Token {
    /// Constructs a `u8x16` from sixteen `u8` values.
    #[inline(always)]
    fn u8x16_new(
        self,
        e0: u8,
        e1: u8,
        e2: u8,
        e3: u8,
        e4: u8,
        e5: u8,
        e6: u8,
        e7: u8,
        e8: u8,
        e9: u8,
        e10: u8,
        e11: u8,
        e12: u8,
        e13: u8,
        e14: u8,
        e15: u8,
    ) -> u8x16 {
        unsafe {
            u8x16(arch::_mm_setr_epi8(
                e0 as i8, e1 as i8, e2 as i8, e3 as i8, e4 as i8, e5 as i8, e6 as i8, e7 as i8,
                e8 as i8, e9 as i8, e10 as i8, e11 as i8, e12 as i8, e13 as i8, e14 as i8,
                e15 as i8,
            ))
        }
    }

    /// Constructs a `i8x16` from sixteen `i8` values.
    #[inline(always)]
    fn i8x16_new(
        self,
        e0: i8,
        e1: i8,
        e2: i8,
        e3: i8,
        e4: i8,
        e5: i8,
        e6: i8,
        e7: i8,
        e8: i8,
        e9: i8,
        e10: i8,
        e11: i8,
        e12: i8,
        e13: i8,
        e14: i8,
        e15: i8,
    ) -> u8x16 {
        unsafe {
            u8x16(arch::_mm_setr_epi8(
                e0 as i8, e1 as i8, e2 as i8, e3 as i8, e4 as i8, e5 as i8, e6 as i8, e7 as i8,
                e8 as i8, e9 as i8, e10 as i8, e11 as i8, e12 as i8, e13 as i8, e14 as i8,
                e15 as i8,
            ))
        }
    }

    /// Constructs a `u16x8` from eight `u16` values.
    #[inline(always)]
    fn u16x8_new(
        self,
        e0: u16,
        e1: u16,
        e2: u16,
        e3: u16,
        e4: u16,
        e5: u16,
        e6: u16,
        e7: u16,
    ) -> u16x8 {
        unsafe {
            u16x8(arch::_mm_setr_epi16(
                e0 as i16, e1 as i16, e2 as i16, e3 as i16, e4 as i16, e5 as i16, e6 as i16,
                e7 as i16,
            ))
        }
    }

    /// Constructs a `i16x8` from eight `i16` values.
    #[inline(always)]
    fn i16x8_new(
        self,
        e0: i16,
        e1: i16,
        e2: i16,
        e3: i16,
        e4: i16,
        e5: i16,
        e6: i16,
        e7: i16,
    ) -> i16x8 {
        unsafe { i16x8(arch::_mm_setr_epi16(e0, e1, e2, e3, e4, e5, e6, e7)) }
    }

    /// Constructs a `u32x4` from four `u32` values.
    #[inline(always)]
    fn u32x4_new(self, e0: u32, e1: u32, e2: u32, e3: u32) -> u32x4 {
        unsafe {
            u32x4(arch::_mm_setr_epi32(
                e0 as i32, e1 as i32, e2 as i32, e3 as i32,
            ))
        }
    }

    /// Constructs a `i32x4` from four `i32` values.
    #[inline(always)]
    fn i32x4_new(self, e0: i32, e1: i32, e2: i32, e3: i32) -> i32x4 {
        unsafe { i32x4(arch::_mm_setr_epi32(e0, e1, e2, e3)) }
    }

    /// Constructs a `u64x2` from two `u64` values.
    #[inline(always)]
    fn u64x2_new(self, e0: u64, e1: u64) -> u64x2 {
        unsafe { u64x2(transmute([e0, e1])) }
    }

    /// Constructs a `i64x2` from two `i64` values.
    #[inline(always)]
    fn i64x2_new(self, e0: i64, e1: i64) -> i64x2 {
        unsafe { i64x2(transmute([e0, e1])) }
    }

    /// Constructs a `f64x2` from two `f64` values.
    #[inline(always)]
    fn f64x2_new(self, e0: f64, e1: f64) -> f64x2 {
        unsafe { f64x2(arch::_mm_setr_pd(e0, e1)) }
    }

    /// Constructs a `u8x16` with all elements set to `a`.
    #[inline(always)]
    fn u8x16_splat(self, a: u8) -> u8x16 {
        unsafe { u8x16(arch::_mm_set1_epi8(a as i8)) }
    }

    /// Constructs a `i8x16` with all elements set to `a`.
    #[inline(always)]
    fn i8x16_splat(self, a: i8) -> i8x16 {
        unsafe { i8x16(arch::_mm_set1_epi8(a)) }
    }

    /// Constructs a `u16x8` with all elements set to `a`.
    #[inline(always)]
    fn u16x8_splat(self, a: u16) -> u16x8 {
        unsafe { u16x8(arch::_mm_set1_epi16(a as i16)) }
    }

    /// Constructs a `i16x8` with all elements set to `a`.
    #[inline(always)]
    fn i16x8_splat(self, a: i16) -> i16x8 {
        unsafe { i16x8(arch::_mm_set1_epi16(a)) }
    }

    /// Constructs a `u32x4` with all elements set to `a`.
    #[inline(always)]
    fn u32x4_splat(self, a: u32) -> u32x4 {
        unsafe { u32x4(arch::_mm_set1_epi32(a as i32)) }
    }

    /// Constructs a `i32x4` with all elements set to `a`.
    #[inline(always)]
    fn i32x4_splat(self, a: i32) -> i32x4 {
        unsafe { i32x4(arch::_mm_set1_epi32(a)) }
    }

    /// Constructs a `u64x2` with all elements set to `a`.
    #[inline(always)]
    fn u64x2_splat(self, a: u64) -> u64x2 {
        unsafe { u64x2(arch::_mm_set1_epi64x(a as i64)) }
    }

    /// Constructs a `i64x2` with all elements set to `a`.
    #[inline(always)]
    fn i64x2_splat(self, a: i64) -> i64x2 {
        unsafe { i64x2(arch::_mm_set1_epi64x(a)) }
    }

    /// Constructs a `f64x2` with all elements set to `a`.
    #[inline(always)]
    fn f64x2_splat(self, a: f64) -> f64x2 {
        unsafe { f64x2(arch::_mm_set1_pd(a)) }
    }

    /// Loads sixteen `u8` values from memory into a `u8x16`.
    #[inline(always)]
    fn u8x16_load(self, addr: &[u8; 16]) -> u8x16 {
        unsafe {
            u8x16(arch::_mm_loadu_si128(
                addr as *const u8 as *const arch::__m128i,
            ))
        }
    }

    /// Loads sixteen `i8` values from memory into a `i8x16`.
    #[inline(always)]
    fn i8x16_load(self, addr: &[i8; 16]) -> i8x16 {
        unsafe {
            i8x16(arch::_mm_loadu_si128(
                addr as *const i8 as *const arch::__m128i,
            ))
        }
    }

    /// Loads eight `u16` values from memory into a `u16x8`.
    #[inline(always)]
    fn u16x8_load(self, addr: &[u16; 8]) -> u16x8 {
        unsafe {
            u16x8(arch::_mm_loadu_si128(
                addr as *const u16 as *const arch::__m128i,
            ))
        }
    }

    /// Loads eight `i16` values from memory into a `i16x8`.
    #[inline(always)]
    fn i16x8_load(self, addr: &[i16; 8]) -> i16x8 {
        unsafe {
            i16x8(arch::_mm_loadu_si128(
                addr as *const i16 as *const arch::__m128i,
            ))
        }
    }

    /// Loads four `u32` values from memory into a `u32x4`.
    #[inline(always)]
    fn u32x4_load(self, addr: &[u32; 4]) -> u32x4 {
        unsafe {
            u32x4(arch::_mm_loadu_si128(
                addr as *const u32 as *const arch::__m128i,
            ))
        }
    }

    /// Loads four `i32` values from memory into a `i32x4`.
    #[inline(always)]
    fn i32x4_load(self, addr: &[i32; 4]) -> i32x4 {
        unsafe {
            i32x4(arch::_mm_loadu_si128(
                addr as *const i32 as *const arch::__m128i,
            ))
        }
    }

    /// Loads two `u64` values from memory into a `u64x2`.
    #[inline(always)]
    fn u64x2_load(self, addr: &[u64; 2]) -> u64x2 {
        unsafe {
            u64x2(arch::_mm_loadu_si128(
                addr as *const u64 as *const arch::__m128i,
            ))
        }
    }

    /// Loads two `i64` values from memory into a `i64x2`.
    #[inline(always)]
    fn i64x2_load(self, addr: &[i64; 2]) -> i64x2 {
        unsafe {
            i64x2(arch::_mm_loadu_si128(
                addr as *const i64 as *const arch::__m128i,
            ))
        }
    }

    /// Loads two `f64` values from memory into a `f64x2`.
    #[inline(always)]
    fn f64x2_load(self, addr: &[f64; 2]) -> f64x2 {
        unsafe { f64x2(arch::_mm_loadu_pd(addr as *const f64)) }
    }

    /// Stores sixteen `u8` values from a `u8x16` into memory.
    #[inline(always)]
    fn u8x16_store(self, addr: &mut [u8; 16], a: u8x16) {
        unsafe { arch::_mm_storeu_si128(addr as *mut u8 as *mut arch::__m128i, a.0) }
    }

    /// Stores sixteen `i8` values from a `i8x16` into memory.
    #[inline(always)]
    fn i8x16_store(self, addr: &mut [i8; 16], a: i8x16) {
        unsafe { arch::_mm_storeu_si128(addr as *mut i8 as *mut arch::__m128i, a.0) }
    }

    /// Stores eight `u16` values from a `u16x8` into memory.
    #[inline(always)]
    fn u16x8_store(self, addr: &mut [u16; 8], a: u16x8) {
        unsafe { arch::_mm_storeu_si128(addr as *mut u16 as *mut arch::__m128i, a.0) }
    }

    /// Stores eight `i16` values from a `i16x8` into memory.
    #[inline(always)]
    fn i16x8_store(self, addr: &mut [i16; 8], a: i16x8) {
        unsafe { arch::_mm_storeu_si128(addr as *mut i16 as *mut arch::__m128i, a.0) }
    }

    /// Stores four `u32` values from a `u32x4` into memory.
    #[inline(always)]
    fn u32x4_store(self, addr: &mut [u32; 4], a: u32x4) {
        unsafe { arch::_mm_storeu_si128(addr as *mut u32 as *mut arch::__m128i, a.0) }
    }

    /// Stores four `i32` values from a `i32x4` into memory.
    #[inline(always)]
    fn i32x4_store(self, addr: &mut [i32; 4], a: i32x4) {
        unsafe { arch::_mm_storeu_si128(addr as *mut i32 as *mut arch::__m128i, a.0) }
    }

    /// Stores two `u64` values from a `u64x2` into memory.
    #[inline(always)]
    fn u64x2_store(self, addr: &mut [u64; 2], a: u64x2) {
        unsafe { arch::_mm_storeu_si128(addr as *mut u64 as *mut arch::__m128i, a.0) }
    }

    /// Stores two `i64` values from a `i64x2` into memory.
    #[inline(always)]
    fn i64x2_store(self, addr: &mut [i64; 2], a: i64x2) {
        unsafe { arch::_mm_storeu_si128(addr as *mut i64 as *mut arch::__m128i, a.0) }
    }

    /// Stores two `f64` values from a `f64x2` into memory.
    #[inline(always)]
    fn f64x2_store(self, addr: &mut [f64; 2], a: f64x2) {
        unsafe { arch::_mm_storeu_pd(addr as *mut f64, a.0) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi8)
    #[doc(alias = "_mm_add_epi8")]
    #[inline(always)]
    fn u8x16_add(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_add_epi8(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi8)
    #[doc(alias = "_mm_add_epi8")]
    #[inline(always)]
    fn i8x16_add(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_add_epi8(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi16)
    #[doc(alias = "_mm_add_epi16")]
    #[inline(always)]
    fn u16x8_add(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_add_epi16(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi16)
    #[doc(alias = "_mm_add_epi16")]
    #[inline(always)]
    fn i16x8_add(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_add_epi16(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi32)
    #[doc(alias = "_mm_add_epi32")]
    #[inline(always)]
    fn u32x4_add(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_add_epi32(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi32)
    #[doc(alias = "_mm_add_epi32")]
    #[inline(always)]
    fn i32x4_add(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_add_epi32(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi64)
    #[doc(alias = "_mm_add_epi64")]
    #[inline(always)]
    fn u64x2_add(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_add_epi64(a.0, b.0)) }
    }

    /// Computes `a.wrapping_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_epi64)
    #[doc(alias = "_mm_add_epi64")]
    #[inline(always)]
    fn i64x2_add(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_add_epi64(a.0, b.0)) }
    }

    /// Adds packed double-precision (64-bit) floating-point elements in `a` and
    /// `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_add_pd)
    #[doc(alias = "_mm_add_pd")]
    #[inline(always)]
    fn f64x2_add(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_add_pd(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi8)
    #[doc(alias = "_mm_sub_epi8")]
    #[inline(always)]
    fn u8x16_sub(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_sub_epi8(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi8)
    #[doc(alias = "_mm_sub_epi8")]
    #[inline(always)]
    fn i8x16_sub(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_sub_epi8(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi16)
    #[doc(alias = "_mm_sub_epi16")]
    #[inline(always)]
    fn u16x8_sub(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_sub_epi16(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi16)
    #[doc(alias = "_mm_sub_epi16")]
    #[inline(always)]
    fn i16x8_sub(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_sub_epi16(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi32)
    #[doc(alias = "_mm_sub_epi32")]
    #[inline(always)]
    fn u32x4_sub(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_sub_epi32(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi32)
    #[doc(alias = "_mm_sub_epi32")]
    #[inline(always)]
    fn i32x4_sub(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_sub_epi32(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi64)
    #[doc(alias = "_mm_sub_epi64")]
    #[inline(always)]
    fn u64x2_sub(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_sub_epi64(a.0, b.0)) }
    }

    /// Computes `a.wrapping_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_epi64)
    #[doc(alias = "_mm_sub_epi64")]
    #[inline(always)]
    fn i64x2_sub(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_sub_epi64(a.0, b.0)) }
    }

    /// Subtracts packed double-precision (64-bit) floating-point elements in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sub_pd)
    #[doc(alias = "_mm_sub_pd")]
    #[inline(always)]
    fn f64x2_sub(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_sub_pd(a.0, b.0)) }
    }

    /// Computes `a.saturating_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_adds_epu8)
    #[doc(alias = "_mm_adds_epu8")]
    #[inline(always)]
    fn u8x16_saturating_add(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_adds_epu8(a.0, b.0)) }
    }

    /// Computes `a.saturating_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_adds_epi8)
    #[doc(alias = "_mm_adds_epi8")]
    #[inline(always)]
    fn i8x16_saturating_add(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_adds_epi8(a.0, b.0)) }
    }

    /// Computes `a.saturating_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_adds_epu16)
    #[doc(alias = "_mm_adds_epu16")]
    #[inline(always)]
    fn u16x8_saturating_add(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_adds_epu16(a.0, b.0)) }
    }

    /// Computes `a.saturating_add(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_adds_epi16)
    #[doc(alias = "_mm_adds_epi16")]
    #[inline(always)]
    fn i16x8_saturating_add(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_adds_epi16(a.0, b.0)) }
    }

    /// Computes `a.saturating_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_subs_epu8)
    #[doc(alias = "_mm_subs_epu8")]
    #[inline(always)]
    fn u8x16_saturating_sub(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_subs_epu8(a.0, b.0)) }
    }

    /// Computes `a.saturating_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_subs_epi8)
    #[doc(alias = "_mm_subs_epi8")]
    #[inline(always)]
    fn i8x16_saturating_sub(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_subs_epi8(a.0, b.0)) }
    }

    /// Computes `a.saturating_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_subs_epu16)
    #[doc(alias = "_mm_subs_epu16")]
    #[inline(always)]
    fn u16x8_saturating_sub(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_subs_epu16(a.0, b.0)) }
    }

    /// Computes `a.saturating_sub(b)` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_subs_epi16)
    #[doc(alias = "_mm_subs_epi16")]
    #[inline(always)]
    fn i16x8_saturating_sub(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_subs_epi16(a.0, b.0)) }
    }

    /// Computes the average of `a` and `b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_avg_epu8)
    #[doc(alias = "_mm_avg_epu8")]
    #[inline(always)]
    fn u8x16_avg(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_avg_epu8(a.0, b.0)) }
    }

    /// Computes the average of `a` and `b` for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_avg_epu16)
    #[doc(alias = "_mm_avg_epu16")]
    #[inline(always)]
    fn u16x8_avg(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_avg_epu16(a.0, b.0)) }
    }

    /// Multiplies and then horizontally add signed 16 bit integers in `a` and `b`.
    ///
    /// Multiplies packed signed 16-bit integers in `a` and `b`, producing
    /// intermediate signed 32-bit integers. Horizontally add adjacent pairs of
    /// intermediate 32-bit integers.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_madd_epi16)
    #[doc(alias = "_mm_madd_epi16")]
    #[inline(always)]
    fn i16x8_mul_add_adjacent(self, a: i16x8, b: i16x8) -> i32x4 {
        unsafe { i32x4(arch::_mm_madd_epi16(a.0, b.0)) }
    }

    /// Multiplies the packed unsigned 16-bit integers in `a` and `b`.
    ///
    /// The multiplication produces intermediate 32-bit integers, and returns the
    /// high 16 bits of the intermediate integers.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mulhi_epu16)
    #[doc(alias = "_mm_mulhi_epu16")]
    #[inline(always)]
    fn u16x8_mulhi(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_mulhi_epu16(a.0, b.0)) }
    }

    /// Multiplies the packed signed 16-bit integers in `a` and `b`.
    ///
    /// The multiplication produces intermediate 32-bit integers, and returns the
    /// high 16 bits of the intermediate integers.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mulhi_epi16)
    #[doc(alias = "_mm_mulhi_epi16")]
    #[inline(always)]
    fn i16x8_mulhi(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_mulhi_epi16(a.0, b.0)) }
    }

    /// Multiplies the packed unsigned 16-bit integers in `a` and `b`, with wrapping on overflow.
    ///
    /// The multiplication produces intermediate 32-bit integers, and returns the
    /// low 16 bits of the intermediate integers.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mullo_epi16)
    #[doc(alias = "_mm_mullo_epi16")]
    #[inline(always)]
    fn u16x8_mul(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_mullo_epi16(a.0, b.0)) }
    }

    /// Multiplies the packed signed 16-bit integers in `a` and `b`, with wrapping on overflow.
    ///
    /// The multiplication produces intermediate 32-bit integers, and returns the
    /// low 16 bits of the intermediate integers.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mullo_epi16)
    #[doc(alias = "_mm_mullo_epi16")]
    #[inline(always)]
    fn i16x8_mul(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_mullo_epi16(a.0, b.0)) }
    }

    /// Multiplies the low unsigned 32-bit integers from each packed 64-bit element
    /// in `a` and `b`.
    ///
    /// Returns the unsigned 64-bit results.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_epu32)
    #[doc(alias = "_mm_mul_epu32")]
    #[inline(always)]
    fn u64x2_as_u32_widening_mul(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_mul_epu32(a.0, b.0)) }
    }

    /// Multiplies packed double-precision (64-bit) floating-point elements in `a`
    /// and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_mul_pd)
    #[doc(alias = "_mm_mul_pd")]
    #[inline(always)]
    fn f64x2_mul(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_mul_pd(a.0, b.0)) }
    }

    /// Divides packed double-precision (64-bit) floating-point elements in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_div_pd)
    #[doc(alias = "_mm_div_pd")]
    #[inline(always)]
    fn f64x2_div(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_div_pd(a.0, b.0)) }
    }

    /// Sum the absolute differences of packed unsigned 8-bit integers.
    ///
    /// Computes the absolute differences of packed unsigned 8-bit integers in `a`
    /// and `b`, then horizontally sum each consecutive 8 differences to produce
    /// two unsigned 16-bit integers, and pack these unsigned 16-bit integers in
    /// the low 16 bits of 64-bit elements returned.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sad_epu8)
    #[doc(alias = "_mm_sad_epu8")]
    #[inline(always)]
    fn u8x16_abs_diff_accumulate(self, a: u8x16, b: u8x16) -> u64x2 {
        unsafe { u64x2(arch::_mm_sad_epu8(a.0, b.0)) }
    }

    /// Shifts `a` left by `BYTE_COUNT` bytes while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_slli_si128)
    #[doc(alias = "_mm_slli_si128")]
    #[inline(always)]
    fn u8x16_shl_bytes<const BYTE_COUNT: i32>(self, a: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_slli_si128::<BYTE_COUNT>(a.0)) }
    }

    /// Shifts `a` right by `BYTE_COUNT` bytes while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srli_si128)
    #[doc(alias = "_mm_srli_si128")]
    #[inline(always)]
    fn u8x16_shr_bytes<const BYTE_COUNT: i32>(self, a: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_srli_si128::<BYTE_COUNT>(a.0)) }
    }

    /// Shifts packed 16-bit integers in `a` left by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_slli_epi16)
    #[doc(alias = "_mm_slli_epi16")]
    #[inline(always)]
    fn u16x8_shl_const<const COUNT: i32>(self, a: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_slli_epi16::<COUNT>(a.0)) }
    }

    /// Shifts packed 32-bit integers in `a` left by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_slli_epi32)
    #[doc(alias = "_mm_slli_epi32")]
    #[inline(always)]
    fn u32x4_shl_const<const COUNT: i32>(self, a: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_slli_epi32::<COUNT>(a.0)) }
    }

    /// Shifts packed 64-bit integers in `a` left by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_slli_epi64)
    #[doc(alias = "_mm_slli_epi64")]
    #[inline(always)]
    fn u64x2_shl_const<const COUNT: i32>(self, a: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_slli_epi64::<COUNT>(a.0)) }
    }

    /// Shifts packed 16-bit integers in `a` left by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sll_epi16)
    #[doc(alias = "_mm_sll_epi16")]
    #[inline(always)]
    fn u16x8_shl(self, a: u16x8, count: u64x2) -> u16x8 {
        unsafe { u16x8(arch::_mm_sll_epi16(a.0, count.0)) }
    }

    /// Shifts packed 32-bit integers in `a` left by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sll_epi32)
    #[doc(alias = "_mm_sll_epi32")]
    #[inline(always)]
    fn u32x4_shl(self, a: u32x4, count: u64x2) -> u32x4 {
        unsafe { u32x4(arch::_mm_sll_epi32(a.0, count.0)) }
    }

    /// Shifts packed 64-bit integers in `a` left by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sll_epi64)
    #[doc(alias = "_mm_sll_epi64")]
    #[inline(always)]
    fn u64x2_shl(self, a: u64x2, count: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_sll_epi64(a.0, count.0)) }
    }

    /// Shifts packed 16-bit integers in `a` right by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srli_epi16)
    #[doc(alias = "_mm_srli_epi16")]
    #[inline(always)]
    fn u16x8_shr_const<const COUNT: i32>(self, a: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_srli_epi16::<COUNT>(a.0)) }
    }

    /// Shifts packed 32-bit integers in `a` right by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srli_epi32)
    #[doc(alias = "_mm_srli_epi32")]
    #[inline(always)]
    fn u32x4_shr_const<const COUNT: i32>(self, a: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_srli_epi32::<COUNT>(a.0)) }
    }

    /// Shifts packed 64-bit integers in `a` right by `COUNT` while shifting in zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srli_epi64)
    #[doc(alias = "_mm_srli_epi64")]
    #[inline(always)]
    fn u64x2_shr_const<const COUNT: i32>(self, a: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_srli_epi64::<COUNT>(a.0)) }
    }

    /// Shifts packed 16-bit integers in `a` right by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srl_epi16)
    #[doc(alias = "_mm_srl_epi16")]
    #[inline(always)]
    fn u16x8_shr(self, a: u16x8, count: u64x2) -> u16x8 {
        unsafe { u16x8(arch::_mm_srl_epi16(a.0, count.0)) }
    }

    /// Shifts packed 32-bit integers in `a` right by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srl_epi32)
    #[doc(alias = "_mm_srl_epi32")]
    #[inline(always)]
    fn u32x4_shr(self, a: u32x4, count: u64x2) -> u32x4 {
        unsafe { u32x4(arch::_mm_srl_epi32(a.0, count.0)) }
    }

    /// Shifts packed 64-bit integers in `a` right by the first element of `count` while shifting in
    /// zeros.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srl_epi64)
    #[doc(alias = "_mm_srl_epi64")]
    #[inline(always)]
    fn u64x2_shr(self, a: u64x2, count: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_srl_epi64(a.0, count.0)) }
    }

    /// Shifts packed 16-bit integers in `a` right by `COUNT` while shifting in sign bits.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srai_epi16)
    #[doc(alias = "_mm_srai_epi16")]
    #[inline(always)]
    fn i16x8_shr_const<const COUNT: i32>(self, a: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_srai_epi16::<COUNT>(a.0)) }
    }

    /// Shifts packed 16-bit integers in `a` right by the first element of `count` while shifting in
    /// sign bits.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sra_epi16)
    #[doc(alias = "_mm_sra_epi16")]
    #[inline(always)]
    fn i16x8_shr(self, a: i16x8, count: u64x2) -> i16x8 {
        unsafe { i16x8(arch::_mm_sra_epi16(a.0, count.0)) }
    }

    /// Shifts packed 32-bit integers in `a` right by `COUNT` while shifting in sign bits.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_srai_epi32)
    #[doc(alias = "_mm_srai_epi32")]
    #[inline(always)]
    fn i32x4_shr_const<const COUNT: i32>(self, a: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_srai_epi32::<COUNT>(a.0)) }
    }

    /// Shifts packed 32-bit integers in `a` right by the first element of `count` while shifting in
    /// sign bits.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sra_epi32)
    #[doc(alias = "_mm_sra_epi32")]
    #[inline(always)]
    fn i32x4_shr(self, a: i32x4, count: u64x2) -> i32x4 {
        unsafe { i32x4(arch::_mm_sra_epi32(a.0, count.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn u8x16_and(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn u8x16_andnot(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn u8x16_or(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn u8x16_xor(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn i8x16_and(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn i8x16_andnot(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn i8x16_or(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn i8x16_xor(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn u16x8_and(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn u16x8_andnot(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn u16x8_or(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn u16x8_xor(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn i16x8_and(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn i16x8_andnot(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn i16x8_or(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn i16x8_xor(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn u32x4_and(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn u32x4_andnot(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn u32x4_or(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn u32x4_xor(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn i32x4_and(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn i32x4_andnot(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn i32x4_or(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn i32x4_xor(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn u64x2_and(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn u64x2_andnot(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn u64x2_or(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn u64x2_xor(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_si128)
    #[doc(alias = "_mm_and_si128")]
    #[inline(always)]
    fn i64x2_and(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_and_si128(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_si128)
    #[doc(alias = "_mm_andnot_si128")]
    #[inline(always)]
    fn i64x2_andnot(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_andnot_si128(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_si128)
    #[doc(alias = "_mm_or_si128")]
    #[inline(always)]
    fn i64x2_or(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_or_si128(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_si128)
    #[doc(alias = "_mm_xor_si128")]
    #[inline(always)]
    fn i64x2_xor(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_xor_si128(a.0, b.0)) }
    }

    /// Computes the bitwise AND of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_and_pd)
    #[doc(alias = "_mm_and_pd")]
    #[inline(always)]
    fn f64x2_and(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_and_pd(a.0, b.0)) }
    }

    /// Computes the bitwise NOT of 128 bits in `a` and then AND with `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_andnot_pd)
    #[doc(alias = "_mm_andnot_pd")]
    #[inline(always)]
    fn f64x2_andnot(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_andnot_pd(a.0, b.0)) }
    }

    /// Computes the bitwise OR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_or_pd)
    #[doc(alias = "_mm_or_pd")]
    #[inline(always)]
    fn f64x2_or(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_or_pd(a.0, b.0)) }
    }

    /// Computes the bitwise XOR of 128 bits in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_xor_pd)
    #[doc(alias = "_mm_xor_pd")]
    #[inline(always)]
    fn f64x2_xor(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_xor_pd(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise minimum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_epu8)
    #[doc(alias = "_mm_min_epu8")]
    #[inline(always)]
    fn u8x16_min(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_min_epu8(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise maximum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_epu8)
    #[doc(alias = "_mm_max_epu8")]
    #[inline(always)]
    fn u8x16_max(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_max_epu8(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise minimum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_epi16)
    #[doc(alias = "_mm_min_epi16")]
    #[inline(always)]
    fn i16x8_min(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_min_epi16(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise maximum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_epi16)
    #[doc(alias = "_mm_max_epi16")]
    #[inline(always)]
    fn i16x8_max(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_max_epi16(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise minimum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_min_pd)
    #[doc(alias = "_mm_min_pd")]
    #[inline(always)]
    fn f64x2_min(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_min_pd(a.0, b.0)) }
    }

    /// Compares the values in `a` and `b` and computes their element-wise maximum.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_max_pd)
    #[doc(alias = "_mm_max_pd")]
    #[inline(always)]
    fn f64x2_max(self, a: f64x2, b: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_max_pd(a.0, b.0)) }
    }

    /// Returns a new vector with the square root of each of the values in `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_sqrt_pd)
    #[doc(alias = "_mm_sqrt_pd")]
    #[inline(always)]
    fn f64x2_sqrt(self, a: f64x2) -> f64x2 {
        unsafe { f64x2(arch::_mm_sqrt_pd(a.0)) }
    }

    /// Compares `a` and `b` for equality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_epi8)
    #[doc(alias = "_mm_cmpeq_epi8")]
    #[inline(always)]
    fn i8x16_cmp_eq(self, a: i8x16, b: i8x16) -> m8x16 {
        unsafe { m8x16(arch::_mm_cmpeq_epi8(a.0, b.0)) }
    }

    /// Compares `a` and `b` for less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmplt_epi8)
    #[doc(alias = "_mm_cmplt_epi8")]
    #[inline(always)]
    fn i8x16_cmp_lt(self, a: i8x16, b: i8x16) -> m8x16 {
        unsafe { m8x16(arch::_mm_cmplt_epi8(a.0, b.0)) }
    }

    /// Compares `a` and `b` for greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpgt_epi8)
    #[doc(alias = "_mm_cmpgt_epi8")]
    #[inline(always)]
    fn i8x16_cmp_gt(self, a: i8x16, b: i8x16) -> m8x16 {
        unsafe { m8x16(arch::_mm_cmpgt_epi8(a.0, b.0)) }
    }

    /// Compares `a` and `b` for equality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_epi16)
    #[doc(alias = "_mm_cmpeq_epi16")]
    #[inline(always)]
    fn i16x8_cmp_eq(self, a: i16x8, b: i16x8) -> m16x8 {
        unsafe { m16x8(arch::_mm_cmpeq_epi16(a.0, b.0)) }
    }

    /// Compares `a` and `b` for less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmplt_epi16)
    #[doc(alias = "_mm_cmplt_epi16")]
    #[inline(always)]
    fn i16x8_cmp_lt(self, a: i16x8, b: i16x8) -> m32x4 {
        unsafe { m32x4(arch::_mm_cmplt_epi16(a.0, b.0)) }
    }

    /// Compares `a` and `b` for greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpgt_epi16)
    #[doc(alias = "_mm_cmpgt_epi16")]
    #[inline(always)]
    fn i16x8_cmp_gt(self, a: i16x8, b: i16x8) -> m32x4 {
        unsafe { m32x4(arch::_mm_cmpgt_epi16(a.0, b.0)) }
    }

    /// Compares `a` and `b` for equality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_epi32)
    #[doc(alias = "_mm_cmpeq_epi32")]
    #[inline(always)]
    fn i32x4_cmp_eq(self, a: i32x4, b: i32x4) -> m32x4 {
        unsafe { m32x4(arch::_mm_cmpeq_epi32(a.0, b.0)) }
    }

    /// Compares `a` and `b` for less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmplt_epi32)
    #[doc(alias = "_mm_cmplt_epi32")]
    #[inline(always)]
    fn i32x4_cmp_lt(self, a: i32x4, b: i32x4) -> m32x4 {
        unsafe { m32x4(arch::_mm_cmplt_epi32(a.0, b.0)) }
    }

    /// Compares `a` and `b` for greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpgt_epi32)
    #[doc(alias = "_mm_cmpgt_epi32")]
    #[inline(always)]
    fn i32x4_cmp_gt(self, a: i32x4, b: i32x4) -> m32x4 {
        unsafe { m32x4(arch::_mm_cmpgt_epi32(a.0, b.0)) }
    }

    /// Compares `a` and `b` for equality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_pd)
    #[doc(alias = "_mm_cmpeq_pd")]
    #[inline(always)]
    fn f64x2_cmp_eq(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpeq_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmplt_pd)
    #[doc(alias = "_mm_cmplt_pd")]
    #[inline(always)]
    fn f64x2_cmp_lt(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmplt_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for less-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmple_pd)
    #[doc(alias = "_mm_cmple_pd")]
    #[inline(always)]
    fn f64x2_cmp_le(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmple_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpgt_pd)
    #[doc(alias = "_mm_cmpgt_pd")]
    #[inline(always)]
    fn f64x2_cmp_gt(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpgt_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpge_pd)
    #[doc(alias = "_mm_cmpge_pd")]
    #[inline(always)]
    fn f64x2_cmp_ge(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpge_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for inequality, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpeq_pd)
    #[doc(alias = "_mm_cmpeq_pd")]
    #[inline(always)]
    fn f64x2_cmp_neq(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpneq_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not-less-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnlt_pd)
    #[doc(alias = "_mm_cmpnlt_pd")]
    #[inline(always)]
    fn f64x2_cmp_not_lt(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpnlt_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not-less-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnle_pd)
    #[doc(alias = "_mm_cmpnle_pd")]
    #[inline(always)]
    fn f64x2_cmp_not_le(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpnle_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for not greater-than, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpngt_pd)
    #[doc(alias = "_mm_cmpngt_pd")]
    #[inline(always)]
    fn f64x2_cmp_not_gt(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpngt_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for greater-than-or-equal, for each element in `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpnge_pd)
    #[doc(alias = "_mm_cmpnge_pd")]
    #[inline(always)]
    fn f64x2_cmp_not_ge(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpnge_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for the "ordered" relation, for each element in `a` and `b`.
    ///
    /// Two elements are ordered if neither of them is a NaN.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpord_pd)
    #[doc(alias = "_mm_cmpord_pd")]
    #[inline(always)]
    fn f64x2_cmp_ord(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpord_pd(a.0, b.0))) }
    }

    /// Compares `a` and `b` for the "unordered" relation, for each element in `a` and `b`.
    ///
    /// Two elements are unordered if at least one of them is a NaN.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cmpunord_pd)
    #[doc(alias = "_mm_cmpunord_pd")]
    #[inline(always)]
    fn f64x2_cmp_unord(self, a: f64x2, b: f64x2) -> m64x2 {
        unsafe { m64x2(transmute(arch::_mm_cmpunord_pd(a.0, b.0))) }
    }

    /// Converts the lower two packed 32-bit integers in `a` to packed
    /// double-precision (64-bit) floating-point elements.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtepi32_pd)
    #[doc(alias = "_mm_cvtepi32_pd")]
    #[inline(always)]
    fn i32x4_as_f64x2(self, a: i32x4) -> f64x2 {
        unsafe { f64x2(arch::_mm_cvtepi32_pd(a.0)) }
    }

    /// Converts packed 32-bit integers in `a` to packed single-precision (32-bit)
    /// floating-point elements.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtepi32_ps)
    #[doc(alias = "_mm_cvtepi32_ps")]
    #[inline(always)]
    fn i32x4_as_f32x4(self, a: i32x4) -> f32x4 {
        unsafe { f32x4(arch::_mm_cvtepi32_ps(a.0)) }
    }

    /// Converts packed double-precision (64-bit) floating-point elements in `a` to
    /// packed single-precision (32-bit) floating-point elements
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtpd_ps)
    #[doc(alias = "_mm_cvtpd_ps")]
    #[inline(always)]
    fn f64x2_as_f32x4(self, a: f64x2) -> f32x4 {
        unsafe { f32x4(arch::_mm_cvtpd_ps(a.0)) }
    }

    /// Converts packed single-precision (64-bit) floating-point elements in `a` to
    /// packed double-precision (32-bit) floating-point elements
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvtps_pd)
    #[doc(alias = "_mm_cvtps_pd")]
    #[inline(always)]
    fn f32x4_as_f64x2(self, a: f32x4) -> f64x2 {
        unsafe { f64x2(arch::_mm_cvtps_pd(a.0)) }
    }

    /// Converts packed double-precision (64-bit) floating-point elements in `a` to
    /// packed 32-bit integers with truncation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvttpd_epi32)
    #[doc(alias = "_mm_cvttpd_epi32")]
    #[inline(always)]
    fn f64x2_as_i32x4(self, a: f64x2) -> i32x4 {
        unsafe { i32x4(arch::_mm_cvttpd_epi32(a.0)) }
    }

    /// Converts packed single-precision (32-bit) floating-point elements in `a` to
    /// packed 32-bit integers with truncation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_cvttps_epi32)
    #[doc(alias = "_mm_cvttps_epi32")]
    #[inline(always)]
    fn f32x4_as_i32x4(self, a: f32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_cvttps_epi32(a.0)) }
    }

    /// Converts packed 16-bit integers from `a` and `b` to packed 8-bit integers
    /// using signed saturation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_packs_epi16)
    #[doc(alias = "_mm_packs_epi16")]
    #[inline(always)]
    fn i16x8_as_saturated_i8(self, a: i16x8, b: i16x8) -> i8x16 {
        unsafe { i8x16(arch::_mm_packs_epi16(a.0, b.0)) }
    }

    /// Converts packed 16-bit integers from `a` and `b` to packed 8-bit integers
    /// using unsigned saturation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_packus_epi16)
    #[doc(alias = "_mm_packus_epi16")]
    #[inline(always)]
    fn u16x8_as_saturated_u8(self, a: u16x8, b: u16x8) -> u8x16 {
        unsafe { u8x16(arch::_mm_packus_epi16(a.0, b.0)) }
    }

    /// Converts packed 32-bit integers from `a` and `b` to packed 16-bit integers
    /// using signed saturation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_packs_epi32)
    #[doc(alias = "_mm_packs_epi32")]
    #[inline(always)]
    fn i32x4_as_saturated_i16(self, a: i32x4, b: i32x4) -> i16x8 {
        unsafe { i16x8(arch::_mm_packs_epi32(a.0, b.0)) }
    }

    /// Converts packed 32-bit integers from `a` and `b` to packed 16-bit integers
    /// using unsigned saturation.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_packus_epi32)
    #[doc(alias = "_mm_packus_epi32")]
    #[inline(always)]
    fn u32x4_as_saturated_u16(self, a: u32x4, b: u32x4) -> u16x8 {
        unsafe { u16x8(arch::_mm_packus_epi32(a.0, b.0)) }
    }

    /// Returns the `INDEX` element of `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_extract_epi16)
    #[doc(alias = "_mm_extract_epi16")]
    #[inline(always)]
    fn u16x8_extract<const INDEX: i32>(self, a: u16x8) -> u16 {
        unsafe { arch::_mm_extract_epi16::<INDEX>(a.0) as u16 }
    }

    /// Returns the `INDEX` element of `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_extract_epi16)
    #[doc(alias = "_mm_extract_epi16")]
    #[inline(always)]
    fn i16x8_extract<const INDEX: i32>(self, a: i16x8) -> i16 {
        unsafe { arch::_mm_extract_epi16::<INDEX>(a.0) as u16 as i16 }
    }

    /// Returns a new vector where the `INDEX` element of `a` is replaced with `value`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_insert_epi16)
    #[doc(alias = "_mm_insert_epi16")]
    #[inline(always)]
    #[must_use]
    fn u16x8_insert<const INDEX: i32>(self, a: u16x8, value: u16) -> u16x8 {
        unsafe { u16x8(arch::_mm_insert_epi16::<INDEX>(a.0, value as i32)) }
    }

    /// Returns a new vector where the `INDEX` element of `a` is replaced with `value`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_insert_epi16)
    #[doc(alias = "_mm_insert_epi16")]
    #[inline(always)]
    #[must_use]
    fn i16x8_insert<const INDEX: i32>(self, a: i16x8, value: i16) -> i16x8 {
        unsafe { i16x8(arch::_mm_insert_epi16::<INDEX>(a.0, value as u16 as i32)) }
    }

    /// Shuffles 32-bit integers in `a` using the control in `SHUFFLE`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shuffle_epi32)
    #[doc(alias = "_mm_shuffle_epi32")]
    #[inline(always)]
    fn u32x4_shuffle<const SHUFFLE: i32>(self, a: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_shuffle_epi32::<SHUFFLE>(a.0)) }
    }

    /// Shuffles 32-bit integers in `a` using the control in `SHUFFLE`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shuffle_epi32)
    #[doc(alias = "_mm_shuffle_epi32")]
    #[inline(always)]
    fn i32x4_shuffle<const SHUFFLE: i32>(self, a: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_shuffle_epi32::<SHUFFLE>(a.0)) }
    }

    /// Shuffles 16-bit integers in the high 64 bits of `a` using the control in
    /// `SHUFFLE`.
    ///
    /// Put the results in the high 64 bits of the returned vector, with the low 64
    /// bits being copied from from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shufflehi_epi16)
    #[doc(alias = "_mm_shufflehi_epi16")]
    #[inline(always)]
    fn u16x8_shuffle_high<const SHUFFLE: i32>(self, a: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_shufflehi_epi16::<SHUFFLE>(a.0)) }
    }

    /// Shuffles 16-bit integers in the high 64 bits of `a` using the control in
    /// `SHUFFLE`.
    ///
    /// Put the results in the high 64 bits of the returned vector, with the low 64
    /// bits being copied from from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shufflehi_epi16)
    #[doc(alias = "_mm_shufflehi_epi16")]
    #[inline(always)]
    fn i16x8_shuffle_high<const SHUFFLE: i32>(self, a: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_shufflehi_epi16::<SHUFFLE>(a.0)) }
    }

    /// Shuffles 16-bit integers in the low 64 bits of `a` using the control in
    /// `SHUFFLE`.
    ///
    /// Put the results in the low 64 bits of the returned vector, with the high 64
    /// bits being copied from from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shufflehi_epi16)
    #[doc(alias = "_mm_shufflelo_epi16")]
    #[inline(always)]
    fn u16x8_shuffle_low<const SHUFFLE: i32>(self, a: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_shufflelo_epi16::<SHUFFLE>(a.0)) }
    }

    /// Shuffles 16-bit integers in the low 64 bits of `a` using the control in
    /// `SHUFFLE`.
    ///
    /// Put the results in the low 64 bits of the returned vector, with the high 64
    /// bits being copied from from `a`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_shufflehi_epi16)
    #[doc(alias = "_mm_shufflelo_epi16")]
    #[inline(always)]
    fn i16x8_shuffle_low<const SHUFFLE: i32>(self, a: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_shufflelo_epi16::<SHUFFLE>(a.0)) }
    }

    /// Unpacks and interleave 8-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi8)
    #[doc(alias = "_mm_unpackhi_epi8")]
    #[inline(always)]
    fn u8x16_unpack_high(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_unpackhi_epi8(a.0, b.0)) }
    }

    /// Unpacks and interleave 8-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi8)
    #[doc(alias = "_mm_unpacklo_epi8")]
    #[inline(always)]
    fn u8x16_unpack_low(self, a: u8x16, b: u8x16) -> u8x16 {
        unsafe { u8x16(arch::_mm_unpacklo_epi8(a.0, b.0)) }
    }

    /// Unpacks and interleave 16-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi16)
    #[doc(alias = "_mm_unpackhi_epi16")]
    #[inline(always)]
    fn u16x8_unpack_high(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_unpackhi_epi16(a.0, b.0)) }
    }

    /// Unpacks and interleave 16-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi16)
    #[doc(alias = "_mm_unpacklo_epi16")]
    #[inline(always)]
    fn u16x8_unpack_low(self, a: u16x8, b: u16x8) -> u16x8 {
        unsafe { u16x8(arch::_mm_unpacklo_epi16(a.0, b.0)) }
    }

    /// Unpacks and interleave 32-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi32)
    #[doc(alias = "_mm_unpackhi_epi32")]
    #[inline(always)]
    fn u32x4_unpack_high(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_unpackhi_epi32(a.0, b.0)) }
    }

    /// Unpacks and interleave 32-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi32)
    #[doc(alias = "_mm_unpacklo_epi32")]
    #[inline(always)]
    fn u32x4_unpack_low(self, a: u32x4, b: u32x4) -> u32x4 {
        unsafe { u32x4(arch::_mm_unpacklo_epi32(a.0, b.0)) }
    }

    /// Unpacks and interleave 64-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi64)
    #[doc(alias = "_mm_unpackhi_epi64")]
    #[inline(always)]
    fn u64x2_unpack_high(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_unpackhi_epi64(a.0, b.0)) }
    }

    /// Unpacks and interleave 64-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi64)
    #[doc(alias = "_mm_unpacklo_epi64")]
    #[inline(always)]
    fn u64x2_unpack_low(self, a: u64x2, b: u64x2) -> u64x2 {
        unsafe { u64x2(arch::_mm_unpacklo_epi64(a.0, b.0)) }
    }

    /// Unpacks and interleave 8-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi8)
    #[doc(alias = "_mm_unpackhi_epi8")]
    #[inline(always)]
    fn i8x16_unpack_high(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_unpackhi_epi8(a.0, b.0)) }
    }

    /// Unpacks and interleave 8-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi8)
    #[doc(alias = "_mm_unpacklo_epi8")]
    #[inline(always)]
    fn i8x16_unpack_low(self, a: i8x16, b: i8x16) -> i8x16 {
        unsafe { i8x16(arch::_mm_unpacklo_epi8(a.0, b.0)) }
    }

    /// Unpacks and interleave 16-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi16)
    #[doc(alias = "_mm_unpackhi_epi16")]
    #[inline(always)]
    fn i16x8_unpack_high(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_unpackhi_epi16(a.0, b.0)) }
    }

    /// Unpacks and interleave 16-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi16)
    #[doc(alias = "_mm_unpacklo_epi16")]
    #[inline(always)]
    fn i16x8_unpack_low(self, a: i16x8, b: i16x8) -> i16x8 {
        unsafe { i16x8(arch::_mm_unpacklo_epi16(a.0, b.0)) }
    }

    /// Unpacks and interleave 32-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi32)
    #[doc(alias = "_mm_unpackhi_epi32")]
    #[inline(always)]
    fn i32x4_unpack_high(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_unpackhi_epi32(a.0, b.0)) }
    }

    /// Unpacks and interleave 32-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi32)
    #[doc(alias = "_mm_unpacklo_epi32")]
    #[inline(always)]
    fn i32x4_unpack_low(self, a: i32x4, b: i32x4) -> i32x4 {
        unsafe { i32x4(arch::_mm_unpacklo_epi32(a.0, b.0)) }
    }

    /// Unpacks and interleave 64-bit integers from the high half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpackhi_epi64)
    #[doc(alias = "_mm_unpackhi_epi64")]
    #[inline(always)]
    fn i64x2_unpack_high(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_unpackhi_epi64(a.0, b.0)) }
    }

    /// Unpacks and interleave 64-bit integers from the low half of `a` and `b`.
    ///
    /// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_unpacklo_epi64)
    #[doc(alias = "_mm_unpacklo_epi64")]
    #[inline(always)]
    fn i64x2_unpack_low(self, a: i64x2, b: i64x2) -> i64x2 {
        unsafe { i64x2(arch::_mm_unpacklo_epi64(a.0, b.0)) }
    }
}

impl<T: Sse2Token> Sse2 for T {}
