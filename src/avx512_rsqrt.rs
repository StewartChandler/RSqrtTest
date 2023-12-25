#![cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    feature = "nightly"
))]
use crate::QRSqrt;

use std::arch::x86_64::{_mm_rsqrt14_ss, _mm_set_ss, _mm_store_ss};

/// An appromation of the reciprocal square root using the [`vrsqrt14ss`] AVX512 instruction on
/// x86_x64, implemented using intrensics only.
///
/// From intel's documentation: "The maximum relative error for this approximation is less than
/// 2^-14."
///
/// [`vrsqrt14ss`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt14_ss
pub struct AVX512RSqrt;

impl QRSqrt for AVX512RSqrt {
    #[inline(always)]
    fn q_rsqrt(mut num: f32) -> f32 {
        // as f32 args are stored in xmm0 by default, the set and store functions should be
        // optimized out, but the possibility exists for pure asm to be faster for unoptimized
        // builds.
        let mut x = unsafe { _mm_set_ss(num) };
        x = unsafe { _mm_rsqrt14_ss(x, x) };
        unsafe { _mm_store_ss(&mut num as *mut _, x) };

        num
    }
}
