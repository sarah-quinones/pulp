use super::arch;
use arch::*;
use tokens::*;

pub mod tokens {
    pub unsafe trait AesToken: Copy {}
    pub unsafe trait PclmulqdqToken: Copy {}
    pub unsafe trait RdrandToken: Copy {}
    pub unsafe trait RdseedToken: Copy {}
    pub unsafe trait TscToken: Copy {}
    pub unsafe trait MmxToken: Copy {}
    pub unsafe trait SseToken: Copy {}
    pub unsafe trait Sse2Token: Copy {}
    pub unsafe trait Sse3Token: Copy {}
    pub unsafe trait Ssse3Token: Copy {}
    pub unsafe trait Sse4_1Token: Copy {}
    pub unsafe trait Sse4_2Token: Copy {}
    pub unsafe trait Sse4aToken: Copy {}
    pub unsafe trait ShaToken: Copy {}
    pub unsafe trait AvxToken: Copy {}
    pub unsafe trait Avx2Token: Copy {}
    pub unsafe trait Avx512fToken: Copy {}
    pub unsafe trait Avx512cdToken: Copy {}
    pub unsafe trait Avx512erToken: Copy {}
    pub unsafe trait Avx512pfToken: Copy {}
    pub unsafe trait Avx512bwToken: Copy {}
    pub unsafe trait Avx512dqToken: Copy {}
    pub unsafe trait Avx512vlToken: Copy {}
    pub unsafe trait Avx512ifmaToken: Copy {}
    pub unsafe trait Avx512vbmiToken: Copy {}
    pub unsafe trait Avx512vpopcntdqToken: Copy {}
    pub unsafe trait Avx512vbmi2Token: Copy {}
    pub unsafe trait Avx512gfniToken: Copy {}
    pub unsafe trait Avx512vaesToken: Copy {}
    pub unsafe trait Avx512vpclmulqdqToken: Copy {}
    pub unsafe trait Avx512vnniToken: Copy {}
    pub unsafe trait Avx512bitalgToken: Copy {}
    pub unsafe trait Avx512bf16Token: Copy {}
    pub unsafe trait Avx512vp2intersectToken: Copy {}
    pub unsafe trait F16cToken: Copy {}
    pub unsafe trait FmaToken: Copy {}
    pub unsafe trait Bmi1Token: Copy {}
    pub unsafe trait Bmi2Token: Copy {}
    pub unsafe trait LzcntToken: Copy {}
    pub unsafe trait TbmToken: Copy {}
    pub unsafe trait PopcntToken: Copy {}
    pub unsafe trait FxsrToken: Copy {}
    pub unsafe trait XsaveToken: Copy {}
    pub unsafe trait XsaveoptToken: Copy {}
    pub unsafe trait XsavesToken: Copy {}
    pub unsafe trait XsavecToken: Copy {}
    pub unsafe trait Cmpxchg16bToken: Copy {}
    pub unsafe trait AdxToken: Copy {}
    pub unsafe trait RtmToken: Copy {}
    pub unsafe trait AbmToken: Copy {}
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
