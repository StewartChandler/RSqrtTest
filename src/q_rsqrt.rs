pub trait QRSqrt {
    /// Calculates an approximation of the reciprocal square root function (1 / sqrt(*x*)).
    ///
    /// # Examples
    /// ```
    /// use rsqrt_test::QRSqrt;
    ///
    /// struct BasicRSqrt;
    ///
    /// impl QRSqrt for BasicRSqrt {
    ///     fn q_rsqrt(num: f32) -> f32 {
    ///         1.0f32 / num.sqrt()
    ///     }
    /// }
    ///
    /// let x = 5.0f32;
    /// let ans = (1.0f64 / 5.0f64.sqrt()) as f32;
    /// let res = <BasicRSqrt as QRSqrt>::q_rsqrt(x);
    ///
    /// let rel_err = (ans - res).abs() / ans.abs();
    /// assert!(rel_err < 2.0f32.powi(-10));
    /// ```
    fn q_rsqrt(num: f32) -> f32;
}
