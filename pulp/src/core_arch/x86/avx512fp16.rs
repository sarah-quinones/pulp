use super::*;

// Only the vector-in/vector-out ops are delegated here (`__m512h`/`__m256h`/`__m512`,
// opaque SIMD types, stable since Rust 1.94).
impl Avx512fp16 {
	delegate!({
		fn _mm512_add_ph(a: __m512h, b: __m512h) -> __m512h;
		fn _mm512_sub_ph(a: __m512h, b: __m512h) -> __m512h;
		fn _mm512_mul_ph(a: __m512h, b: __m512h) -> __m512h;
		fn _mm512_fmadd_ph(a: __m512h, b: __m512h, c: __m512h) -> __m512h;
		fn _mm512_cvtxps_ph(a: __m512) -> __m256h;
		fn _mm512_cvtxph_ps(a: __m256h) -> __m512;
	});
}
