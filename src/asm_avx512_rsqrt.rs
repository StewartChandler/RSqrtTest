#![cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use crate::QRSqrt;

use std::arch::asm;

/// An appromation of the reciprocal square root using the [`vrsqrt14ss`] AVX512 instruction on
/// x86_x64, implemented using inline asm to avoid calls to intrensics.  One of the downsides of
/// this method is that 
///
/// From intel's documentation: "The maximum relative error for this approximation is less than
/// 2^-14."
///
/// [`vrsqrt14ss`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt14_ss
pub struct AsmAVX512RSqrt;

impl QRSqrt for AsmAVX512RSqrt {
    #[inline(always)]
    fn q_rsqrt(mut num: f32) -> f32 {
        // as f32 args are stored in xmm0, should compile to `vrsqrt14ss xmm0 xmm0 xmm0`, which will
        // be inlined and (hopefully) shouldn't cause weird compiled assembly that we got with the
        // intrensics, but maybe also won't get benefit from vectorization in the way that other
        // methods will as the compiler will not know too vectorize it.
        //
        // From the intel software development manual, pg. 2642:
        // "MXCSR exception flags are not affected by this instruction and floating-point exceptions
        // are not reported."
        //
        // So we can mark it as 'preserves_flags'
        unsafe {
            asm!(
                "vrsqrt14ss {0} {0} {0}",
                inout(xmm_reg) num,
                options(pure, nomem, nostack, preserves_flags)
            )
        };

        num
    }
}
