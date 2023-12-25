use crate::QRSqrt;

/// An implementation of the famous [`q_rsqrt`] function from Quake 3. Mostly defunct nowadays as
/// more useful techniques exist including, on modern-ish x86/x86_64 platforms the [`rsqrtss`] SSE
/// instruction or, on more modern platforms, the AVX512 instruction, [`rsqrt14ss`].
///
/// Uses the constants given by the work of [Jan Kadlec].
///
/// [`q_rsqrt`]: https://en.wikipedia.org/wiki/Fast_inverse_square_root
/// [`rsqrtss`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt_ss
/// [`rsqrt14ss`]: https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html#text=_mm_rsqrt14_ss
/// [Jan Kadlec]: http://rrrola.wz.cz/inv_sqrt.html
pub struct QuakeRSqrt;

impl QRSqrt for QuakeRSqrt {
    #[inline(always)]
    fn q_rsqrt(num: f32) -> f32 {
        let mut x = num.to_bits();
        x = 0x5f1ffff9 - (x >> 1);
        let y = f32::from_bits(x);
        y * 0.703952253f32 * (2.38924456f32 - num * y * y)
    }
}
