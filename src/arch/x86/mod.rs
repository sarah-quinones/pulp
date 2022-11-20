use super::arch;
use arch::*;

macro_rules! __impl {
    ($name: ident, $feature: tt) => {
        impl $name {
            #[inline(always)]
            pub fn new_unchecked() -> Self {
                Self { __private: () }
            }
            #[inline(always)]
            pub fn try_new() -> Option<Self> {
                if feature_detected!($feature) {
                    Some(Self { __private: () })
                } else {
                    None
                }
            }

            #[inline(always)]
            pub fn is_available() -> bool {
                feature_detected!($feature)
            }
        }

        impl ::core::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(stringify!($name))
            }
        }
    };
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Aes {
    __private: (),
}
__impl!(Aes, "aes");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Pclmulqdq {
    __private: (),
}
__impl!(Pclmulqdq, "pclmulqdq");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Rdrand {
    __private: (),
}
__impl!(Rdrand, "rdrand");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Rdseed {
    __private: (),
}
__impl!(Rdseed, "rdseed");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Tsc {
    __private: (),
}
__impl!(Tsc, "tsc");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Mmx {
    __private: (),
}
__impl!(Mmx, "mmx");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse {
    __private: (),
}
__impl!(Sse, "sse");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse2 {
    __private: (),
}
__impl!(Sse2, "sse2");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse3 {
    __private: (),
}
__impl!(Sse3, "sse3");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Ssse3 {
    __private: (),
}
__impl!(Ssse3, "ssse3");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse4_1 {
    __private: (),
}
__impl!(Sse4_1, "sse4.1");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse4_2 {
    __private: (),
}
__impl!(Sse4_2, "sse4.2");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sse4a {
    __private: (),
}
__impl!(Sse4a, "sse4a");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Sha {
    __private: (),
}
__impl!(Sha, "sha");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx {
    __private: (),
}
__impl!(Avx, "avx");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx2 {
    __private: (),
}
__impl!(Avx2, "avx2");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512f {
    __private: (),
}
__impl!(Avx512f, "avx512f");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512cd {
    __private: (),
}
__impl!(Avx512cd, "avx512cd");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512er {
    __private: (),
}
__impl!(Avx512er, "avx512er");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512pf {
    __private: (),
}
__impl!(Avx512pf, "avx512pf");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512bw {
    __private: (),
}
__impl!(Avx512bw, "avx512bw");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512dq {
    __private: (),
}
__impl!(Avx512dq, "avx512dq");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vl {
    __private: (),
}
__impl!(Avx512vl, "avx512vl");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512ifma {
    __private: (),
}
__impl!(Avx512ifma, "avx512ifma");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vbmi {
    __private: (),
}
__impl!(Avx512vbmi, "avx512vbmi");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vpopcntdq {
    __private: (),
}
__impl!(Avx512vpopcntdq, "avx512vpopcntdq");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vbmi2 {
    __private: (),
}
__impl!(Avx512vbmi2, "avx512vbmi2");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512gfni {
    __private: (),
}
__impl!(Avx512gfni, "avx512gfni");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vaes {
    __private: (),
}
__impl!(Avx512vaes, "avx512vaes");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vpclmulqdq {
    __private: (),
}
__impl!(Avx512vpclmulqdq, "avx512vpclmulqdq");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vnni {
    __private: (),
}
__impl!(Avx512vnni, "avx512vnni");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512bitalg {
    __private: (),
}
__impl!(Avx512bitalg, "avx512bitalg");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512bf16 {
    __private: (),
}
__impl!(Avx512bf16, "avx512bf16");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Avx512vp2intersect {
    __private: (),
}
__impl!(Avx512vp2intersect, "avx512vp2intersect");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct F16c {
    __private: (),
}
__impl!(F16c, "f16c");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Fma {
    __private: (),
}
__impl!(Fma, "fma");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bmi1 {
    __private: (),
}
__impl!(Bmi1, "bmi1");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Bmi2 {
    __private: (),
}
__impl!(Bmi2, "bmi2");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Lzcnt {
    __private: (),
}
__impl!(Lzcnt, "lzcnt");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Tbm {
    __private: (),
}
__impl!(Tbm, "tbm");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Popcnt {
    __private: (),
}
__impl!(Popcnt, "popcnt");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Fxsr {
    __private: (),
}
__impl!(Fxsr, "fxsr");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Xsave {
    __private: (),
}
__impl!(Xsave, "xsave");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Xsaveopt {
    __private: (),
}
__impl!(Xsaveopt, "xsaveopt");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Xsaves {
    __private: (),
}
__impl!(Xsaves, "xsaves");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Xsavec {
    __private: (),
}
__impl!(Xsavec, "xsavec");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Cmpxchg16b {
    __private: (),
}
__impl!(Cmpxchg16b, "cmpxchg16b");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Adx {
    __private: (),
}
__impl!(Adx, "adx");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Rtm {
    __private: (),
}
__impl!(Rtm, "rtm");
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Abm {
    __private: (),
}
__impl!(Abm, "abm");

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Avx512f_Avx512vl {
    pub avx512f: Avx512f,
    pub avx512vl: Avx512vl,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Avx512bw_Avx512vl {
    pub avx512bw: Avx512bw,
    pub avx512vl: Avx512vl,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Avx512cd_Avx512vl {
    pub avx512cd: Avx512cd,
    pub avx512vl: Avx512vl,
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Avx512ifma_Avx512vl {
    pub avx512ifma: Avx512ifma,
    pub avx512vl: Avx512vl,
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
