use super::*;

impl F16c {
	delegate!({
		fn _mm_cvtph_ps(a: __m128i) -> __m128;
		fn _mm256_cvtph_ps(a: __m128i) -> __m256;
		fn _mm_cvtps_ph<const IMM8: i32>(a: __m128) -> __m128i;
		fn _mm256_cvtps_ph<const IMM8: i32>(a: __m256) -> __m128i;
	});
}
