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
            pub $(unsafe $($placeholder)?)? fn $func $(<$(const $generic: $generic_ty,)*>)?(self, $($arg: $ty,)*) $(-> $ret)? {
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
macro_rules! __impl_type {
    ("aes") => { $crate::arch::x86::Aes };
    ("pclmulqdq") => { $crate::arch::x86::Pclmulqdq };
    ("rdrand") => { $crate::arch::x86::Rdrand };
    ("rdseed") => { $crate::arch::x86::Rdseed };
    ("tsc") => { $crate::arch::x86::Tsc };
    ("mmx") => { $crate::arch::x86::Mmx };
    ("sse") => { $crate::arch::x86::Sse };
    ("sse2") => { $crate::arch::x86::Sse2 };
    ("sse3") => { $crate::arch::x86::Sse3 };
    ("ssse3") => { $crate::arch::x86::Ssse3 };
    ("sse4.1") => { $crate::arch::x86::Sse4_1 };
    ("sse4.2") => { $crate::arch::x86::Sse4_2 };
    ("sse4a") => { $crate::arch::x86::Sse4a };
    ("sha") => { $crate::arch::x86::Sha };
    ("avx") => { $crate::arch::x86::Avx };
    ("avx2") => { $crate::arch::x86::Avx2 };
    ("avx512f") => { $crate::arch::x86::Avx512f };
    ("avx512cd") => { $crate::arch::x86::Avx512cd };
    ("avx512er") => { $crate::arch::x86::Avx512er };
    ("avx512pf") => { $crate::arch::x86::Avx512pf };
    ("avx512bw") => { $crate::arch::x86::Avx512bw };
    ("avx512dq") => { $crate::arch::x86::Avx512dq };
    ("avx512vl") => { $crate::arch::x86::Avx512vl };
    ("avx512ifma") => { $crate::arch::x86::Avx512ifma };
    ("avx512vbmi") => { $crate::arch::x86::Avx512vbmi };
    ("avx512vpopcntdq") => { $crate::arch::x86::Avx512vpopcntdq };
    ("avx512vbmi2") => { $crate::arch::x86::Avx512vbmi2 };
    ("avx512gfni") => { $crate::arch::x86::Avx512gfni };
    ("avx512vaes") => { $crate::arch::x86::Avx512vaes };
    ("avx512vpclmulqdq") => { $crate::arch::x86::Avx512vpclmulqdq };
    ("avx512vnni") => { $crate::arch::x86::Avx512vnni };
    ("avx512bitalg") => { $crate::arch::x86::Avx512bitalg };
    ("avx512bf16") => { $crate::arch::x86::Avx512bf16 };
    ("avx512vp2intersect") => { $crate::arch::x86::Avx512vp2intersect };
    ("f16c") => { $crate::arch::x86::F16c };
    ("fma") => { $crate::arch::x86::Fma };
    ("bmi1") => { $crate::arch::x86::Bmi1 };
    ("bmi2") => { $crate::arch::x86::Bmi2 };
    ("lzcnt") => { $crate::arch::x86::Lzcnt };
    ("tbm") => { $crate::arch::x86::Tbm };
    ("popcnt") => { $crate::arch::x86::Popcnt };
    ("fxsr") => { $crate::arch::x86::Fxsr };
    ("xsave") => { $crate::arch::x86::Xsave };
    ("xsaveopt") => { $crate::arch::x86::Xsaveopt };
    ("xsaves") => { $crate::arch::x86::Xsaves };
    ("xsavec") => { $crate::arch::x86::Xsavec };
    ("cmpxchg16b") => { $crate::arch::x86::Cmpxchg16b };
    ("adx") => { $crate::arch::x86::Adx };
    ("rtm") => { $crate::arch::x86::Rtm };
    ("abm") => { $crate::arch::x86::Abm };
}

#[macro_export]
macro_rules! simd_type {
    (
        $(
            $(#[$attr: meta])*
            $vis: vis struct $name: ident {
                $($feature_vis: vis $ident: ident: $feature: tt),* $(,)?
            }
        )*
    ) => {
        $(
            #[repr(transparent)]
            #[allow(dead_code)]
            $(#[$attr])*
            #[derive(Clone, Copy, Debug)]
            $vis struct $name{
                $($feature_vis $ident : $crate::__impl_type!($feature),)*
            }

            #[allow(dead_code)]
            $(#[$attr])*
            impl $name {
                #[inline(always)]
                pub unsafe fn new_unchecked() -> Self {
                    Self{
                        $($ident: <$crate::__impl_type!($feature)>::new_unchecked(),)*
                    }
                }

                #[inline(always)]
                pub fn try_new() -> Option<Self> {
                    if Self::is_available() {
                        Some(Self{
                            $($ident: <$crate::__impl_type!($feature)>::new_unchecked(),)*
                        })
                    } else {
                        None
                    }
                }

                #[inline(always)]
                pub fn is_available() -> bool {
                    true $(&& <$crate::__impl_type!($feature)>::is_available())*
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
        )*
    };
}

macro_rules! internal_simd_type {
    (
        $(
            $(#[$attr: meta])*
            $vis: vis struct $name: ident {
                $($feature_vis: vis $ident: ident: $feature: tt),* $(,)?
            }
        )*
    ) => {
        $(
            #[repr(transparent)]
            #[allow(dead_code)]
            $(#[$attr])*
            #[derive(Clone, Copy, Debug)]
            $vis struct $name{
                $($feature_vis $ident : __impl_type!($feature),)*
            }

            #[allow(dead_code)]
            $(#[$attr])*
            impl $name {
                #[inline(always)]
                pub unsafe fn new_unchecked() -> Self {
                    Self{
                        $($ident: <__impl_type!($feature)>::new_unchecked(),)*
                    }
                }

                #[inline(always)]
                pub fn try_new() -> Option<Self> {
                    if Self::is_available() {
                        Some(Self{
                            $($ident: <__impl_type!($feature)>::new_unchecked(),)*
                        })
                    } else {
                        None
                    }
                }

                #[inline(always)]
                pub fn is_available() -> bool {
                    true $(&& <__impl_type!($feature)>::is_available())*
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
        )*
    };
}

pub(crate) use internal_simd_type;

#[cfg(any(doc, target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(docsrs, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
pub mod x86;
