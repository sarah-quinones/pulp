use super::arch;
use bytemuck::{NoUninit, Pod, Zeroable};
use core::fmt::Debug;
use core::mem::transmute;

#[derive(Copy, Clone)]
pub struct m8x16(arch::__m128i);
#[derive(Copy, Clone)]
pub struct m16x8(arch::__m128i);
#[derive(Copy, Clone)]
pub struct m32x4(arch::__m128i);
#[derive(Copy, Clone)]
pub struct m64x2(arch::__m128i);
#[derive(Copy, Clone)]
pub struct u8x16(arch::__m128i);
#[derive(Copy, Clone)]
pub struct u16x8(arch::__m128i);
#[derive(Copy, Clone)]
pub struct u32x4(arch::__m128i);
#[derive(Copy, Clone)]
pub struct u64x2(arch::__m128i);
#[derive(Copy, Clone)]
pub struct i8x16(arch::__m128i);
#[derive(Copy, Clone)]
pub struct i16x8(arch::__m128i);
#[derive(Copy, Clone)]
pub struct i32x4(arch::__m128i);
#[derive(Copy, Clone)]
pub struct i64x2(arch::__m128i);
#[derive(Copy, Clone)]
pub struct f32x4(arch::__m128);
#[derive(Copy, Clone)]
pub struct f64x2(arch::__m128d);

#[derive(Copy, Clone)]
pub struct m8x32(arch::__m256i);
#[derive(Copy, Clone)]
pub struct m16x16(arch::__m256i);
#[derive(Copy, Clone)]
pub struct m32x8(arch::__m256i);
#[derive(Copy, Clone)]
pub struct m64x4(arch::__m256i);
#[derive(Copy, Clone)]
pub struct u8x32(arch::__m256i);
#[derive(Copy, Clone)]
pub struct u16x16(arch::__m256i);
#[derive(Copy, Clone)]
pub struct u32x8(arch::__m256i);
#[derive(Copy, Clone)]
pub struct u64x4(arch::__m256i);
#[derive(Copy, Clone)]
pub struct i8x32(arch::__m256i);
#[derive(Copy, Clone)]
pub struct i16x16(arch::__m256i);
#[derive(Copy, Clone)]
pub struct i32x8(arch::__m256i);
#[derive(Copy, Clone)]
pub struct i64x4(arch::__m256i);
#[derive(Copy, Clone)]
pub struct f32x8(arch::__m256);
#[derive(Copy, Clone)]
pub struct f64x4(arch::__m256d);

#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct u8x64(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct u16x32(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct u32x16(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct u64x8(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct i8x64(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct i16x32(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct i32x16(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct i64x8(arch::__m512i);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct f32x16(arch::__m512);
#[cfg(feature = "nightly")]
#[derive(Copy, Clone)]
pub struct f64x8(arch::__m512d);

unsafe impl Zeroable for u8x16 {}
unsafe impl Zeroable for u8x32 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for u8x64 {}

unsafe impl Zeroable for u16x8 {}
unsafe impl Zeroable for u16x16 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for u16x32 {}

unsafe impl Zeroable for u32x4 {}
unsafe impl Zeroable for u32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for u32x16 {}

unsafe impl Zeroable for u64x2 {}
unsafe impl Zeroable for u64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for u64x8 {}

unsafe impl Zeroable for i8x16 {}
unsafe impl Zeroable for i8x32 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for i8x64 {}

unsafe impl Zeroable for i16x8 {}
unsafe impl Zeroable for i16x16 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for i16x32 {}

unsafe impl Zeroable for i32x4 {}
unsafe impl Zeroable for i32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for i32x16 {}

unsafe impl Zeroable for i64x2 {}
unsafe impl Zeroable for i64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for i64x8 {}

unsafe impl Zeroable for f32x4 {}
unsafe impl Zeroable for f32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for f32x16 {}

unsafe impl Zeroable for f64x2 {}
unsafe impl Zeroable for f64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Zeroable for f64x8 {}

unsafe impl Pod for u8x16 {}
unsafe impl Pod for u8x32 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for u8x64 {}

unsafe impl Pod for u16x8 {}
unsafe impl Pod for u16x16 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for u16x32 {}

