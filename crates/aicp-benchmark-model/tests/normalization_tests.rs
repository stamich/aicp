use aicp_benchmark_model::{RawEstimate, TimeUnit};

#[test]
fn microseconds_normalize_to_ns() {
    let raw = RawEstimate {
        low: 15.0,
        mean: 15.769,
        high: 16.0,
        unit: TimeUnit::Microseconds,
    };
    assert!((raw.normalize_to_ns().mean_ns - 15_769.0).abs() < 1e-9);
}
