use super::arch;

#[cfg(feature = "nightly")]
pub use arch::__m512;
#[cfg(feature = "nightly")]
pub use arch::__m512d;
#[cfg(feature = "nightly")]
pub use arch::__m512i;

pub use arch::{__m128, __m128d, __m128i, __m256, __m256d, __m256i};

mod sse;
pub use sse::*;

mod sse2;
pub use sse2::*;

mod sse3;
pub use sse3::*;

mod ssse3;
pub use ssse3::*;

mod sse41;
pub use sse41::*;

mod sse42;
pub use sse42::*;

mod avx;
pub use avx::*;

mod avx2;
pub use avx2::*;

mod fma;
pub use fma::*;
