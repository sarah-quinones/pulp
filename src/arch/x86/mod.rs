use super::arch;

#[cfg(feature = "nightly")]
pub use arch::{__m512, __m512d, __m512i, __mmask16, __mmask32, __mmask64, __mmask8};

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

#[cfg(feature = "nightly")]
pub unsafe trait Avx512vlToken: Copy {}

#[cfg(feature = "nightly")]
mod avx512f;
#[cfg(feature = "nightly")]
pub use avx512f::*;

#[cfg(feature = "nightly")]
mod avx512bw;
#[cfg(feature = "nightly")]
pub use avx512bw::*;

#[cfg(feature = "nightly")]
mod avx512cd;
#[cfg(feature = "nightly")]
pub use avx512cd::*;

#[cfg(feature = "nightly")]
mod avx512ifma;
#[cfg(feature = "nightly")]
pub use avx512ifma::*;

#[cfg(feature = "nightly")]
mod avx512dq;
#[cfg(feature = "nightly")]
pub use avx512dq::*;
