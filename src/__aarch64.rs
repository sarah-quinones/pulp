use super::*;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m32(u32);
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct m64(u64);

impl m32 {
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u32::MAX } else { 0 })
    }
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
    #[inline(always)]
    pub const fn flip(self) -> Self {
        Self(!self.0)
    }
}
impl m64 {
    #[inline(always)]
    pub const fn new(flag: bool) -> Self {
        Self(if flag { u64::MAX } else { 0 })
    }
    #[inline(always)]
    pub const fn is_set(self) -> bool {
        self.0 != 0
    }
    #[inline(always)]
    pub const fn flip(self) -> Self {
        Self(!self.0)
    }
}

impl core::fmt::Debug for m32 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}
impl core::fmt::Debug for m64 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        self.is_set().fmt(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct m32x4(pub m32, pub m32, pub m32, pub m32);
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[rustfmt::skip]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct i32x4(pub i32, pub i32, pub i32, pub i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct m64x2(pub m64, pub m64);
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
#[rustfmt::skip]
pub struct f64x2(pub f64, pub f64);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct i64x2(pub i64, pub i64);
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[rustfmt::skip]
pub struct u64x2(pub u64, pub u64);

unsafe impl Zeroable for m32 {}
unsafe impl Pod for m32 {}
unsafe impl Zeroable for m64 {}
unsafe impl Pod for m64 {}

unsafe impl Zeroable for m32x4 {}
unsafe impl Pod for m32x4 {}
unsafe impl Zeroable for f32x4 {}
unsafe impl Pod for f32x4 {}
unsafe impl Zeroable for i32x4 {}
unsafe impl Pod for i32x4 {}
unsafe impl Zeroable for u32x4 {}
unsafe impl Pod for u32x4 {}

unsafe impl Zeroable for m64x2 {}
unsafe impl Pod for m64x2 {}
unsafe impl Zeroable for f64x2 {}
unsafe impl Pod for f64x2 {}
unsafe impl Zeroable for i64x2 {}
unsafe impl Pod for i64x2 {}
unsafe impl Zeroable for u64x2 {}
unsafe impl Pod for u64x2 {}

#[derive(Debug, Clone, Copy)]
pub struct Neon;

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ArchInner {
    Neon(Neon),
    Scalar(crate::Scalar),
}

impl Seal for Neon {}
autovectorize_impl!(Neon, #[target_feature(enable = "neon")]);

#[cfg(feature = "std")]
macro_rules! aarch64_feature_detected {
    ($tt: tt) => {
        ::std::arch::is_aarch64_feature_detected!($tt)
    };
}

#[cfg(not(feature = "std"))]
macro_rules! aarch64_feature_detected {
    ($tt: tt) => {
        cfg!(target_arch = $tt)
    };
}

impl ArchInner {
    #[inline]
    pub fn new() -> Self {
        #[allow(unused_unsafe)]
        unsafe {
            if aarch64_feature_detected!("neon") {
                Self::Neon(Neon)
            } else {
                Self::Scalar(crate::Scalar::new())
            }
        }
    }

    #[inline(always)]
    pub fn dispatch<Op: WithSimd>(self, op: Op) -> Op::Output {
        match self {
            ArchInner::Neon(simd) => simd.vectorize(op),
            ArchInner::Scalar(simd) => simd.vectorize(op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    *x = *x * 3.0;
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
}