unsafe impl Pod for u32x4 {}
unsafe impl Pod for u32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for u32x16 {}

unsafe impl Pod for u64x2 {}
unsafe impl Pod for u64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for u64x8 {}

unsafe impl Pod for i8x16 {}
unsafe impl Pod for i8x32 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for i8x64 {}

unsafe impl Pod for i16x8 {}
unsafe impl Pod for i16x16 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for i16x32 {}

unsafe impl Pod for i32x4 {}
unsafe impl Pod for i32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for i32x16 {}

unsafe impl Pod for i64x2 {}
unsafe impl Pod for i64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for i64x8 {}

unsafe impl Pod for f32x4 {}
unsafe impl Pod for f32x8 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for f32x16 {}

unsafe impl Pod for f64x2 {}
unsafe impl Pod for f64x4 {}
#[cfg(feature = "nightly")]
unsafe impl Pod for f64x8 {}

unsafe impl NoUninit for m8x16 {}
unsafe impl NoUninit for m8x32 {}

unsafe impl NoUninit for m16x8 {}
unsafe impl NoUninit for m16x16 {}

unsafe impl NoUninit for m32x4 {}
unsafe impl NoUninit for m32x8 {}

unsafe impl NoUninit for m64x2 {}
unsafe impl NoUninit for m64x4 {}

impl Debug for u8x16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [u8; 16] = unsafe { transmute(*self) };
        f.debug_tuple("u8x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}
impl Debug for u8x32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [u8; 32] = unsafe { transmute(*self) };
        f.debug_tuple("u8x32")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for u8x64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [u8; 64] = unsafe { transmute(*self) };
        f.debug_tuple("u8x64")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .field(&this[0x20])
            .field(&this[0x21])
            .field(&this[0x22])
            .field(&this[0x23])
            .field(&this[0x24])
            .field(&this[0x25])
            .field(&this[0x26])
            .field(&this[0x27])
            .field(&this[0x28])
            .field(&this[0x29])
            .field(&this[0x2a])
            .field(&this[0x2b])
            .field(&this[0x2c])
            .field(&this[0x2d])
            .field(&this[0x2e])
            .field(&this[0x2f])
            .field(&this[0x30])
            .field(&this[0x31])
            .field(&this[0x32])
            .field(&this[0x33])
            .field(&this[0x34])
            .field(&this[0x35])
            .field(&this[0x36])
            .field(&this[0x37])
            .field(&this[0x38])
            .field(&this[0x39])
            .field(&this[0x3a])
            .field(&this[0x3b])
            .field(&this[0x3c])
            .field(&this[0x3d])
            .field(&this[0x3e])
            .field(&this[0x3f])
            .finish()
    }
}

impl Debug for i8x16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [i8; 16] = unsafe { transmute(*self) };
        f.debug_tuple("i8x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}
impl Debug for i8x32 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [i8; 32] = unsafe { transmute(*self) };
        f.debug_tuple("i8x32")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for i8x64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let this: [i8; 64] = unsafe { transmute(*self) };
        f.debug_tuple("i8x64")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .field(&this[0x20])
            .field(&this[0x21])
            .field(&this[0x22])
            .field(&this[0x23])
            .field(&this[0x24])
            .field(&this[0x25])
            .field(&this[0x26])
            .field(&this[0x27])
            .field(&this[0x28])
            .field(&this[0x29])
            .field(&this[0x2a])
            .field(&this[0x2b])
            .field(&this[0x2c])
            .field(&this[0x2d])
            .field(&this[0x2e])
            .field(&this[0x2f])
            .field(&this[0x30])
            .field(&this[0x31])
            .field(&this[0x32])
            .field(&this[0x33])
            .field(&this[0x34])
            .field(&this[0x35])
            .field(&this[0x36])
            .field(&this[0x37])
            .field(&this[0x38])
            .field(&this[0x39])
            .field(&this[0x3a])
            .field(&this[0x3b])
            .field(&this[0x3c])
            .field(&this[0x3d])
            .field(&this[0x3e])
            .field(&this[0x3f])
            .finish()
    }
}

