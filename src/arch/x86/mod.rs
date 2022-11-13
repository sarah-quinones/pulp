use super::arch;
use arch::*;
use tokens::*;

pub mod tokens {
    pub unsafe trait SseToken: Copy {}
    pub unsafe trait Sse2Token: Copy {}
    pub unsafe trait Sse3Token: Copy {}
    pub unsafe trait Ssse3Token: Copy {}
    pub unsafe trait Sse41Token: Copy {}
    pub unsafe trait Sse42Token: Copy {}
    pub unsafe trait AvxToken: Copy {}
    pub unsafe trait Avx2Token: Copy {}
    pub unsafe trait FmaToken: Copy {}

    #[cfg(feature = "nightly")]
    pub use super::nightly_tokens::*;
}

#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod nightly_tokens {
    pub unsafe trait Avx512vlToken: Copy {}
    pub unsafe trait Avx512fToken: Copy {}
    pub unsafe trait Avx512bwToken: Copy {}
    pub unsafe trait Avx512cdToken: Copy {}
    pub unsafe trait Avx512ifmaToken: Copy {}
    pub unsafe trait Avx512dqToken: Copy {}
}

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
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod avx512f;
#[cfg(feature = "nightly")]
pub use avx512f::*;

#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod avx512bw;
#[cfg(feature = "nightly")]
pub use avx512bw::*;

#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod avx512cd;
#[cfg(feature = "nightly")]
pub use avx512cd::*;

#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod avx512ifma;
#[cfg(feature = "nightly")]
pub use avx512ifma::*;

#[cfg(feature = "nightly")]
#[cfg_attr(docsrs, doc(cfg(feature = "nightly")))]
mod avx512dq;
#[cfg(feature = "nightly")]
pub use avx512dq::*;
