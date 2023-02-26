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
        cfg!(target_arch = $feature)
    };
}

#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
#[doc(hidden)]
#[rustfmt::skip]
#[macro_export]
macro_rules! __impl_type {
    ("aes") => { $crate::core_arch::x86::Aes };
    ("pclmulqdq") => { $crate::core_arch::x86::Pclmulqdq };
    ("rdrand") => { $crate::core_arch::x86::Rdrand };
    ("rdseed") => { $crate::core_arch::x86::Rdseed };
    ("tsc") => { $crate::core_arch::x86::Tsc };
    ("mmx") => { $crate::core_arch::x86::Mmx };
    ("sse") => { $crate::core_arch::x86::Sse };
    ("sse2") => { $crate::core_arch::x86::Sse2 };
    ("sse3") => { $crate::core_arch::x86::Sse3 };
    ("ssse3") => { $crate::core_arch::x86::Ssse3 };
    ("sse4.1") => { $crate::core_arch::x86::Sse4_1 };
    ("sse4.2") => { $crate::core_arch::x86::Sse4_2 };
    ("sse4a") => { $crate::core_arch::x86::Sse4a };
    ("sha") => { $crate::core_arch::x86::Sha };
    ("avx") => { $crate::core_arch::x86::Avx };
    ("avx2") => { $crate::core_arch::x86::Avx2 };
    ("gfni") => { $crate::core_arch::x86::Gfni };
    ("vaes") => { $crate::core_arch::x86::Vaes };
    ("vpclmulqdq") => { $crate::core_arch::x86::Vpclmulqdq };
    ("avx512f") => { $crate::core_arch::x86::Avx512f };
    ("avx512cd") => { $crate::core_arch::x86::Avx512cd };
    ("avx512er") => { $crate::core_arch::x86::Avx512er };
    ("avx512pf") => { $crate::core_arch::x86::Avx512pf };
    ("avx512bw") => { $crate::core_arch::x86::Avx512bw };
    ("avx512dq") => { $crate::core_arch::x86::Avx512dq };
    ("avx512vl") => { $crate::core_arch::x86::Avx512vl };
    ("avx512ifma") => { $crate::core_arch::x86::Avx512ifma };
    ("avx512vbmi") => { $crate::core_arch::x86::Avx512vbmi };
    ("avx512vpopcntdq") => { $crate::core_arch::x86::Avx512vpopcntdq };
    ("avx512vbmi2") => { $crate::core_arch::x86::Avx512vbmi2 };
    ("avx512gfni") => { $crate::core_arch::x86::Avx512gfni };
    ("avx512vaes") => { $crate::core_arch::x86::Avx512vaes };
    ("avx512vpclmulqdq") => { $crate::core_arch::x86::Avx512vpclmulqdq };
    ("avx512vnni") => { $crate::core_arch::x86::Avx512vnni };
    ("avx512bitalg") => { $crate::core_arch::x86::Avx512bitalg };
    ("avx512bf16") => { $crate::core_arch::x86::Avx512bf16 };
    ("avx512vp2intersect") => { $crate::core_arch::x86::Avx512vp2intersect };
    ("f16c") => { $crate::core_arch::x86::F16c };
    ("fma") => { $crate::core_arch::x86::Fma };
    ("bmi1") => { $crate::core_arch::x86::Bmi1 };
    ("bmi2") => { $crate::core_arch::x86::Bmi2 };
    ("lzcnt") => { $crate::core_arch::x86::Lzcnt };
    ("tbm") => { $crate::core_arch::x86::Tbm };
    ("popcnt") => { $crate::core_arch::x86::Popcnt };
    ("fxsr") => { $crate::core_arch::x86::Fxsr };
    ("xsave") => { $crate::core_arch::x86::Xsave };
    ("xsaveopt") => { $crate::core_arch::x86::Xsaveopt };
    ("xsaves") => { $crate::core_arch::x86::Xsaves };
    ("xsavec") => { $crate::core_arch::x86::Xsavec };
    ("cmpxchg16b") => { $crate::core_arch::x86::Cmpxchg16b };
    ("adx") => { $crate::core_arch::x86::Adx };
    ("rtm") => { $crate::core_arch::x86::Rtm };
    ("abm") => { $crate::core_arch::x86::Abm };
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
            #[allow(dead_code)]
            $(#[$attr])*
            #[derive(Clone, Copy, Debug)]
            $vis struct $name{
                $($feature_vis $ident : $crate::__impl_type!($feature),)*
            }

            #[allow(dead_code)]
            $(#[$attr])*
            impl $name {
                /// Returns a SIMD token type without checking if the required CPU features for
                /// this type are available.
                ///
                /// # Safety
                /// - the required CPU features must be available.
                #[inline]
                pub unsafe fn new_unchecked() -> Self {
                    Self{
                        $($ident: <$crate::__impl_type!($feature)>::new_unchecked(),)*
                    }
                }

                /// Returns a SIMD token type if the required CPU features for this type are
                /// available, otherwise returns `None`.
                #[inline]
                pub fn try_new() -> Option<Self> {
                    if Self::is_available() {
                        Some(Self{
                            $($ident: <$crate::__impl_type!($feature)>::new_unchecked(),)*
                        })
                    } else {
                        None
                    }
                }

                /// Returns `true` if the required CPU features for this type are available,
                /// otherwise returns `false`.
                #[inline]
                pub fn is_available() -> bool {
                    true $(&& <$crate::__impl_type!($feature)>::is_available())*
                }

                /// Vectorizes the given function as if the CPU features for this type were applied
                /// to it.
                ///
                /// # Note
                /// For the vectorization to work properly, the given function must be inlined.
                /// Consider marking it as `#[inline(always)]`
                #[inline(always)]
                pub fn vectorize<F: $crate::NullaryFnOnce>(self, f: F) -> F::Output {
                    $(#[target_feature(enable = $feature)])*
                    #[inline]
                    unsafe fn __impl<F: $crate::NullaryFnOnce>(f: F) -> F::Output {
                        f.call()
                    }
                    unsafe { __impl(f) }
                }

                /// Takes a proof of the existence of this SIMD token (`self`), and returns a
                /// persistent reference to it.
                #[inline]
                pub fn to_ref(self) -> &'static Self {
                    const __ASSERT_ZST: () = {
                        assert!(::core::mem::size_of::<$name>() == 0);
                    };
                    unsafe { &*::core::ptr::NonNull::dangling().as_ptr() }
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
                /// Returns a SIMD token type without checking if the required CPU features for
                /// this type are available.
                ///
                /// # Safety
                /// - the required CPU features must be available.
                #[inline]
                pub unsafe fn new_unchecked() -> Self {
                    Self{
                        $($ident: <__impl_type!($feature)>::new_unchecked(),)*
                    }
                }

                /// Returns a SIMD token type if the required CPU features for this type are
                /// available, otherwise returns `None`.
                #[inline]
                pub fn try_new() -> Option<Self> {
                    if Self::is_available() {
                        Some(Self{
                            $($ident: <__impl_type!($feature)>::new_unchecked(),)*
                        })
                    } else {
                        None
                    }
                }

                /// Returns `true` if the required CPU features for this type are available,
                /// otherwise returns `false`.
                #[inline]
                pub fn is_available() -> bool {
                    true $(&& <__impl_type!($feature)>::is_available())*
                }

                /// Vectorizes the given function as if the CPU features for this type were applied
                /// to it.
                ///
                /// # Note
                /// For the vectorization to work properly, the given function must be inlined.
                /// Consider marking it as `#[inline(always)]`
                #[inline(always)]
                pub fn vectorize<F: $crate::NullaryFnOnce>(self, f: F) -> F::Output {
                    $(#[target_feature(enable = $feature)])*
                    #[inline]
                    unsafe fn __impl<F: $crate::NullaryFnOnce>(f: F) -> F::Output {
                        f.call()
                    }
                    unsafe { __impl(f) }
                }

                /// Takes a proof of the existence of this SIMD token (`self`), and returns a
                /// persistent reference to it.
                #[inline]
                pub fn to_ref(self) -> &'static Self {
                    const __ASSERT_ZST: () = {
                        assert!(::core::mem::size_of::<$name>() == 0);
                    };
                    unsafe { &*::core::ptr::NonNull::dangling().as_ptr() }
                }
            }
        )*
    };
}

pub(crate) use internal_simd_type;

#[cfg(any(doc, target_arch = "x86", target_arch = "x86_64"))]
#[cfg_attr(docsrs, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
pub mod x86;
