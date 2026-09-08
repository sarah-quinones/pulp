//! 32-bit ARM NEON feature tokens + safe intrinsic wrappers.
//!
//! Gated on pulp `nightly` + `target_arch = "arm"` because Rust's
//! `core::arch::arm` NEON surface (`stdarch_arm_neon_intrinsics`) and the
//! `neon` target feature are still unstable on the stable channel.

use super::arch;
use arch::*;

macro_rules! __impl {
	($name: ident, $feature: tt) => {
		#[derive(Clone, Copy)]
		#[repr(transparent)]
		pub struct $name {
			__private: (),
		}

		impl $name {
			/// # Safety
			/// Not checked
			#[inline(always)]
			pub const unsafe fn new_unchecked() -> Self {
				Self { __private: () }
			}

			#[inline(always)]
			pub fn try_new() -> Option<Self> {
				if feature_detected!($feature) {
					Some(Self { __private: () })
				} else {
					None
				}
			}

			#[inline(always)]
			pub fn is_available() -> bool {
				feature_detected!($feature)
			}
		}

		impl ::core::fmt::Debug for $name {
			#[inline]
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> core::fmt::Result {
				f.write_str(stringify!($name))
			}
		}
	};
}

__impl!(Neon, "neon");

impl Neon {
	delegate!({
		fn vdupq_n_u8(value: u8) -> uint8x16_t;
		fn vdupq_n_s8(value: i8) -> int8x16_t;
		fn vaddq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn vaddq_s8(a: int8x16_t, b: int8x16_t) -> int8x16_t;
		fn vsubq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn veorq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn vandq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn vorrq_u8(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn vextq_u8<const N: i32>(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t;
		fn vgetq_lane_u8<const IMM5: i32>(v: uint8x16_t) -> u8;
		fn vsetq_lane_u8<const IMM5: i32>(a: u8, b: uint8x16_t) -> uint8x16_t;

		// f32 lane ops
		fn vdupq_n_f32(value: f32) -> float32x4_t;
		fn vaddq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t;
		fn vsubq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t;
		fn vmulq_f32(a: float32x4_t, b: float32x4_t) -> float32x4_t;

		// u32/u16 lane ops software f32<->f16 bit-trick
		fn vdupq_n_u32(value: u32) -> uint32x4_t;
		fn vandq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn veorq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vorrq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vaddq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vsubq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vcgeq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vcgtq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vcltq_u32(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vbslq_u32(mask: uint32x4_t, a: uint32x4_t, b: uint32x4_t) -> uint32x4_t;
		fn vshrq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t;
		fn vshlq_n_u32<const N: i32>(a: uint32x4_t) -> uint32x4_t;
		fn vmovn_u32(a: uint32x4_t) -> uint16x4_t;
		fn vmovl_u16(a: uint16x4_t) -> uint32x4_t;
		fn vcombine_u16(lo: uint16x4_t, hi: uint16x4_t) -> uint16x8_t;
		fn vget_low_u16(a: uint16x8_t) -> uint16x4_t;
		fn vget_high_u16(a: uint16x8_t) -> uint16x4_t;
		fn vsetq_lane_u16<const IMM4: i32>(a: u16, b: uint16x8_t) -> uint16x8_t;
		fn vgetq_lane_u16<const IMM4: i32>(v: uint16x8_t) -> u16;
		fn vreinterpretq_f32_u32(a: uint32x4_t) -> float32x4_t;
		fn vreinterpretq_u32_f32(a: float32x4_t) -> uint32x4_t;
	});

	/// AArch64-compatible `vqtbl1q_u8` for a 16-byte table.
	#[inline(always)]
	pub fn vqtbl1q_u8(self, t: uint8x16_t, idx: uint8x16_t) -> uint8x16_t {
		// Safety: NEON token proves the feature; types are plain 128-bit registers.
		let table: [u8; 16] = unsafe { core::mem::transmute(t) };
		let indices: [u8; 16] = unsafe { core::mem::transmute(idx) };
		let mut out = [0u8; 16];
		for i in 0..16 {
			let j = indices[i];
			// AArch64 vqtbl1 zeros lanes with index >= 16.
			out[i] = if (j as usize) < 16 {
				table[j as usize]
			} else {
				0
			};
		}
		unsafe { core::mem::transmute(out) }
	}

	/// Broadcast byte lane `IMM5` of `v` to all 16 lanes (handy AArch32 stand-in
	/// for `vqtbl1q` with a constant index).
	#[inline(always)]
	pub fn vdupq_laneq_u8<const IMM5: i32>(self, v: uint8x16_t) -> uint8x16_t {
		self.vdupq_n_u8(self.vgetq_lane_u8::<IMM5>(v))
	}
}
