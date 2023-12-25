#![cfg_attr(feature = "nightly", feature(stdsimd))]

mod q_rsqrt;
pub use q_rsqrt::*;

pub mod asm_avx512_rsqrt;
pub mod avx512_rsqrt;
pub mod basic_rsqrt;
pub mod exact_rsqrt;
pub mod quake_rsqrt;
pub mod sse_rsqrt;

pub mod operation_tester;
