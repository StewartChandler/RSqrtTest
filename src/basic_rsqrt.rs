use crate::QRSqrt;

/// An almost trivial basic implementation of the rsqrt function to approximate the reciprocal
/// square root by calculating the result directly. 
pub struct BasicRSqrt;

impl QRSqrt for BasicRSqrt {
    #[inline(always)]
    fn q_rsqrt(num: f32) -> f32 {
        1.0 / num.sqrt()
    }
}