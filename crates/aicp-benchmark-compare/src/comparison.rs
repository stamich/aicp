//! Comparison of normalized measurements against a baseline.

use crate::classification::{classify, suspicious_scale_change};
use aicp_benchmark_model::{BenchmarkComparison, NormalizedEstimate};

/// Compares normalized estimates and returns a fully classified comparison.
pub fn compare_measurement(
    baseline_milestone: &str,
    baseline: &NormalizedEstimate,
    current: &NormalizedEstimate,
    p_value: Option<f64>,
    compatible: bool,
) -> BenchmarkComparison {
    let percent = (current.mean_ns - baseline.mean_ns) / baseline.mean_ns * 100.0;
    let suspicious = suspicious_scale_change(baseline.mean_ns, current.mean_ns);
    let mut warnings = Vec::new();
    if suspicious {
        warnings.push("measurement changed by more than two orders of magnitude; verify units and benchmark body".into());
    }
    if !compatible {
        warnings.push("environment is not compatible with baseline".into());
    }
    BenchmarkComparison {
        baseline: baseline_milestone.into(),
        percent,
        p_value,
        classification: classify(percent, p_value, suspicious, compatible),
        warnings,
    }
}
