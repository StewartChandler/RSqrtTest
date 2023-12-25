use std::{
    arch::asm,
    mem::{transmute, MaybeUninit},
    ops::{Deref, DerefMut},
};

use crate::{exact_rsqrt::ExactRSqrt, QRSqrt};

#[repr(align(64))]
#[derive(Debug, Clone)]
struct SimdAlignedBuf<T, const N: usize>([T; N]);

impl<T, const N: usize> Deref for SimdAlignedBuf<T, N> {
    type Target = [T; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for SimdAlignedBuf<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct OpTester<const N: usize> {
    refer: Box<SimdAlignedBuf<f32, N>>,
    exact: Box<SimdAlignedBuf<f32, N>>,
    result: Option<Box<SimdAlignedBuf<f32, N>>>,
    error: Option<Box<SimdAlignedBuf<f32, N>>>,
}

impl<const N: usize> OpTester<N> {
    pub const fn len() -> usize {
        N
    }

    pub fn new<I, T>(it: I) -> Self
    where
        T: Into<f32>,
        I: IntoIterator<Item = T>,
    {
        let iter = it
            .into_iter()
            .map(|val| Into::<f32>::into(val))
            .chain((1..).map(|i| i as f32));

        let mut refer = Box::new(SimdAlignedBuf([MaybeUninit::<f32>::uninit(); N]));
        let mut exact = Box::new(SimdAlignedBuf([MaybeUninit::<f32>::uninit(); N]));
        for (i, val) in iter.enumerate().take(N) {
            refer[i].write(val);
            let res = <ExactRSqrt as QRSqrt>::q_rsqrt(val);
            exact[i].write(res);
        }

        // then as every value in exact was written to, we can assume init for exact, safely,
        // however `assume_init` for slices in unstable, so we must use transmute
        let exact: Box<SimdAlignedBuf<f32, N>> = unsafe { transmute(exact) };
        let refer: Box<SimdAlignedBuf<f32, N>> = unsafe { transmute(refer) };

        Self {
            refer,
            exact,
            result: None,
            error: None,
        }
    }

    pub fn run_q_sqrt_seq<T: QRSqrt>(&mut self) {
        let mut res_buf = if let Some(res) = self.result.take() {
            unsafe { transmute(res) }
        } else {
            Box::new(SimdAlignedBuf([MaybeUninit::<f32>::uninit(); N]))
        };

        for (i, mut val) in self.refer.iter().copied().enumerate() {
            // just to aviod the loop from being vectorized
            unsafe {
                asm!("/* {0} */", inout(xmm_reg) val, options(pure, nomem, nostack, preserves_flags))
            };

            let res = T::q_rsqrt(val);
            res_buf[i].write(res);
        }

        // then, we have written all of the data, so this is okay
        self.result = Some(unsafe { transmute(res_buf) });
    }

    pub fn calculate_error(&mut self) -> Option<&mut [f32; N]> {
        if let Some(result) = &self.result {
            let mut err_buf = if let Some(err) = self.error.take() {
                unsafe { transmute(err) }
            } else {
                Box::new(SimdAlignedBuf([MaybeUninit::<f32>::uninit(); N]))
            };

            for ((x_calc, x_true), rel_err) in result
                .iter()
                .copied()
                .zip(self.exact.iter().copied())
                .zip(err_buf.iter_mut())
            {
                rel_err.write((x_calc - x_true).abs() / x_true.abs());
            }

            // result
            self.error = Some(unsafe { transmute(err_buf) });

            // Safety
            self.error.as_mut().map(|val| &mut **(val.as_mut()))
        } else {
            None
        }
    }
}
