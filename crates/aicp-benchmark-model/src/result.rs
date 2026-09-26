//! Individual benchmark result and baseline comparison.

use crate::{
    classification::PerformanceClassification,
    estimate::{NormalizedEstimate, RawEstimate},
};
use serde::{Deserialize, Serialize};

/// One benchmark result in schema 1.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkResult {
    /// Stable benchmark identifier.
    pub name: String,
    /// Raw value from the source artifact.
    pub raw: RawEstimate,
    /// Canonical nanosecond value.
    pub normalized: NormalizedEstimate,
    /// Optional baseline comparison.
    pub comparison: Option<BenchmarkComparison>,
}

/// Comparison of one measurement with its baseline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkComparison {
    /// Baseline milestone identifier.
    pub baseline: String,
    /// Relative change in percent; negative means faster.
    pub percent: f64,
    /// Optional p-value if provided by the source benchmark tool.
    pub p_value: Option<f64>,
    /// Classification after sanity checks.
    pub classification: PerformanceClassification,
    /// Warnings that affect interpretation.
    pub warnings: Vec<String>,
}
