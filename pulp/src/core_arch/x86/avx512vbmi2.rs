use super::*;

impl Avx512vbmi2 {
	delegate!({
		fn _mm512_mask_compress_epi16(src: __m512i, k: __mmask32, a: __m512i) -> __m512i;
		fn _mm512_maskz_compress_epi16(k: __mmask32, a: __m512i) -> __m512i;
		fn _mm512_mask_compress_epi8(src: __m512i, k: __mmask64, a: __m512i) -> __m512i;
		fn _mm512_maskz_compress_epi8(k: __mmask64, a: __m512i) -> __m512i;
		fn _mm512_mask_expand_epi16(src: __m512i, k: __mmask32, a: __m512i) -> __m512i;
		fn _mm512_maskz_expand_epi16(k: __mmask32, a: __m512i) -> __m512i;
		fn _mm512_mask_expand_epi8(src: __m512i, k: __mmask64, a: __m512i) -> __m512i;
		fn _mm512_maskz_expand_epi8(k: __mmask64, a: __m512i) -> __m512i;
	});
}
