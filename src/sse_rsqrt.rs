#![cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"
))]
use crate::QRSqrt;

#[cfg(target_arch = "x86")]
use std::arch::x86::{_mm_rsqrt_ss, _mm_set_ss, _mm_store_ss};
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_rsqrt_ss, _mm_set_ss, _mm_store_ss};

/// An appromation of the reciprocal square root using the [`rsqrtss`] SSE instruction on x86/x64,
/// implemented using intrensics only.
///
/// From intel's documentation: "The maximum relative error for this approximation is less than
/// 1.5\*2^-12."
///
/// [`rsqrtss`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt_ss
pub struct SSERSqrt;

impl QRSqrt for SSERSqrt {
    #[inline(always)]
    fn q_rsqrt(mut num: f32) -> f32 {
        // as f32 args are stored in xmm0 by default, the set and store functions should be
        // optimized out, but the possibility exists for pure asm to be faster for unoptimized
        // builds.
        let mut x = unsafe { _mm_set_ss(num) };
        x = unsafe { _mm_rsqrt_ss(x) };
        unsafe { _mm_store_ss(&mut num as *mut _, x) };

        num
    }
}
