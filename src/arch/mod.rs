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

#[cfg(target_arch = "x86_64")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __impl_simd {
    ($name: ident, "sse") => { unsafe impl $crate::arch::x86_64::SseToken for $name {} };
    ($name: ident, "sse2") => { unsafe impl $crate::arch::x86_64::Sse2Token for $name {} };
    ($name: ident, "sse3") => { unsafe impl $crate::arch::x86_64::Sse3Token for $name {} };
    ($name: ident, "ssse3") => { unsafe impl $crate::arch::x86_64::Ssse3Token for $name {} };
    ($name: ident, "sse4.1") => { unsafe impl $crate::arch::x86_64::Sse41Token for $name {} };
    ($name: ident, "sse4.2") => { unsafe impl $crate::arch::x86_64::Sse42Token for $name{} };
    ($name: ident, "avx") => { unsafe impl $crate::arch::x86_64::AvxToken for $name {} };
    ($name: ident, "avx2") => { unsafe impl $crate::arch::x86_64::Avx2Token for $name {} };
    ($name: ident, "fma") => { unsafe impl $crate::arch::x86_64::FmaToken for $name {} };
}

#[cfg(target_arch = "x86")]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __impl_simd {
    ($name: ident, "sse") => { unsafe impl $crate::arch::x86::SseToken for $name {} };
    ($name: ident, "sse2") => { unsafe impl $crate::arch::x86::Sse2Token for $name {} };
    ($name: ident, "sse3") => { unsafe impl $crate::arch::x86::Sse3Token for $name {} };
    ($name: ident, "ssse3") => { unsafe impl $crate::arch::x86::Ssse3Token for $name {} };
    ($name: ident, "sse4.1") => { unsafe impl $crate::arch::x86::Sse41Token for $name {} };
    ($name: ident, "sse4.2") => { unsafe impl $crate::arch::x86::Sse42Token for $name{} };
    ($name: ident, "avx") => { unsafe impl $crate::arch::x86::AvxToken for $name {} };
    ($name: ident, "avx2") => { unsafe impl $crate::arch::x86::Avx2Token for $name {} };
    ($name: ident, "fma") => { unsafe impl $crate::arch::x86::FmaToken for $name {} };
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

#[cfg(target_arch = "x86")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
mod x86;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
