# RSqrt Test

This repository is something I created for the purposes of testing and analyzing the error and efficiency of different approaches to calculating the reciprocal square root (1 / sqrt(*x*)).  This was created out of messing around on [compiler explorer](https://godbolt.org) and finding that, among other things, the behaviour of rust's intrinsics particularly the [`std::arch::x86_64::_mm_rsqrt14_ss`] (`vrsqrt14ss`) instruction from the [AVX-512f] instruction set extension, produced far more instructions than I was expecting and was also (seemingly unnecessarily) dependent on rust's unstable [standard simd](https://github.com/rust-lang/rust/issues/48556) language feature.

[`std::arch::x86_64::_mm_rsqrt14_ss`]: https://doc.rust-lang.org/core/arch/x86_64/fn._mm_rsqrt14_ss.html 
[AVX-512f]: https://en.wikipedia.org/wiki/AVX-512

## Weird assembly with `_mm_rsqrt14_ss`

We can see with the usage of [compiler explorer](https://godbolt.org/z/soMr9je5s) the assembly produced for the following source code.
```rust
#![feature(stdsimd)]
use std::arch::x86_64::{__m128, _mm_set_ss, _mm_store_ss, _mm_rsqrt14_ss};

pub fn q_rsqrt(mut num: f32) -> f32 {
    let mut x: __m128 = unsafe{ _mm_set_ss(num) };

    x = unsafe{ _mm_rsqrt14_ss(x, x) };

    unsafe{ _mm_store_ss(&mut num as *mut _, x) };
    num
}
```
Produces:
```nasm
core::core_arch::x86::avx512f::_mm_rsqrt14_ss:
        mov     rax, rdi
        vmovaps xmm0, xmmword ptr [rsi]
        vrsqrt14ss      xmm0, xmm0, dword ptr [rdx]
        vmovaps xmmword ptr [rdi], xmm0
        ret

example::q_rsqrt:
        sub     rsp, 56
        xorps   xmm1, xmm1
        movss   xmm1, xmm0
        movaps  xmmword ptr [rsp + 16], xmm1
        movaps  xmmword ptr [rsp + 32], xmm1
        mov     rdi, rsp
        lea     rsi, [rsp + 16]
        lea     rdx, [rsp + 32]
        call    core::core_arch::x86::avx512f::_mm_rsqrt14_ss
        movss   xmm0, dword ptr [rsp]
        add     rsp, 56
        ret
```
Anyways, we can see that the intrinsic wasn't outlined, and the function is performing unnecessary loads and stores given that the argument `num` in the first place is stored in the `xmm0` register, ideally it should be a matter of a single instruction `vrsqrt14ss xmm0 xmm0 xmm0`.  For reference, we can compare to equivalent SSE instruction `rsqrtss` and the [assembly output](https://godbolt.org/z/z364heTT8) for that.
```rust
use std::arch::x86_64::{__m128, _mm_set_ss, _mm_store_ss, _mm_rsqrt_ss};

pub fn q_rsqrt(mut num: f32) -> f32 {
    let mut x: __m128 = unsafe{ _mm_set_ss(num) };

    x = unsafe{ _mm_rsqrt_ss(x) };

    unsafe{ _mm_store_ss(&mut num as *mut _, x) };
    num
}
```
Which produces:
```nasm
example::q_rsqrt:
        rsqrtss xmm0, xmm0
        ret
```
And here we see code that is much more in line with what we'ed expect.

Interestingly enough if we use [c and clang](https://godbolt.org/z/c4K7KeP3r) as opposed to rust and rustc, we don't have the nearly all the extraneous parts that we saw before.
```c
#define __AVX512__
#include <xmmintrin.h>
#include <immintrin.h>

float q_rsqrt(float num) {
    __m128 x = _mm_set_ss(num);

    x = _mm_rsqrt14_ss(x, x);

    _mm_store_ss(&num, x);
    return num;
}
```
Which produces:
```nasm
q_rsqrt:                                # @q_rsqrt
        vxorps  xmm1, xmm1, xmm1
        vblendps        xmm0, xmm1, xmm0, 1             # xmm0 = xmm0[0],xmm1[1,2,3]
        vrsqrt14ss      xmm0, xmm0, xmm0
        ret
```
Interestingly, unlike the case of `rsqrtss`, this assembly does actually bother to preserve the upper packed elements of `xmm0` being zeroed out by the `_mm_set_ss(num)` call, despite the fact that it gets reduced back down to a float when returned from the function, making that irrelevant.  This is not just a quirk of c either, `_mm_rsqrt_ss` in c and clang produces the same assembly as its rust equivalent, so it must be something with the semantics of `_mm_rsqrt14_ss`, likely the aspect where it copies over the upper packed floats from the second operand, that changes things somehow.  If I am to speculate, it's likely the case that due to the finicky nature of intrinsics, the compiler is missing an optimization, specifically, as `_mm_set_ss` sets the upper packed floats of `xmm0` to 0, and then we operate on only the single float, then store it once again as a float before `xmm0` gets clobbered, we can skip the whole setting the upper packed floats to 0, and the compiler successfully does this for `_mm_rsqrt_ss`.  However, `_mm_rsqrt14_ss` will copy the upper packed floats from the second argument to the returned value, in this case that does nothing, but as the compiler sees that we are using the upper 3 packed floats of `xmm0` for `_mm_rsqrt14_ss`, it can't elide setting them to 0, despite the fact that it is unnecessary.  That being said, that's just speculation, I don't know for sure.