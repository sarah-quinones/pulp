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

#[cfg(target_arch = "x86")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
mod x86;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
