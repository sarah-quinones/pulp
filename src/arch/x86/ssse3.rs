use super::*;

pub unsafe trait Ssse3Token: Sse3Token {}

pub trait Ssse3: Ssse3Token {
    delegate! {
        fn _mm_abs_epi8(a: __m128i) -> __m128i;
        fn _mm_abs_epi16(a: __m128i) -> __m128i;
        fn _mm_abs_epi32(a: __m128i) -> __m128i;
        fn _mm_shuffle_epi8(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_alignr_epi8<const IMM8: i32>(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hadd_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hadds_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hadd_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hsub_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hsubs_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_hsub_epi32(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_maddubs_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_mulhrs_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_sign_epi8(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_sign_epi16(a: __m128i, b: __m128i) -> __m128i;
        fn _mm_sign_epi32(a: __m128i, b: __m128i) -> __m128i;
    }
}

impl<T: Ssse3Token> Ssse3 for T {}
