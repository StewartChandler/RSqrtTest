#![cfg_attr(feature = "nightly", feature(stdsimd))]

mod q_rsqrt;
pub use q_rsqrt::*;

mod quake_rsqrt;
mod sse_rsqrt;
mod avx512_rsqrt;
mod asm_avx512_rsqrt;
mod basic_rsqrt;
mod exact_rsqrt;
