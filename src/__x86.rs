use super::*;
use crate::cast;
use crate::core_arch::internal_simd_type;
use core::fmt::Debug;
use core::mem::transmute;

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m8(u8);
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m16(u16);
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m32(u32);
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct m64(u64);

impl Debug for m8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m16 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m32 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl Debug for m64 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}

impl m8 {
    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u8::MAX
    }
}
impl m16 {
    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u16::MAX
    }
}
impl m32 {
    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u32::MAX
    }
}
impl m64 {
    #[inline]
    pub fn is_set(self) -> bool {
        self.0 == u64::MAX
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f32x4(f32, f32, f32, f32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f32x8(f32, f32, f32, f32, f32, f32, f32, f32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub struct f32x16(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32, f32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i32x4(i32, i32, i32, i32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i32x8(i32, i32, i32, i32, i32, i32, i32, i32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub struct i32x16(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u32x4(u32, u32, u32, u32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u32x8(u32, u32, u32, u32, u32, u32, u32, u32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub struct u32x16(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct m32x4(m32, m32, m32, m32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct m32x8(m32, m32, m32, m32, m32, m32, m32, m32);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f64x2(f64, f64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f64x4(f64, f64, f64, f64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f64x8(f64, f64, f64, f64, f64, f64, f64, f64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i64x2(i64, i64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i64x4(i64, i64, i64, i64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i64x8(i64, i64, i64, i64, i64, i64, i64, i64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u64x2(u64, u64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u64x4(u64, u64, u64, u64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u64x8(u64, u64, u64, u64, u64, u64, u64, u64);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct m64x2(m64, m64);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct m64x4(m64, m64, m64, m64);

unsafe impl Zeroable for f32x4 {}
unsafe impl Zeroable for f32x8 {}
unsafe impl Zeroable for f32x16 {}
unsafe impl Pod for f32x4 {}
unsafe impl Pod for f32x8 {}
unsafe impl Pod for f32x16 {}
unsafe impl Zeroable for i32x4 {}
unsafe impl Zeroable for i32x8 {}
unsafe impl Zeroable for i32x16 {}
unsafe impl Pod for i32x4 {}
unsafe impl Pod for i32x8 {}
unsafe impl Pod for i32x16 {}
unsafe impl Zeroable for u32x4 {}
unsafe impl Zeroable for u32x8 {}
unsafe impl Zeroable for u32x16 {}
unsafe impl Pod for u32x4 {}
unsafe impl Pod for u32x8 {}
unsafe impl Pod for u32x16 {}

unsafe impl Zeroable for f64x2 {}
unsafe impl Zeroable for f64x4 {}
unsafe impl Zeroable for f64x8 {}
unsafe impl Pod for f64x2 {}
unsafe impl Pod for f64x4 {}
unsafe impl Pod for f64x8 {}
unsafe impl Zeroable for i64x2 {}
unsafe impl Zeroable for i64x4 {}
unsafe impl Zeroable for i64x8 {}
unsafe impl Pod for i64x2 {}
unsafe impl Pod for i64x4 {}
unsafe impl Pod for i64x8 {}
unsafe impl Zeroable for u64x2 {}
unsafe impl Zeroable for u64x4 {}
unsafe impl Zeroable for u64x8 {}
unsafe impl Pod for u64x2 {}
unsafe impl Pod for u64x4 {}
unsafe impl Pod for u64x8 {}

unsafe impl NoUninit for m32x4 {}
unsafe impl NoUninit for m32x8 {}
unsafe impl NoUninit for m64x2 {}
unsafe impl NoUninit for m64x4 {}

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

impl f32x4 {
    #[inline]
    fn as_vec(self) -> __m128 {
        unsafe { transmute(self) }
    }
}

impl f64x2 {
    #[inline]
    fn as_vec(self) -> __m128d {
        unsafe { transmute(self) }
    }
}

impl f32x8 {
    #[inline]
    fn as_vec(self) -> __m256 {
        unsafe { transmute(self) }
    }
}

impl f64x4 {
    #[inline]
    fn as_vec(self) -> __m256d {
        unsafe { transmute(self) }
    }
}

#[cfg(feature = "nightly")]
impl f32x16 {
    #[inline]
    fn as_vec(self) -> __m512 {
        unsafe { transmute(self) }
    }
}

#[cfg(feature = "nightly")]
impl f64x8 {
    #[inline]
    fn as_vec(self) -> __m512d {
        unsafe { transmute(self) }
    }
}

// https://en.wikipedia.org/wiki/X86-64#Microarchitecture_levels
internal_simd_type! {
    pub struct V2 {
        sse: "sse",
        sse2: "sse2",
        fxsr: "fxsr",
        sse3: "sse3",
        ssse3: "ssse3",
        sse4_1: "sse4.1",
        sse4_2: "sse4.2",
        popcnt: "popcnt",
    }

    pub struct V3 {
        sse: "sse",
        sse2: "sse2",
        fxsr: "fxsr",
        sse3: "sse3",
        ssse3: "ssse3",
        sse4_1: "sse4.1",
        sse4_2: "sse4.2",
        popcnt: "popcnt",
        avx: "avx",
        axv2: "avx2",
        bmi1: "bmi1",
        bmi2: "bmi2",
        fma: "fma",
        lzcnt: "lzcnt",
    }

    #[cfg(feature = "nightly")]
    pub struct V4 {
        sse: "sse",
        sse2: "sse2",
        fxsr: "fxsr",
        sse3: "sse3",
        ssse3: "ssse3",
        sse4_1: "sse4.1",
        sse4_2: "sse4.2",
        popcnt: "popcnt",
        avx: "avx",
        axv2: "avx2",
        bmi1: "bmi1",
        bmi2: "bmi2",
        fma: "fma",
        lzcnt: "lzcnt",
        avx512f: "avx512f",
        avx512bw: "avx512bw",
        avx512cd: "avx512cd",
        avx512dq: "avx512dq",
        avx512vl: "avx512vl",
    }
}

// x86-32 wants to use a 32-bit address size, but asm! defaults to using the full
// register name (e.g. rax). We have to explicitly override the placeholder to
// use the 32-bit register name in that case.

#[cfg(feature = "nightly")]
#[cfg(target_pointer_width = "32")]
macro_rules! vpl {
    ($inst:expr) => {
        concat!($inst, ", [{p:e}]")
    };
}
#[cfg(feature = "nightly")]
#[cfg(target_pointer_width = "64")]
macro_rules! vpl {
    ($inst:expr) => {
        concat!($inst, ", [{p}]")
    };
}
#[cfg(feature = "nightly")]
#[cfg(target_pointer_width = "32")]
macro_rules! vps {
    ($inst1:expr, $inst2:expr) => {
        concat!($inst1, " [{p:e}]", $inst2)
    };
}
#[cfg(feature = "nightly")]
#[cfg(target_pointer_width = "64")]
macro_rules! vps {
    ($inst1:expr, $inst2:expr) => {
        concat!($inst1, " [{p}]", $inst2)
    };
}

// this is copied from the standard library with added flags from V4.
// if the full flags are not provided, the function doesn't get inlined properly
#[cfg(feature = "nightly")]
#[inline]
#[target_feature(enable = "sse")]
#[target_feature(enable = "sse2")]
#[target_feature(enable = "fxsr")]
#[target_feature(enable = "sse3")]
#[target_feature(enable = "ssse3")]
#[target_feature(enable = "sse4.1")]
#[target_feature(enable = "sse4.2")]
#[target_feature(enable = "popcnt")]
#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
#[target_feature(enable = "bmi1")]
#[target_feature(enable = "bmi2")]
#[target_feature(enable = "fma")]
#[target_feature(enable = "lzcnt")]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512cd")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
pub unsafe fn _mm512_maskz_loadu_epi32(k: __mmask16, mem_addr: *const i32) -> __m512i {
    let mut dst: __m512i;
    core::arch::asm!(
        vpl!("vmovdqu32 {dst}{{{k}}} {{z}}"),
        p = in(reg) mem_addr,
        k = in(kreg) k,
        dst = out(zmm_reg) dst,
        options(pure, readonly, nostack)
    );
    dst
}

// see above comment
#[cfg(feature = "nightly")]
#[inline]
#[target_feature(enable = "sse")]
#[target_feature(enable = "sse2")]
#[target_feature(enable = "fxsr")]
#[target_feature(enable = "sse3")]
#[target_feature(enable = "ssse3")]
#[target_feature(enable = "sse4.1")]
#[target_feature(enable = "sse4.2")]
#[target_feature(enable = "popcnt")]
#[target_feature(enable = "avx")]
#[target_feature(enable = "avx2")]
#[target_feature(enable = "bmi1")]
#[target_feature(enable = "bmi2")]
#[target_feature(enable = "fma")]
#[target_feature(enable = "lzcnt")]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[target_feature(enable = "avx512cd")]
#[target_feature(enable = "avx512dq")]
#[target_feature(enable = "avx512vl")]
pub unsafe fn _mm512_mask_storeu_epi32(mem_addr: *mut i32, mask: __mmask16, a: __m512i) {
    core::arch::asm!(
        vps!("vmovdqu32", "{{{mask}}}, {a}"),
        p = in(reg) mem_addr,
        mask = in(kreg) mask,
        a = in(zmm_reg) a,
        options(nostack)
    );
}

impl Seal for V2 {}
impl Seal for V3 {}
#[cfg(feature = "nightly")]
impl Seal for V4 {}

#[cfg(feature = "nightly")]
impl V4 {
    #[target_feature(enable = "avx512f")]
    #[inline]
    unsafe fn fmsubadd_ps(a: __m512, b: __m512, c: __m512) -> __m512 {
        _mm512_fmaddsub_ps(a, b, _mm512_sub_ps(_mm512_set1_ps(-0.0), c))
    }
    #[target_feature(enable = "avx512f")]
    #[inline]
    unsafe fn fmsubadd_pd(a: __m512d, b: __m512d, c: __m512d) -> __m512d {
        _mm512_fmaddsub_pd(a, b, _mm512_sub_pd(_mm512_set1_pd(-0.0), c))
    }
}

static V3_U32_MASKS: [u32x8; 9] = [
    u32x8(0, 0, 0, 0, 0, 0, 0, 0),
    u32x8(!0, 0, 0, 0, 0, 0, 0, 0),
    u32x8(!0, !0, 0, 0, 0, 0, 0, 0),
    u32x8(!0, !0, !0, 0, 0, 0, 0, 0),
    u32x8(!0, !0, !0, !0, 0, 0, 0, 0),
    u32x8(!0, !0, !0, !0, !0, 0, 0, 0),
    u32x8(!0, !0, !0, !0, !0, !0, 0, 0),
    u32x8(!0, !0, !0, !0, !0, !0, !0, 0),
    u32x8(!0, !0, !0, !0, !0, !0, !0, !0),
];

static V3_U32_LAST_MASKS: [u32x8; 9] = [
    u32x8(0, 0, 0, 0, 0, 0, 0, 0),
    u32x8(0, 0, 0, 0, 0, 0, 0, !0),
    u32x8(0, 0, 0, 0, 0, 0, !0, !0),
    u32x8(0, 0, 0, 0, 0, !0, !0, !0),
    u32x8(0, 0, 0, 0, !0, !0, !0, !0),
    u32x8(0, 0, 0, !0, !0, !0, !0, !0),
    u32x8(0, 0, !0, !0, !0, !0, !0, !0),
    u32x8(0, !0, !0, !0, !0, !0, !0, !0),
    u32x8(!0, !0, !0, !0, !0, !0, !0, !0),
];

#[cfg(feature = "nightly")]
static V4_U32_MASKS: [u16; 17] = [
    0b0000000000000000,
    0b0000000000000001,
    0b0000000000000011,
    0b0000000000000111,
    0b0000000000001111,
    0b0000000000011111,
    0b0000000000111111,
    0b0000000001111111,
    0b0000000011111111,
    0b0000000111111111,
    0b0000001111111111,
    0b0000011111111111,
    0b0000111111111111,
    0b0001111111111111,
    0b0011111111111111,
    0b0111111111111111,
    0b1111111111111111,
];

#[cfg(feature = "nightly")]
static V4_U32_LAST_MASKS: [u16; 17] = [
    0b0000000000000000,
    0b1000000000000000,
    0b1100000000000000,
    0b1110000000000000,
    0b1111000000000000,
    0b1111100000000000,
    0b1111110000000000,
    0b1111111000000000,
    0b1111111100000000,
    0b1111111110000000,
    0b1111111111000000,
    0b1111111111100000,
    0b1111111111110000,
    0b1111111111111000,
    0b1111111111111100,
    0b1111111111111110,
    0b1111111111111111,
];

#[rustfmt::skip]
impl Simd for V2 {
    type m32s = m32x4;
    type f32s = f32x4;
    type i32s = i32x4;
    type u32s = u32x4;

    type m64s = m64x2;
    type f64s = f64x2;
    type i64s = i64x2;
    type u64s = u64x2;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_xor_si128(_mm_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_and_si128(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_or_si128(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_xor_si128(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmpeq_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmplt_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm_cmple_ps(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmpeq_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmplt_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm_cmple_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm_max_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm_sub_epi64(transmute(a), transmute(b))) } }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm_set1_epi64x(value as i64)) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_blendv_pd(if_false, if_true, mask))
        }
    }
    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __m128d = transmute(mask);
            let if_true: __m128d = transmute(if_true);
            let if_false: __m128d = transmute(if_false);

            transmute(_mm_blendv_pd(if_false, if_true, mask))
        }
    }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        struct Impl<Op> {
            this: V2,
            op: Op,
        }
        impl<Op: WithSimd> crate::NullaryFnOnce for Impl<Op> {
            type Output = Op::Output;

            #[inline(always)]
            fn call(self) -> Self::Output {
                self.op.with_simd(self.this)
            }
        }
        self.vectorize(Impl { this: self, op })
    }

    #[inline(always)]
    fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            // a0 a1 a2 a3
            let a: __m128 = transmute(a);
            // a2 a3 a2 a3
            let hi = _mm_movehl_ps(a, a);

            // a0+a2 a1+a3 _ _
            let r0 = _mm_add_ps(a, hi);
            // a1+a3 a2+a1 _ _
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);

            let r = _mm_add_ss(r0, r0_shuffled);

            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)]
    fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_mul_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_mul_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)]
    fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_min_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_min_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)]
    fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m128 = transmute(a);
            let hi = _mm_movehl_ps(a, a);
            let r0 = _mm_max_ps(a, hi);
            let r0_shuffled = _mm_shuffle_ps::<0b0001>(r0, r0);
            let r = _mm_max_ss(r0, r0_shuffled);
            _mm_cvtss_f32(r)
        }
    }
    #[inline(always)]
    fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_add_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)]
    fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_mul_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)]
    fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_min_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }
    #[inline(always)]
    fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m128d = transmute(a);
            let hi = transmute(_mm_movehl_ps(transmute(a), transmute(a)));
            let r = _mm_max_sd(a, hi);
            _mm_cvtsd_f64(r)
        }
    }

    type c32s = f32x4;
    type c64s = f64x2;

    #[inline(always)] fn c32s_splat(self, value: c32) -> Self::c32s { cast(self.f64s_splat(cast(value))) }
    #[inline(always)] fn c32s_add(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_add(a, b) }
    #[inline(always)] fn c32s_sub(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_sub(a, b) }

    #[inline(always)]
    fn c32s_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm_shuffle_ps::<0b10_11_00_01>(xy, xy);
            let aa = _mm_moveldup_ps(ab);
            let bb = _mm_movehdup_ps(ab);

            cast(_mm_addsub_ps(_mm_mul_ps(aa, xy), _mm_mul_ps(bb, yx)))
        }
    }

    #[inline(always)] fn f32s_mul_add(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s {
        f32x4(
            f32::mul_add(a.0, b.0, c.0),
            f32::mul_add(a.1, b.1, c.1),
            f32::mul_add(a.2, b.2, c.2),
            f32::mul_add(a.3, b.3, c.3),
        )
    }
    #[inline(always)] fn f64s_mul_add(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s {
        f64x2(f64::mul_add(a.0, b.0, c.0), f64::mul_add(a.1, b.1, c.1))
    }

    #[inline(always)] fn c64s_splat(self, value: c64) -> Self::c64s { cast(value) }
    #[inline(always)] fn c64s_add(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_add(a, b) }
    #[inline(always)] fn c64s_sub(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_sub(a, b) }

    #[inline(always)]
    fn c64s_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm_shuffle_pd::<0b01>(xy, xy);
            let aa = _mm_unpacklo_pd(ab, ab);
            let bb = _mm_unpackhi_pd(ab, ab);

            cast(_mm_addsub_pd(_mm_mul_pd(aa, xy), _mm_mul_pd(bb, yx)))
        }
    }

    #[inline(always)]
    fn c32s_abs2(self, a: Self::c32s) -> Self::c32s {
        unsafe {
            let sqr = self.f32s_mul(a, a);
            let sqr_rev = _mm_shuffle_ps::<0b10_11_00_01>(cast(sqr), cast(sqr));
            self.f32s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn c64s_abs2(self, a: Self::c64s) -> Self::c64s {
        unsafe {
            let sqr = self.f64s_mul(a, a);
            let sqr_rev = _mm_shuffle_pd::<0b01>(cast(sqr), cast(sqr));
            self.f64s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn u32s_partial_load(self, slice: &[u32]) -> Self::u32s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn u32s_partial_store(self, slice: &mut [u32], values: Self::u32s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn u64s_partial_load(self, slice: &[u64]) -> Self::u64s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn u64s_partial_store(self, slice: &mut [u64], values: Self::u64s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn c64s_partial_load(self, slice: &[c64]) -> Self::c64s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn c64s_partial_store(self, slice: &mut [c64], values: Self::c64s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn u32s_partial_load_last(self, slice: &[u32]) -> Self::u32s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn u32s_partial_store_last(self, slice: &mut [u32], values: Self::u32s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn u64s_partial_load_last(self, slice: &[u64]) -> Self::u64s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn u64s_partial_store_last(self, slice: &mut [u64], values: Self::u64s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn c64s_partial_load_last(self, slice: &[c64]) -> Self::c64s {
        let _ = &slice;
        todo!()
    }

    #[inline(always)]
    fn c64s_partial_store_last(self, slice: &mut [c64], values: Self::c64s) {
        let _ = (&slice, &values);
        todo!()
    }

    #[inline(always)]
    fn c32s_conj(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.c32s_splat(c32{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c32s_conj_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        self.c32s_mul(self.c32s_conj(a), b)
    }

    #[inline(always)]
    fn c32s_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.c32s_add(self.c32s_mul(a, b), c)
    }

    #[inline(always)]
    fn c32s_conj_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        self.c32s_add(self.c32s_conj_mul(a, b), c)
    }

    #[inline(always)]
    fn c64s_conj(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.c64s_splat(c64{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c64s_conj_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        self.c64s_mul(self.c64s_conj(a), b)
    }

    #[inline(always)]
    fn c64s_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.c64s_add(self.c64s_mul(a, b), c)
    }

    #[inline(always)]
    fn c64s_conj_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        self.c64s_add(self.c64s_conj_mul(a, b), c)
    }

    #[inline(always)]
    fn c32s_neg(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.f32s_splat(-0.0))
    }

    #[inline(always)]
    fn c32s_reduce_sum(self, a: Self::c32s) -> c32 {
        unsafe {
            // a0 a1 a2 a3
            let a: __m128 = transmute(a);
            // a2 a3 a2 a3
            let hi = _mm_movehl_ps(a, a);

            // a0+a2 a1+a3 _ _
            let r0 = _mm_add_ps(a, hi);

            cast(_mm_cvtsd_f64(cast(r0)))
        }
    }

    #[inline(always)]
    fn c64s_neg(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.f64s_splat(-0.0))
    }

    #[inline(always)]
    fn c64s_reduce_sum(self, a: Self::c64s) -> c64 {
        cast(a)
    }
}

#[rustfmt::skip]
impl Simd for V3 {
    type m32s = m32x8;
    type f32s = f32x8;
    type i32s = i32x8;
    type u32s = u32x8;

    type m64s = m64x4;
    type f64s = f64x4;
    type i64s = i64x4;
    type u64s = u64x4;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_xor_pd(transmute(_mm256_set1_epi32(-1)), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_and_pd(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_or_pd(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_xor_pd(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm256_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm256_cmp_ps::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm256_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm256_cmp_pd::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __m256d = transmute(mask);
            let if_true: __m256d = transmute(if_true);
            let if_false: __m256d = transmute(if_false);

            transmute(_mm256_blendv_pd(if_false, if_true, mask))
        }
    }
    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __m256d = transmute(mask);
            let if_true: __m256d = transmute(if_true);
            let if_false: __m256d = transmute(if_false);

            transmute(_mm256_blendv_pd(if_false, if_true, mask))
        }
    }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_max_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm256_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm256_set1_epi64x(value as i64)) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm256_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm256_sub_epi64(transmute(a), transmute(b))) } }

    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { unsafe { transmute(_mm256_fmadd_pd(a.as_vec(), b.as_vec(), c.as_vec())) } }
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { unsafe { transmute(_mm256_fmadd_ps(a.as_vec(), b.as_vec(), c.as_vec())) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        struct Impl<Op> {
            this: V3,
            op: Op,
        }
        impl<Op: WithSimd> crate::NullaryFnOnce for Impl<Op> {
            type Output = Op::Output;

            #[inline(always)]
            fn call(self) -> Self::Output {
                self.op.with_simd(self.this)
            }
        }
        self.vectorize(Impl { this: self, op })
    }

    #[inline(always)]
    fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_add_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_mul_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_product(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_min_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_min(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_max_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().f32s_reduce_max(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_add_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_mul_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_product(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_min_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_min(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_max_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().f64s_reduce_max(transmute(r))
        }
    }

    type c32s = f32x8;
    type c64s = f64x4;

    #[inline(always)] fn c32s_splat(self, value: c32) -> Self::c32s { cast(self.f64s_splat(cast(value))) }
    #[inline(always)] fn c32s_add(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_add(a, b) }
    #[inline(always)] fn c32s_sub(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_sub(a, b) }

    #[inline(always)]
    fn c32s_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm256_moveldup_ps(ab);
            let bb = _mm256_movehdup_ps(ab);

            cast(_mm256_fmaddsub_ps(aa, xy, _mm256_mul_ps(bb, yx)))
        }
    }

    #[inline(always)] fn f32s_mul_add(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { self.f32s_mul_adde(a, b, c) }
    #[inline(always)] fn f64s_mul_add(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { self.f64s_mul_adde(a, b, c) }

    #[inline(always)] fn c64s_splat(self, value: c64) -> Self::c64s { unsafe { cast(_mm256_broadcast_pd(&*(&value as *const _ as *const _))) } }
    #[inline(always)] fn c64s_add(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_add(a, b) }
    #[inline(always)] fn c64s_sub(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_sub(a, b) }

    #[inline(always)]
    fn c64s_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_pd::<0b0101>(xy);
            let aa = _mm256_unpacklo_pd(ab, ab);
            let bb = _mm256_unpackhi_pd(ab, ab);

            cast(_mm256_fmaddsub_pd(aa, xy, _mm256_mul_pd(bb, yx)))
        }
    }

    #[inline(always)]
    fn c32s_abs2(self, a: Self::c32s) -> Self::c32s {
        unsafe {
            let sqr = self.f32s_mul(a, a);
            let sqr_rev = _mm256_shuffle_ps::<0b10_11_00_01>(cast(sqr), cast(sqr));
            self.f32s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn c64s_abs2(self, a: Self::c64s) -> Self::c64s {
        unsafe {
            let sqr = self.f64s_mul(a, a);
            let sqr_rev = _mm256_shuffle_pd::<0b0101>(cast(sqr), cast(sqr));
            self.f64s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn u32s_partial_load(self, slice: &[u32]) -> Self::u32s {
        unsafe {
            let mask = cast(V3_U32_MASKS[slice.len().min(8)]);
            cast(_mm256_maskload_epi32(slice.as_ptr() as _, mask))
        }
    }

    #[inline(always)]
    fn u32s_partial_store(self, slice: &mut [u32], values: Self::u32s) {
        unsafe {
            let mask = cast(V3_U32_MASKS[slice.len().min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr() as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn u64s_partial_load(self, slice: &[u64]) -> Self::u64s {
        unsafe {
            let mask = cast(V3_U32_MASKS[(2 * slice.len()).min(8)]);
            cast(_mm256_maskload_epi64(slice.as_ptr() as _, mask))
        }
    }

    #[inline(always)]
    fn u64s_partial_store(self, slice: &mut [u64], values: Self::u64s) {
        unsafe {
            let mask = cast(V3_U32_MASKS[(slice.len() * 2).min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr() as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn c64s_partial_load(self, slice: &[c64]) -> Self::c64s {
        unsafe {
            let mask = cast(V3_U32_MASKS[(4 * slice.len()).min(8)]);
            cast(_mm256_maskload_epi64(slice.as_ptr() as _, mask))
        }
    }

    #[inline(always)]
    fn c64s_partial_store(self, slice: &mut [c64], values: Self::c64s) {
        unsafe {
            let mask = cast(V3_U32_MASKS[(slice.len() * 4).min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr() as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn u32s_partial_load_last(self, slice: &[u32]) -> Self::u32s {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[len.min(8)]);
            cast(_mm256_maskload_epi32(slice.as_ptr().add(len).wrapping_sub(8) as _, mask))
        }
    }

    #[inline(always)]
    fn u32s_partial_store_last(self, slice: &mut [u32], values: Self::u32s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[len.min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr().add(len).wrapping_sub(8) as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn u64s_partial_load_last(self, slice: &[u64]) -> Self::u64s {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[(2 * len).min(8)]);
            cast(_mm256_maskload_epi64(slice.as_ptr().add(len).wrapping_sub(4) as _, mask))
        }
    }

    #[inline(always)]
    fn u64s_partial_store_last(self, slice: &mut [u64], values: Self::u64s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[(len * 2).min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr().add(len).wrapping_sub(4) as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn c64s_partial_load_last(self, slice: &[c64]) -> Self::c64s {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[(4 * len).min(8)]);
            cast(_mm256_maskload_epi64(slice.as_ptr().add(len).wrapping_sub(2) as _, mask))
        }
    }

    #[inline(always)]
    fn c64s_partial_store_last(self, slice: &mut [c64], values: Self::c64s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V3_U32_LAST_MASKS[(len * 4).min(8)]);
            _mm256_maskstore_epi32(slice.as_mut_ptr().add(len).wrapping_sub(2) as _, mask, cast(values))
        }
    }

    #[inline(always)]
    fn c32s_conj(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.c32s_splat(c32{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c32s_conj_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm256_moveldup_ps(ab);
            let bb = _mm256_movehdup_ps(ab);

            cast(_mm256_fmsubadd_ps(aa, xy, _mm256_mul_ps(bb, yx)))
        }
    }

    #[inline(always)]
    fn c32s_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm256_moveldup_ps(ab);
            let bb = _mm256_movehdup_ps(ab);

            cast(_mm256_fmaddsub_ps(aa, xy, _mm256_fmaddsub_ps(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c32s_conj_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm256_moveldup_ps(ab);
            let bb = _mm256_movehdup_ps(ab);

            cast(_mm256_fmsubadd_ps(aa, xy, _mm256_fmsubadd_ps(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c64s_conj(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.c64s_splat(c64{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c64s_conj_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_pd::<0b0101>(xy);
            let aa = _mm256_unpacklo_pd(ab, ab);
            let bb = _mm256_unpackhi_pd(ab, ab);

            cast(_mm256_fmsubadd_pd(aa, xy, _mm256_mul_pd(bb, yx)))
        }
    }

    #[inline(always)]
    fn c64s_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_pd::<0b0101>(xy);
            let aa = _mm256_unpacklo_pd(ab, ab);
            let bb = _mm256_unpackhi_pd(ab, ab);

            cast(_mm256_fmaddsub_pd(aa, xy, _mm256_fmaddsub_pd(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c64s_conj_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm256_permute_pd::<0b0101>(xy);
            let aa = _mm256_unpacklo_pd(ab, ab);
            let bb = _mm256_unpackhi_pd(ab, ab);

            cast(_mm256_fmsubadd_pd(aa, xy, _mm256_fmsubadd_pd(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c32s_neg(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.f32s_splat(-0.0))
    }

    #[inline(always)]
    fn c32s_reduce_sum(self, a: Self::c32s) -> c32 {
        unsafe {
            let a: __m256 = transmute(a);
            let r = _mm_add_ps(_mm256_castps256_ps128(a), _mm256_extractf128_ps::<1>(a));
            V2::new_unchecked().c32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn c64s_neg(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.f64s_splat(-0.0))
    }

    #[inline(always)]
    fn c64s_reduce_sum(self, a: Self::c64s) -> c64 {
        unsafe {
            let a: __m256d = transmute(a);
            let r = _mm_add_pd(_mm256_castpd256_pd128(a), _mm256_extractf128_pd::<1>(a));
            V2::new_unchecked().c64s_reduce_sum(transmute(r))
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
#[cfg(feature = "nightly")]
pub struct m32x16(pub __mmask16);

#[derive(Copy, Clone)]
#[repr(transparent)]
#[cfg(feature = "nightly")]
pub struct m64x8(pub __mmask8);

#[cfg(feature = "nightly")]
impl Debug for m64x8 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("m64x8")
            .field(&((self.0 >> 0x0) & 1 == 1))
            .field(&((self.0 >> 0x1) & 1 == 1))
            .field(&((self.0 >> 0x2) & 1 == 1))
            .field(&((self.0 >> 0x3) & 1 == 1))
            .field(&((self.0 >> 0x4) & 1 == 1))
            .field(&((self.0 >> 0x5) & 1 == 1))
            .field(&((self.0 >> 0x6) & 1 == 1))
            .field(&((self.0 >> 0x7) & 1 == 1))
            .finish()
    }
}

#[cfg(feature = "nightly")]
impl Debug for m32x16 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("m32x16")
            .field(&((self.0 >> 0x0) & 1 == 1))
            .field(&((self.0 >> 0x1) & 1 == 1))
            .field(&((self.0 >> 0x2) & 1 == 1))
            .field(&((self.0 >> 0x3) & 1 == 1))
            .field(&((self.0 >> 0x4) & 1 == 1))
            .field(&((self.0 >> 0x5) & 1 == 1))
            .field(&((self.0 >> 0x6) & 1 == 1))
            .field(&((self.0 >> 0x7) & 1 == 1))
            .field(&((self.0 >> 0x8) & 1 == 1))
            .field(&((self.0 >> 0x9) & 1 == 1))
            .field(&((self.0 >> 0xa) & 1 == 1))
            .field(&((self.0 >> 0xb) & 1 == 1))
            .field(&((self.0 >> 0xc) & 1 == 1))
            .field(&((self.0 >> 0xd) & 1 == 1))
            .field(&((self.0 >> 0xe) & 1 == 1))
            .field(&((self.0 >> 0xf) & 1 == 1))
            .finish()
    }
}

#[cfg(feature = "nightly")]
#[rustfmt::skip]
impl Simd for V4 {
    type m32s = m32x16;
    type f32s = f32x16;
    type i32s = i32x16;
    type u32s = u32x16;

    type m64s = m64x8;
    type f64s = f64x8;
    type i64s = i64x8;
    type u64s = u64x8;

    #[inline] fn m32s_not(self, a: Self::m32s) -> Self::m32s { m32x16(!a.0) }
    #[inline] fn m32s_and(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 & b.0) }
    #[inline] fn m32s_or(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 | b.0) }
    #[inline] fn m32s_xor(self, a: Self::m32s, b: Self::m32s) -> Self::m32s { m32x16(a.0 ^ b.0) }

    #[inline] fn m64s_not(self, a: Self::m64s) -> Self::m64s { m64x8(!a.0) }
    #[inline] fn m64s_and(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 & b.0) }
    #[inline] fn m64s_or(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 | b.0) }
    #[inline] fn m64s_xor(self, a: Self::m64s, b: Self::m64s) -> Self::m64s { m64x8(a.0 ^ b.0) }

    #[inline] fn u32s_not(self, a: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_xor_si512(_mm512_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u32s_and(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_and_si512(transmute(a), transmute(b))) } }
    #[inline] fn u32s_or(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_or_si512(transmute(a), transmute(b))) } }
    #[inline] fn u32s_xor(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_xor_si512(transmute(a), transmute(b))) } }

    #[inline] fn u64s_not(self, a: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_xor_si512(_mm512_set1_epi32(-1), transmute(a))) } }
    #[inline] fn u64s_and(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_and_si512(transmute(a), transmute(b))) } }
    #[inline] fn u64s_or(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_or_si512(transmute(a), transmute(b))) } }
    #[inline] fn u64s_xor(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_xor_si512(transmute(a), transmute(b))) } }

    #[inline] fn f32s_splat(self, value: f32) -> Self::f32s { unsafe { transmute(_mm512_set1_ps(value)) } }
    #[inline] fn f32s_add(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_add_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_sub(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_sub_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_mul(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_mul_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_div(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_div_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_less_than_or_equal(self, a: Self::f32s, b: Self::f32s) -> Self::m32s { unsafe { transmute(_mm512_cmp_ps_mask::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_splat(self, value: f64) -> Self::f64s { unsafe { transmute(_mm512_set1_pd(value)) } }
    #[inline] fn f64s_add(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_add_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_sub(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_sub_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_mul(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_mul_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_div(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_div_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_EQ_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_LT_OQ>(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_less_than_or_equal(self, a: Self::f64s, b: Self::f64s) -> Self::m64s { unsafe { transmute(_mm512_cmp_pd_mask::<_CMP_LE_OQ>(a.as_vec(), b.as_vec())) } }

    #[inline] fn f64s_mul_adde(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_fmadd_pd(a.as_vec(), b.as_vec(), c.as_vec())) } }
    #[inline] fn f32s_mul_adde(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_fmadd_ps(a.as_vec(), b.as_vec(), c.as_vec())) } }

    #[inline]
    fn m32s_select_u32s(self, mask: Self::m32s, if_true: Self::u32s, if_false: Self::u32s) -> Self::u32s {
        unsafe {
            let mask: __mmask16 = mask.0;
            let if_true: __m512 = transmute(if_true);
            let if_false: __m512 = transmute(if_false);

            transmute(_mm512_mask_blend_ps(mask, if_false, if_true))
        }
    }

    #[inline]
    fn m64s_select_u64s(self, mask: Self::m64s, if_true: Self::u64s, if_false: Self::u64s) -> Self::u64s {
        unsafe {
            let mask: __mmask8 = mask.0;
            let if_true: __m512d = transmute(if_true);
            let if_false: __m512d = transmute(if_false);

            transmute(_mm512_mask_blend_pd(mask, if_false, if_true))
        }
    }

    #[inline] fn f32s_min(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_min_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f32s_max(self, a: Self::f32s, b: Self::f32s) -> Self::f32s { unsafe { transmute(_mm512_max_ps(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_min(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_min_pd(a.as_vec(), b.as_vec())) } }
    #[inline] fn f64s_max(self, a: Self::f64s, b: Self::f64s) -> Self::f64s { unsafe { transmute(_mm512_max_pd(a.as_vec(), b.as_vec())) } }

    #[inline] fn u32s_add(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_add_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u32s_sub(self, a: Self::u32s, b: Self::u32s) -> Self::u32s { unsafe { transmute(_mm512_sub_epi32(transmute(a), transmute(b))) } }
    #[inline] fn u64s_add(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_add_epi64(transmute(a), transmute(b))) } }
    #[inline] fn u64s_sub(self, a: Self::u64s, b: Self::u64s) -> Self::u64s { unsafe { transmute(_mm512_sub_epi64(transmute(a), transmute(b))) } }

    #[inline]
    fn vectorize<Op: WithSimd>(self, op: Op) -> Op::Output {
        struct Impl<Op> {
            this: V4,
            op: Op,
        }
        impl<Op: WithSimd> crate::NullaryFnOnce for Impl<Op> {
            type Output = Op::Output;

            #[inline(always)]
            fn call(self) -> Self::Output {
                self.op.with_simd(self.this)
            }
        }
        self.vectorize(Impl { this: self, op })
    }

    #[inline] fn u32s_splat(self, value: u32) -> Self::u32s { unsafe { transmute(_mm512_set1_epi32(value as i32)) } }
    #[inline] fn u64s_splat(self, value: u64) -> Self::u64s { unsafe { transmute(_mm512_set1_epi64(value as i64)) } }

    #[inline(always)]
    fn f32s_reduce_sum(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_add_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_product(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_mul_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_product(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_min(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_min_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_min(transmute(r))
        }
    }

    #[inline(always)]
    fn f32s_reduce_max(self, a: Self::f32s) -> f32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_max_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().f32s_reduce_max(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_sum(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_add_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_product(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_mul_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_product(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_min(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_min_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_min(transmute(r))
        }
    }

    #[inline(always)]
    fn f64s_reduce_max(self, a: Self::f64s) -> f64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_max_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().f64s_reduce_max(transmute(r))
        }
    }

    type c32s = f32x16;
    type c64s = f64x8;

    #[inline(always)] fn c32s_splat(self, value: c32) -> Self::c32s { cast(self.f64s_splat(cast(value))) }
    #[inline(always)] fn c32s_add(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_add(a, b) }
    #[inline(always)] fn c32s_sub(self, a: Self::c32s, b: Self::c32s) -> Self::c32s { self.f32s_sub(a, b) }

    #[inline(always)]
    fn c32s_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm512_moveldup_ps(ab);
            let bb = _mm512_movehdup_ps(ab);

            cast(_mm512_fmaddsub_ps(aa, xy, _mm512_mul_ps(bb, yx)))
        }
    }

    #[inline(always)] fn f32s_mul_add(self, a: Self::f32s, b: Self::f32s, c: Self::f32s) -> Self::f32s { self.f32s_mul_adde(a, b, c) }
    #[inline(always)] fn f64s_mul_add(self, a: Self::f64s, b: Self::f64s, c: Self::f64s) -> Self::f64s { self.f64s_mul_adde(a, b, c) }

    #[inline(always)] fn c64s_splat(self, value: c64) -> Self::c64s { unsafe { cast(_mm512_broadcast_f32x4(cast(value))) } }
    #[inline(always)] fn c64s_add(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_add(a, b) }
    #[inline(always)] fn c64s_sub(self, a: Self::c64s, b: Self::c64s) -> Self::c64s { self.f64s_sub(a, b) }

    #[inline(always)]
    fn c64s_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_pd::<0b01010101>(xy);
            let aa = _mm512_unpacklo_pd(ab, ab);
            let bb = _mm512_unpackhi_pd(ab, ab);

            cast(_mm512_fmaddsub_pd(aa, xy, _mm512_mul_pd(bb, yx)))
        }
    }

    #[inline(always)]
    fn c32s_abs2(self, a: Self::c32s) -> Self::c32s {
        unsafe {
            let sqr = self.f32s_mul(a, a);
            let sqr_rev = _mm512_shuffle_ps::<0b10_11_00_01>(cast(sqr), cast(sqr));
            self.f32s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn c64s_abs2(self, a: Self::c64s) -> Self::c64s {
        unsafe {
            let sqr = self.f64s_mul(a, a);
            let sqr_rev = _mm512_shuffle_pd::<0b01010101>(cast(sqr), cast(sqr));
            self.f64s_add(sqr, cast(sqr_rev))
        }
    }

    #[inline(always)]
    fn u32s_partial_load(self, slice: &[u32]) -> Self::u32s {
        unsafe {
            let mask = cast(V4_U32_MASKS[slice.len().min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr() as _))
        }
    }

    #[inline(always)]
    fn u32s_partial_store(self, slice: &mut [u32], values: Self::u32s) {
        unsafe {
            let mask = cast(V4_U32_MASKS[slice.len().min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr() as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn u64s_partial_load(self, slice: &[u64]) -> Self::u64s {
        unsafe {
            let mask = cast(V4_U32_MASKS[(2 * slice.len()).min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr() as _))
        }
    }

    #[inline(always)]
    fn u64s_partial_store(self, slice: &mut [u64], values: Self::u64s) {
        unsafe {
            let mask = cast(V4_U32_MASKS[(2 * slice.len()).min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr() as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn c64s_partial_load(self, slice: &[c64]) -> Self::c64s {
        unsafe {
            let mask = cast(V4_U32_MASKS[(4 * slice.len()).min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr() as _))
        }
    }

    #[inline(always)]
    fn c64s_partial_store(self, slice: &mut [c64], values: Self::c64s) {
        unsafe {
            let mask = cast(V4_U32_MASKS[(4 * slice.len()).min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr() as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn u32s_partial_load_last(self, slice: &[u32]) -> Self::u32s {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[slice.len().min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr().add(len).wrapping_sub(16) as _))
        }
    }

    #[inline(always)]
    fn u32s_partial_store_last(self, slice: &mut [u32], values: Self::u32s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[slice.len().min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr().add(len).wrapping_sub(16) as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn u64s_partial_load_last(self, slice: &[u64]) -> Self::u64s {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[(2 * slice.len()).min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr().add(len).wrapping_sub(8) as _))
        }
    }

    #[inline(always)]
    fn u64s_partial_store_last(self, slice: &mut [u64], values: Self::u64s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[(2 * slice.len()).min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr().add(len).wrapping_sub(8) as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn c64s_partial_load_last(self, slice: &[c64]) -> Self::c64s {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[(4 * slice.len()).min(16)]);
            cast(_mm512_maskz_loadu_epi32(mask, slice.as_ptr().add(len).wrapping_sub(4) as _))
        }
    }

    #[inline(always)]
    fn c64s_partial_store_last(self, slice: &mut [c64], values: Self::c64s) {
        unsafe {
            let len = slice.len();
            let mask = cast(V4_U32_LAST_MASKS[(4 * slice.len()).min(16)]);
            _mm512_mask_storeu_epi32(slice.as_mut_ptr().add(len).wrapping_sub(4) as _, mask, cast(values));
        }
    }

    #[inline(always)]
    fn c32s_conj(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.c32s_splat(c32{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c32s_conj_mul(self, a: Self::c32s, b: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm512_moveldup_ps(ab);
            let bb = _mm512_movehdup_ps(ab);

            cast(Self::fmsubadd_ps(aa, xy, _mm512_mul_ps(bb, yx)))
        }
    }

    #[inline(always)]
    fn c32s_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm512_moveldup_ps(ab);
            let bb = _mm512_movehdup_ps(ab);

            cast(_mm512_fmaddsub_ps(aa, xy, _mm512_fmaddsub_ps(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c32s_conj_mul_adde(self, a: Self::c32s, b: Self::c32s, c: Self::c32s) -> Self::c32s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_ps::<0b10_11_00_01>(xy);
            let aa = _mm512_moveldup_ps(ab);
            let bb = _mm512_movehdup_ps(ab);

            cast(Self::fmsubadd_ps(aa, xy, Self::fmsubadd_ps(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c64s_conj(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.c64s_splat(c64{ re: 0.0, im: -0.0 }))
    }

    #[inline(always)]
    fn c64s_conj_mul(self, a: Self::c64s, b: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_pd::<0b01010101>(xy);
            let aa = _mm512_unpacklo_pd(ab, ab);
            let bb = _mm512_unpackhi_pd(ab, ab);

            cast(Self::fmsubadd_pd(aa, xy, _mm512_mul_pd(bb, yx)))
        }
    }

    #[inline(always)]
    fn c64s_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_pd::<0b01010101>(xy);
            let aa = _mm512_unpacklo_pd(ab, ab);
            let bb = _mm512_unpackhi_pd(ab, ab);

            cast(_mm512_fmaddsub_pd(aa, xy, _mm512_fmaddsub_pd(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c64s_conj_mul_adde(self, a: Self::c64s, b: Self::c64s, c: Self::c64s) -> Self::c64s {
        unsafe {
            let ab = cast(a);
            let xy = cast(b);

            let yx = _mm512_permute_pd::<0b01010101>(xy);
            let aa = _mm512_unpacklo_pd(ab, ab);
            let bb = _mm512_unpackhi_pd(ab, ab);

            cast(Self::fmsubadd_pd(aa, xy, Self::fmsubadd_pd(bb, yx, cast(c))))
        }
    }

    #[inline(always)]
    fn c32s_neg(self, a: Self::c32s) -> Self::c32s {
        self.f32s_xor(a, self.f32s_splat(-0.0))
    }

    #[inline(always)]
    fn c32s_reduce_sum(self, a: Self::c32s) -> c32 {
        unsafe {
            let a: __m512 = transmute(a);
            let r = _mm256_add_ps(_mm512_castps512_ps256(a), transmute(_mm512_extractf64x4_pd::<1>(transmute(a))));
            V3::new_unchecked().c32s_reduce_sum(transmute(r))
        }
    }

    #[inline(always)]
    fn c64s_neg(self, a: Self::c64s) -> Self::c64s {
        self.f64s_xor(a, self.f64s_splat(-0.0))
    }

    #[inline(always)]
    fn c64s_reduce_sum(self, a: Self::c64s) -> c64 {
        unsafe {
            let a: __m512d = transmute(a);
            let r = _mm256_add_pd(_mm512_castpd512_pd256(a), _mm512_extractf64x4_pd::<1>(a));
            V3::new_unchecked().c64s_reduce_sum(transmute(r))
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchInner {
    #[cfg(feature = "nightly")]
    V4(V4),
    V3(V3),
    Scalar(crate::Scalar),
}

impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        #[cfg(feature = "nightly")]
        if let Some(simd) = V4::try_new() {
            return Self::V4(simd);
        }
        if let Some(simd) = V3::try_new() {
            return Self::V3(simd);
        }
        Self::Scalar(crate::Scalar::new())
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            #[cfg(feature = "nightly")]
            ArchInner::V4(simd) => Simd::vectorize(simd, op),
            ArchInner::V3(simd) => Simd::vectorize(simd, op),
            ArchInner::Scalar(simd) => Simd::vectorize(simd, op),
        }
    }

    #[inline(always)]
    pub fn dispatch_true_simd<Op: WithSimd>(self, op: Op) -> Result<Op::Output, Op> {
        match self {
            #[cfg(feature = "nightly")]
            ArchInner::V4(simd) => Ok(Simd::vectorize(simd, op)),
            ArchInner::V3(simd) => Ok(Simd::vectorize(simd, op)),
            ArchInner::Scalar(_) => Err(op),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::vec;
    use alloc::vec::Vec;
    use assert_approx_eq::assert_approx_eq;
    use core::iter::zip;
    use rand::random;

    #[test]
    fn times_two() {
        let n = 1312;
        let mut v = (0..n).map(|i| i as f64).collect::<Vec<_>>();
        let arch = Arch::new();

        struct TimesThree<'a>(&'a mut [f64]);
        impl<'a> WithSimd for TimesThree<'a> {
            type Output = ();

            #[inline(always)]
            fn with_simd<S: Simd>(self, simd: S) -> Self::Output {
                let v = self.0;
                let (head, tail) = S::f64s_as_mut_simd(v);

                let three = simd.f64s_splat(3.0);
                for x in head {
                    *x = simd.f64s_mul(three, *x);
                }

                for x in tail {
                    *x *= 3.0;
                }
            }
        }

        arch.dispatch(|| {
            for x in &mut v {
                *x *= 2.0;
            }
        });

        arch.dispatch(TimesThree(&mut v));

        for (i, x) in v.into_iter().enumerate() {
            assert_eq!(x, 6.0 * i as f64);
        }
    }

    #[test]
    fn cplx_ops() {
        let n = 16;
        let a = (0..n)
            .map(|_| c32 {
                re: random(),
                im: random(),
            })
            .collect::<Vec<_>>();
        let b = (0..n)
            .map(|_| c32 {
                re: random(),
                im: random(),
            })
            .collect::<Vec<_>>();
        let c = (0..n)
            .map(|_| c32 {
                re: random(),
                im: random(),
            })
            .collect::<Vec<_>>();

        let axb_target = zip(&a, &b).map(|(a, b)| a * b).collect::<Vec<_>>();
        let conjaxb_target = zip(&a, &b).map(|(a, b)| a.conj() * b).collect::<Vec<_>>();
        let axbpc_target = zip(zip(&a, &b), &c)
            .map(|((a, b), c)| a * b + c)
            .collect::<Vec<_>>();
        let conjaxbpc_target = zip(zip(&a, &b), &c)
            .map(|((a, b), c)| a.conj() * b + c)
            .collect::<Vec<_>>();

        if let Some(simd) = V2::try_new() {
            let mut axb = vec![c32::new(0.0, 0.0); n];
            let mut conjaxb = vec![c32::new(0.0, 0.0); n];
            let mut axbpc = vec![c32::new(0.0, 0.0); n];
            let mut conjaxbpc = vec![c32::new(0.0, 0.0); n];

            {
                let a = V2::c32s_as_simd(&a).0;
                let b = V2::c32s_as_simd(&b).0;
                let c = V2::c32s_as_simd(&c).0;
                let axb = V2::c32s_as_mut_simd(&mut axb).0;
                let conjaxb = V2::c32s_as_mut_simd(&mut conjaxb).0;
                let axbpc = V2::c32s_as_mut_simd(&mut axbpc).0;
                let conjaxbpc = V2::c32s_as_mut_simd(&mut conjaxbpc).0;

                for (axb, (a, b)) in zip(axb, zip(a, b)) {
                    *axb = simd.c32s_mul(*a, *b);
                }
                for (conjaxb, (a, b)) in zip(conjaxb, zip(a, b)) {
                    *conjaxb = simd.c32s_conj_mul(*a, *b);
                }
                for (axbpc, ((a, b), c)) in zip(axbpc, zip(zip(a, b), c)) {
                    *axbpc = simd.c32s_mul_adde(*a, *b, *c);
                }
                for (conjaxbpc, ((a, b), c)) in zip(conjaxbpc, zip(zip(a, b), c)) {
                    *conjaxbpc = simd.c32s_conj_mul_adde(*a, *b, *c);
                }
            }

            for (target, actual) in zip(&axb_target, &axb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxb_target, &conjaxb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&axbpc_target, &axbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxbpc_target, &conjaxbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
        }

        if let Some(simd) = V3::try_new() {
            let mut axb = vec![c32::new(0.0, 0.0); n];
            let mut conjaxb = vec![c32::new(0.0, 0.0); n];
            let mut axbpc = vec![c32::new(0.0, 0.0); n];
            let mut conjaxbpc = vec![c32::new(0.0, 0.0); n];

            {
                let a = V3::c32s_as_simd(&a).0;
                let b = V3::c32s_as_simd(&b).0;
                let c = V3::c32s_as_simd(&c).0;
                let axb = V3::c32s_as_mut_simd(&mut axb).0;
                let conjaxb = V3::c32s_as_mut_simd(&mut conjaxb).0;
                let axbpc = V3::c32s_as_mut_simd(&mut axbpc).0;
                let conjaxbpc = V3::c32s_as_mut_simd(&mut conjaxbpc).0;

                for (axb, (a, b)) in zip(axb, zip(a, b)) {
                    *axb = simd.c32s_mul(*a, *b);
                }
                for (conjaxb, (a, b)) in zip(conjaxb, zip(a, b)) {
                    *conjaxb = simd.c32s_conj_mul(*a, *b);
                }
                for (axbpc, ((a, b), c)) in zip(axbpc, zip(zip(a, b), c)) {
                    *axbpc = simd.c32s_mul_adde(*a, *b, *c);
                }
                for (conjaxbpc, ((a, b), c)) in zip(conjaxbpc, zip(zip(a, b), c)) {
                    *conjaxbpc = simd.c32s_conj_mul_adde(*a, *b, *c);
                }
            }

            for (target, actual) in zip(&axb_target, &axb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxb_target, &conjaxb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&axbpc_target, &axbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxbpc_target, &conjaxbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
        }

        #[cfg(feature = "nightly")]
        if let Some(simd) = V4::try_new() {
            let mut axb = vec![c32::new(0.0, 0.0); n];
            let mut conjaxb = vec![c32::new(0.0, 0.0); n];
            let mut axbpc = vec![c32::new(0.0, 0.0); n];
            let mut conjaxbpc = vec![c32::new(0.0, 0.0); n];

            {
                let a = V4::c32s_as_simd(&a).0;
                let b = V4::c32s_as_simd(&b).0;
                let c = V4::c32s_as_simd(&c).0;
                let axb = V4::c32s_as_mut_simd(&mut axb).0;
                let conjaxb = V4::c32s_as_mut_simd(&mut conjaxb).0;
                let axbpc = V4::c32s_as_mut_simd(&mut axbpc).0;
                let conjaxbpc = V4::c32s_as_mut_simd(&mut conjaxbpc).0;

                for (axb, (a, b)) in zip(axb, zip(a, b)) {
                    *axb = simd.c32s_mul(*a, *b);
                }
                for (conjaxb, (a, b)) in zip(conjaxb, zip(a, b)) {
                    *conjaxb = simd.c32s_conj_mul(*a, *b);
                }
                for (axbpc, ((a, b), c)) in zip(axbpc, zip(zip(a, b), c)) {
                    *axbpc = simd.c32s_mul_adde(*a, *b, *c);
                }
                for (conjaxbpc, ((a, b), c)) in zip(conjaxbpc, zip(zip(a, b), c)) {
                    *conjaxbpc = simd.c32s_conj_mul_adde(*a, *b, *c);
                }
            }

            for (target, actual) in zip(&axb_target, &axb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxb_target, &conjaxb) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&axbpc_target, &axbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
            for (target, actual) in zip(&conjaxbpc_target, &conjaxbpc) {
                assert_approx_eq!(target.re, actual.re);
                assert_approx_eq!(target.im, actual.im);
            }
        }
    }
}
