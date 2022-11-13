mod arch {
    #[cfg(target_arch = "aarch64")]
    pub use core::arch::aarch64::*;
    #[cfg(target_arch = "arm")]
    pub use core::arch::arm::*;
    #[cfg(target_arch = "mips")]
    pub use core::arch::mips::*;
    #[cfg(target_arch = "mips64")]
    pub use core::arch::mips64::*;
    #[cfg(target_arch = "nvptx")]
    pub use core::arch::nvptx::*;
    #[cfg(target_arch = "powerpc")]
    pub use core::arch::powerpc::*;
    #[cfg(target_arch = "powerpc64")]
    pub use core::arch::powerpc64::*;
    #[cfg(target_arch = "riscv32")]
    pub use core::arch::riscv32::*;
    #[cfg(target_arch = "riscv64")]
    pub use core::arch::riscv64::*;
    #[cfg(target_arch = "wasm32")]
    pub use core::arch::wasm32::*;
    #[cfg(target_arch = "wasm64")]
    pub use core::arch::wasm64::*;
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::*;
}

#[allow(unused_macros)]
macro_rules! delegate {
    ($(
        $(#[$attr: meta])*
        $(unsafe $($placeholder: lifetime)?)?
        fn $func: ident $(<$(const $generic: ident: $generic_ty: ty),* $(,)?>)?(
            $($arg: ident: $ty: ty),* $(,)?
        ) $(-> $ret: ty)?;
    )*) => {
        $(
            $(#[$attr])*
            #[inline(always)]
            $(unsafe $($placeholder)?)? fn $func $(<$(const $generic: $generic_ty,)*>)?(self, $($arg: $ty,)*) $(-> $ret)? {
                unsafe { arch::$func $(::<$($generic,)*>)?($($arg,)*) }
            }
        )*
    };
}

#[cfg(all(feature = "std", any(target_arch = "x86", target_arch = "x86_64")))]
#[macro_export]
macro_rules! feature_detected {
    ($feature: tt) => {
        ::std::is_x86_feature_detected!($feature)
    };
}

#[cfg(all(feature = "std", target_arch = "aarch64"))]
#[macro_export]
macro_rules! feature_detected {
    ($feature: tt) => {
        ::std::is_aarch64_feature_detected!($feature)
    };
}

#[cfg(any(
    not(feature = "std"),
    not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))
))]
#[macro_export]
macro_rules! feature_detected {
    ($feature: tt) => {
        cfg!($feature)
    };
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __impl_simd {
    ($name: ident, "aes") => { unsafe impl $crate::arch::x86::tokens::AesToken for $name {} };
    ($name: ident, "pclmulqdq") => { unsafe impl $crate::arch::x86::tokens::PclmulqdqToken for $name {} };
    ($name: ident, "rdrand") => { unsafe impl $crate::arch::x86::tokens::RdrandToken for $name {} };
    ($name: ident, "rdseed") => { unsafe impl $crate::arch::x86::tokens::RdseedToken for $name {} };
    ($name: ident, "tsc") => { unsafe impl $crate::arch::x86::tokens::TscToken for $name {} };
    ($name: ident, "mmx") => { unsafe impl $crate::arch::x86::tokens::MmxToken for $name {} };
    ($name: ident, "sse") => { unsafe impl $crate::arch::x86::tokens::SseToken for $name {} };
    ($name: ident, "sse2") => { unsafe impl $crate::arch::x86::tokens::Sse2Token for $name {} };
    ($name: ident, "sse3") => { unsafe impl $crate::arch::x86::tokens::Sse3Token for $name {} };
    ($name: ident, "ssse3") => { unsafe impl $crate::arch::x86::tokens::Ssse3Token for $name {} };
    ($name: ident, "sse4.1") => { unsafe impl $crate::arch::x86::tokens::Sse4_1Token for $name {} };
    ($name: ident, "sse4.2") => { unsafe impl $crate::arch::x86::tokens::Sse4_2Token for $name {} };
    ($name: ident, "sse4a") => { unsafe impl $crate::arch::x86::tokens::Sse4aToken for $name {} };
    ($name: ident, "sha") => { unsafe impl $crate::arch::x86::tokens::ShaToken for $name {} };
    ($name: ident, "avx") => { unsafe impl $crate::arch::x86::tokens::AvxToken for $name {} };
    ($name: ident, "avx2") => { unsafe impl $crate::arch::x86::tokens::Avx2Token for $name {} };
    ($name: ident, "avx512f") => { unsafe impl $crate::arch::x86::tokens::Avx512fToken for $name {} };
    ($name: ident, "avx512cd") => { unsafe impl $crate::arch::x86::tokens::Avx512cdToken for $name {} };
    ($name: ident, "avx512er") => { unsafe impl $crate::arch::x86::tokens::Avx512erToken for $name {} };
    ($name: ident, "avx512pf") => { unsafe impl $crate::arch::x86::tokens::Avx512pfToken for $name {} };
    ($name: ident, "avx512bw") => { unsafe impl $crate::arch::x86::tokens::Avx512bwToken for $name {} };
    ($name: ident, "avx512dq") => { unsafe impl $crate::arch::x86::tokens::Avx512dqToken for $name {} };
    ($name: ident, "avx512vl") => { unsafe impl $crate::arch::x86::tokens::Avx512vlToken for $name {} };
    ($name: ident, "avx512ifma") => { unsafe impl $crate::arch::x86::tokens::Avx512ifmaToken for $name {} };
    ($name: ident, "avx512vbmi") => { unsafe impl $crate::arch::x86::tokens::Avx512vbmiToken for $name {} };
    ($name: ident, "avx512vpopcntdq") => { unsafe impl $crate::arch::x86::tokens::Avx512vpopcntdqToken for $name {} };
    ($name: ident, "avx512vbmi2") => { unsafe impl $crate::arch::x86::tokens::Avx512vbmi2Token for $name {} };
    ($name: ident, "avx512gfni") => { unsafe impl $crate::arch::x86::tokens::Avx512gfniToken for $name {} };
    ($name: ident, "avx512vaes") => { unsafe impl $crate::arch::x86::tokens::Avx512vaesToken for $name {} };
    ($name: ident, "avx512vpclmulqdq") => { unsafe impl $crate::arch::x86::tokens::Avx512vpclmulqdqToken for $name {} };
    ($name: ident, "avx512vnni") => { unsafe impl $crate::arch::x86::tokens::Avx512vnniToken for $name {} };
    ($name: ident, "avx512bitalg") => { unsafe impl $crate::arch::x86::tokens::Avx512bitalgToken for $name {} };
    ($name: ident, "avx512bf16") => { unsafe impl $crate::arch::x86::tokens::Avx512bf16Token for $name {} };
    ($name: ident, "avx512vp2intersect") => { unsafe impl $crate::arch::x86::tokens::Avx512vp2intersectToken for $name {} };
    ($name: ident, "f16c") => { unsafe impl $crate::arch::x86::tokens::F16cToken for $name {} };
    ($name: ident, "fma") => { unsafe impl $crate::arch::x86::tokens::FmaToken for $name {} };
    ($name: ident, "bmi1") => { unsafe impl $crate::arch::x86::tokens::Bmi1Token for $name {} };
    ($name: ident, "bmi2") => { unsafe impl $crate::arch::x86::tokens::Bmi2Token for $name {} };
    ($name: ident, "lzcnt") => { unsafe impl $crate::arch::x86::tokens::LzcntToken for $name {} };
    ($name: ident, "tbm") => { unsafe impl $crate::arch::x86::tokens::TbmToken for $name {} };
    ($name: ident, "popcnt") => { unsafe impl $crate::arch::x86::tokens::PopcntToken for $name {} };
    ($name: ident, "fxsr") => { unsafe impl $crate::arch::x86::tokens::FxsrToken for $name {} };
    ($name: ident, "xsave") => { unsafe impl $crate::arch::x86::tokens::XsaveToken for $name {} };
    ($name: ident, "xsaveopt") => { unsafe impl $crate::arch::x86::tokens::XsaveoptToken for $name {} };
    ($name: ident, "xsaves") => { unsafe impl $crate::arch::x86::tokens::XsavesToken for $name {} };
    ($name: ident, "xsavec") => { unsafe impl $crate::arch::x86::tokens::XsavecToken for $name {} };
    ($name: ident, "cmpxchg16b") => { unsafe impl $crate::arch::x86::tokens::Cmpxchg16bToken for $name {} };
    ($name: ident, "adx") => { unsafe impl $crate::arch::x86::tokens::AdxToken for $name {} };
    ($name: ident, "rtm") => { unsafe impl $crate::arch::x86::tokens::RtmToken for $name {} };
    ($name: ident, "abm") => { unsafe impl $crate::arch::x86::tokens::AbmToken for $name {} };
}

#[macro_export]
macro_rules! simd_type {
    ($vis: vis $name: ident, $($feature: tt),* $(,)?) => {
        #[repr(transparent)]
        $vis struct $name {
            #[doc(hidden)]
            __private_do_not_touch: (),
        }

        impl ::core::clone::Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl ::core::marker::Copy for $name {}
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        #[allow(dead_code)]
        impl $name {
            #[inline(always)]
            pub fn try_new() -> ::core::option::Option<Self> {
                if true $(&& $crate::feature_detected!($feature))* {
                    ::core::option::Option::Some(
                        Self {
                            __private_do_not_touch: ()
                        }
                    )
                } else {
                    ::core::option::Option::None
                }
            }

            #[inline(always)]
            pub unsafe fn new_unchecked() -> Self {
                Self {
                    __private_do_not_touch: (),
                }
            }

            #[inline(always)]
            pub fn vectorize<R>(self, f: impl FnOnce() -> R) -> R {
                $(#[target_feature(enable = $feature)])*
                unsafe fn vectorize<R>(f: impl FnOnce() -> R) -> R {
                    f()
                }
                unsafe { vectorize(f) }
            }
        }

        $($crate::__impl_simd!($name, $feature);)*
    };
}

macro_rules! internal_simd_type {
    ($vis: vis $name: ident, $($feature: tt),* $(,)?) => {
        #[repr(transparent)]
        $vis struct $name {
            #[doc(hidden)]
            __private_do_not_touch: (),
        }

        impl ::core::clone::Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl ::core::marker::Copy for $name {}
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(stringify!($name))
            }
        }

        #[allow(dead_code)]
        impl $name {
            #[inline(always)]
            pub fn try_new() -> ::core::option::Option<Self> {
                if true $(&& $crate::feature_detected!($feature))* {
                    ::core::option::Option::Some(
                        Self {
                            __private_do_not_touch: ()
                        }
                    )
                } else {
                    ::core::option::Option::None
                }
            }

            #[inline(always)]
            pub unsafe fn new_unchecked() -> Self {
                Self {
                    __private_do_not_touch: (),
                }
            }

            #[inline(always)]
            pub fn vectorize<R>(self, f: impl FnOnce() -> R) -> R {
                $(#[target_feature(enable = $feature)])*
                unsafe fn vectorize<R>(f: impl FnOnce() -> R) -> R {
                    f()
                }
                unsafe { vectorize(f) }
            }
        }

        $(__impl_simd!($name, $feature);)*
    };
}

pub(crate) use internal_simd_type;

#[cfg(any(doc, target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(docsrs, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
pub mod x86;
