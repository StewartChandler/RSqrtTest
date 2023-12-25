use crate::QRSqrt;

/// A baseline implementation of the reciprocal square root by using `f64` and downcasting to get
/// an exact answer to use as a baseline. 
pub struct ExactRSqrt;

impl QRSqrt for ExactRSqrt {
    #[inline(always)]
    fn q_rsqrt(num: f32) -> f32 {
        1.0 / num.sqrt()
    }
}