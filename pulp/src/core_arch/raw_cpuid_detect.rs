macro_rules! raw_cpuid_detect {
	("aes") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_aesni())
	};
	("pclmulqdq") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_pclmulqdq())
	};
	("rdrand") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_rdrand())
	};
	("rdseed") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_rdseed())
	};
	("sse") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_sse())
	};
	("sse2") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_sse2())
	};
	("sse3") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_sse3())
	};
	("ssse3") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_ssse3())
	};
	("sse4.1") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_sse41())
	};
	("sse4.2") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_sse42())
	};
	("sse4a") => {
		raw_cpuid::CpuId::new()
			.get_extended_processor_and_feature_identifiers()
			.is_some_and(|x| x.has_sse4a())
	};
	("sha") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_sha())
	};
	("avx") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_avx())
	};
	("avx2") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx2())
	};
	("f16c") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_f16c())
	};
	("fma") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_fma())
	};
	("bmi1") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_bmi1())
	};
	("bmi2") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_bmi2())
	};
	("lzcnt") => {
		raw_cpuid::CpuId::new()
			.get_extended_processor_and_feature_identifiers()
			.is_some_and(|x| x.has_lzcnt())
	};
	("tbm") => {
		raw_cpuid::CpuId::new()
			.get_extended_processor_and_feature_identifiers()
			.is_some_and(|x| x.has_tbm())
	};
	("popcnt") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_popcnt())
	};
	("fxsr") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_fxsave_fxstor())
	};
	("xsave") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_xsave())
	};
	("xsaveopt") => {
		raw_cpuid::CpuId::new()
			.get_extended_state_info()
			.is_some_and(|x| x.has_xsaveopt())
	};
	("xsaves") => {
		raw_cpuid::CpuId::new()
			.get_extended_state_info()
			.is_some_and(|x| x.has_xsaves_xrstors())
	};
	("xsavec") => {
		raw_cpuid::CpuId::new()
			.get_extended_state_info()
			.is_some_and(|x| x.has_xsavec())
	};
	("cmpxchg16b") => {
		raw_cpuid::CpuId::new()
			.get_feature_info()
			.is_some_and(|x| x.has_cmpxchg16b())
	};
	("adx") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_adx())
	};
	("rtm") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_rtm())
	};
	("avx512vl") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512vl())
	};
	("avx512f") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512f())
	};
	("avx512cd") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512cd())
	};
	("avx512er") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512er())
	};
	("avx512pf") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512pf())
	};
	("avx512bw") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512bw())
	};
	("avx512dq") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512dq())
	};
	("avx512ifma") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512_ifma())
	};
	("avx512vbmi") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512vbmi())
	};
	("avx512vpopcntdq") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512vpopcntdq())
	};
	("avx512vbmi2") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512vbmi2())
	};
	("gfni") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_gfni())
	};
	("vaes") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_vaes())
	};
	("vpclmulqdq") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_vpclmulqdq())
	};
	("avx512vnni") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512vnni())
	};
	("avx512bitalg") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512bitalg())
	};
	("avx512bf16") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512_bf16())
	};
	("avx512vp2intersect") => {
		raw_cpuid::CpuId::new()
			.get_extended_feature_info()
			.is_some_and(|x| x.has_avx512_vp2intersect())
	};
}

macro_rules! feature_detected {
	($feature: tt) => {
		cfg!(target_feature = $feature) || raw_cpuid_detect!($feature)
	};
}
