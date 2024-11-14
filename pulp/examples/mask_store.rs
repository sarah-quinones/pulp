#[cfg(all(target_arch = "x86_64", feature = "nightly"))]
mod x86 {
    use diol::prelude::*;
    use pulp::x86::V4;
    use pulp::Simd;
    use std::arch::x86_64::__m512d;

    fn bench_masked_store(bencher: Bencher, (): ()) {
        let simd = V4::try_new().unwrap();

        let mut x: __m512d = pulp::cast(simd.splat_f64s(0.0));
        let x: &mut [f64] = bytemuck::cast_slice_mut(core::slice::from_mut(&mut x));
        let x = x.as_mut_ptr();
        let mask = simd.mask_between_m64s(3, 13);
        let raw_mask = mask.mask();
        let mask = raw_mask.into();

        bencher.bench(|| {
            simd.vectorize(
                #[inline(always)]
                || unsafe {
                    for _ in 0..16 {
                        simd.mask_store_ptr_f64s(mask, x, simd.mask_load_ptr_f64s(mask, x));
                    }
                },
            )
        });
    }

    fn bench_combined_stores(bencher: Bencher, (): ()) {
        let simd = V4::try_new().unwrap();

        let mut x: __m512d = pulp::cast(simd.splat_f64s(0.0));
        let x: &mut [f64] = bytemuck::cast_slice_mut(core::slice::from_mut(&mut x));
        let x = x.as_mut_ptr();
        let mask = simd.mask_between_m64s(3, 13);

        bencher.bench(|| {
            simd.vectorize(
                #[inline(always)]
                || unsafe {
                    for _ in 0..16 {
                        simd.mask_store_ptr_f64s(mask, x, simd.mask_load_ptr_f64s(mask, x));
                    }
                },
            )
        });
    }

    pub fn main() -> std::io::Result<()> {
        let mut bench = diol::Bench::new(BenchConfig::from_args()?);
        bench.register_many(list![bench_masked_store, bench_combined_stores], [()]);

        bench.run()?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    #[cfg(all(target_arch = "x86_64", feature = "nightly"))]
    x86::main()?;
    Ok(())
}
