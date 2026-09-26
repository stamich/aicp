//! Performance regression classification and scale sanity checks.

use aicp_benchmark_model::PerformanceClassification;

/// Classifies a percentage change after statistical and scale sanity checks.
pub fn classify(
    change_percent: f64,
    p_value: Option<f64>,
    suspicious_scale: bool,
    compatible: bool,
) -> PerformanceClassification {
    if !compatible {
        return PerformanceClassification::Invalid;
    }
    if suspicious_scale {
        return PerformanceClassification::Suspicious;
    }
    if p_value.is_some_and(|p| p >= 0.05) {
        return PerformanceClassification::Stable;
    }
    if change_percent <= -5.0 {
        PerformanceClassification::Improved
    } else if change_percent <= 5.0 {
        PerformanceClassification::Stable
    } else if change_percent <= 10.0 {
        PerformanceClassification::Warning
    } else {
        PerformanceClassification::Regression
    }
}

/// Detects implausible order-of-magnitude jumps that often indicate a unit conversion bug.
pub fn suspicious_scale_change(baseline_ns: f64, current_ns: f64) -> bool {
    if baseline_ns <= 0.0 || current_ns <= 0.0 {
        return true;
    }
    let ratio = current_ns / baseline_ns;
    ratio < 0.01 || ratio > 100.0
}