impl Debug for u16x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u16; 8] = unsafe { transmute(*self) };
        f.debug_tuple("u16x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}
impl Debug for u16x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u16; 16] = unsafe { transmute(*self) };
        f.debug_tuple("u16x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for u16x32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u16; 32] = unsafe { transmute(*self) };
        f.debug_tuple("u16x32")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .finish()
    }
}

impl Debug for i16x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i16; 8] = unsafe { transmute(*self) };
        f.debug_tuple("i16x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}
impl Debug for i16x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i16; 16] = unsafe { transmute(*self) };
        f.debug_tuple("i16x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for i16x32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i16; 32] = unsafe { transmute(*self) };
        f.debug_tuple("i16x32")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .field(&this[0x10])
            .field(&this[0x11])
            .field(&this[0x12])
            .field(&this[0x13])
            .field(&this[0x14])
            .field(&this[0x15])
            .field(&this[0x16])
            .field(&this[0x17])
            .field(&this[0x18])
            .field(&this[0x19])
            .field(&this[0x1a])
            .field(&this[0x1b])
            .field(&this[0x1c])
            .field(&this[0x1d])
            .field(&this[0x1e])
            .field(&this[0x1f])
            .finish()
    }
}

impl Debug for u32x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u32; 4] = unsafe { transmute(*self) };
        f.debug_tuple("u32x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
impl Debug for u32x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u32; 8] = unsafe { transmute(*self) };
        f.debug_tuple("u32x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for u32x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u32; 16] = unsafe { transmute(*self) };
        f.debug_tuple("u32x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}

impl Debug for i32x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i32; 4] = unsafe { transmute(*self) };
        f.debug_tuple("i32x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
impl Debug for i32x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i32; 8] = unsafe { transmute(*self) };
        f.debug_tuple("i32x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for i32x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i32; 16] = unsafe { transmute(*self) };
        f.debug_tuple("i32x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}

impl Debug for u64x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u64; 2] = unsafe { transmute(*self) };
        f.debug_tuple("u64x2")
            .field(&this[0x0])
            .field(&this[0x1])
            .finish()
    }
}
impl Debug for u64x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u64; 4] = unsafe { transmute(*self) };
        f.debug_tuple("u64x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for u64x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [u64; 8] = unsafe { transmute(*self) };
        f.debug_tuple("u64x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}

impl Debug for i64x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i64; 2] = unsafe { transmute(*self) };
        f.debug_tuple("i64x2")
            .field(&this[0x0])
            .field(&this[0x1])
            .finish()
    }
}
impl Debug for i64x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i64; 4] = unsafe { transmute(*self) };
        f.debug_tuple("i64x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for i64x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [i64; 8] = unsafe { transmute(*self) };
        f.debug_tuple("i64x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}

impl Debug for f32x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f32; 4] = unsafe { transmute(*self) };
        f.debug_tuple("f32x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
impl Debug for f32x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f32; 8] = unsafe { transmute(*self) };
        f.debug_tuple("f32x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for f32x16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f32; 16] = unsafe { transmute(*self) };
        f.debug_tuple("f32x16")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .field(&this[0x8])
            .field(&this[0x9])
            .field(&this[0xa])
            .field(&this[0xb])
            .field(&this[0xc])
            .field(&this[0xd])
            .field(&this[0xe])
            .field(&this[0xf])
            .finish()
    }
}

impl Debug for f64x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f64; 2] = unsafe { transmute(*self) };
        f.debug_tuple("f64x2")
            .field(&this[0x0])
            .field(&this[0x1])
            .finish()
    }
}
impl Debug for f64x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f64; 4] = unsafe { transmute(*self) };
        f.debug_tuple("f64x4")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .finish()
    }
}
#[cfg(feature = "nightly")]
impl Debug for f64x8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this: [f64; 8] = unsafe { transmute(*self) };
        f.debug_tuple("f64x8")
            .field(&this[0x0])
            .field(&this[0x1])
            .field(&this[0x2])
            .field(&this[0x3])
            .field(&this[0x4])
            .field(&this[0x5])
            .field(&this[0x6])
            .field(&this[0x7])
            .finish()
    }
}

mod sse;
pub use sse::*;

mod sse2;
pub use sse2::*;

mod sse3;
pub use sse3::*;

mod ssse3;
pub use ssse3::*;
