use rsqrt_test::{operation_tester::OpTester, quake_rsqrt::QuakeRSqrt, QRSqrt, basic_rsqrt::BasicRSqrt};

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "sse"
))]
use rsqrt_test::sse_rsqrt::SSERSqrt;

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    feature = "nightly"
))]
use rsqrt_test::avx512_rsqrt::AVX512RSqrt;

#[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
use rsqrt_test::asm_avx512_rsqrt::AsmAVX512RSqrt;


fn run_test<T: QRSqrt, const N: usize>(op_tester: &mut OpTester<N>) {
    op_tester.run_q_sqrt_seq::<T>();
    let err = op_tester.calculate_error().unwrap();
    let avg = (err.iter().copied().map(|val| val as f64).sum::<f64>() / err.len() as f64) as f32;
    let max = err.iter().copied().reduce(f32::max).unwrap();

    let nfth: f64 = (err.len() as f64) * 0.95;
    let nnth: f64 = (err.len() as f64) * 0.99;

    // get the 95th precentile, if the 95th precentile lies between 2 values, linearly interpolate
    // to get result
    let (_, mid, end) =
        err.select_nth_unstable_by(nfth.floor() as usize, |x, y| x.partial_cmp(y).unwrap());
    let err95 = if nfth.fract() == 0.0f64 {
        *mid as f64
    } else {
        let ratio = nfth.fract();
        *mid as f64 * ratio
            + end.iter().copied().reduce(f32::min).unwrap_or(*mid) as f64 * (1.0 - ratio)
    };

    let (_, mid, end) =
        err.select_nth_unstable_by(nnth.floor() as usize, |x, y| x.partial_cmp(y).unwrap());
    let err99 = if nnth.fract() == 0.0f64 {
        *mid as f64
    } else {
        let ratio = nnth.fract();
        *mid as f64 * ratio
            + end.iter().copied().reduce(f32::min).unwrap_or(*mid) as f64 * (1.0 - ratio)
    };

    println!(
        "{:>14.7e} {:>14.7e} {:>14.7e} {:>14.7e}",
        avg, max, err95, err99
    );
}

macro_rules! test_seq {
    ($q_rsqrt_method:ty, $ex:expr, $n:expr) => {
        let name = if stringify!($q_rsqrt_method).len() > 20 {
            &stringify!($q_rsqrt_method)[..20]
        } else {
            stringify!($q_rsqrt_method)
        };
        print!("    {:>20}: ", name);
        let optester = $ex;
        run_test::<$q_rsqrt_method, $n>(optester);
    };
}

fn main() {
    const NUMBER: usize = 1usize << 24;

    let base: f32 = 100000.0f32.powf(1.0f32 / ((NUMBER - 1) as f32));
    let iter = (0..).map(|i| 0.001f32 * base.powf(i as f32));
    let mut opt = OpTester::<NUMBER>::new(iter);


    println!(
        "Tests for different implementations of `q_rsqrt` (approximate reciprocal square root):"
    );
    println!(
        "Method Name               Average Error  Maximum Error  95 Percentile  99 Percentile"
    );
    test_seq!(BasicRSqrt, &mut opt, NUMBER);
    test_seq!(QuakeRSqrt, &mut opt, NUMBER);
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "sse"
    ))]
    test_seq!(SSERSqrt, &mut opt, NUMBER);
    
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        feature = "nightly"
    ))]
    test_seq!(AVX512RSqrt, &mut opt, NUMBER);
    #[cfg(all(target_arch = "x86_64", target_feature = "avx512f"))]
    test_seq!(AsmAVX512RSqrt, &mut opt, NUMBER);

}
