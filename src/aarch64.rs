use super::*;

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
