use super::*;

pub unsafe trait Avx512ifmaToken: Copy {}

pub trait Avx512ifma: Avx512ifmaToken {
    delegate! {
    fn _mm512_madd52hi_epu64(a: __m512i, b: __m512i, c: __m512i) -> __m512i;
    fn _mm512_madd52lo_epu64(a: __m512i, b: __m512i, c: __m512i) -> __m512i;
    }
}
pub trait Avx512ifma_Avx512vl: Avx512ifmaToken + Avx512vlToken {
    delegate!{
    fn _mm256_madd52hi_epu64(a: __m256i, b: __m256i, c: __m256i) -> __m256i;
    fn _mm256_madd52lo_epu64(a: __m256i, b: __m256i, c: __m256i) -> __m256i;
    fn _mm_madd52hi_epu64(a: __m128i, b: __m128i, c: __m128i) -> __m128i;
    fn _mm_madd52lo_epu64(a: __m128i, b: __m128i, c: __m128i) -> __m128i;
    }
}

impl<T: Avx512ifmaToken> Avx512ifma for T {}
impl<T: Avx512ifmaToken + Avx512vlToken> Avx512ifma_Avx512vl for T {}
