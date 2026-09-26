use serde::{Deserialize, Serialize};

/// Top-level portable benchmark report.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkReport {
    /// Report schema version.
    pub schema_version: String,
    /// Project identifier.
    pub project: String,
    /// Milestone identifier.
    pub milestone: String,
    /// Environment metadata.
    pub environment: Environment,
    /// Individual benchmark results.
    pub benchmarks: Vec<BenchmarkResult>,
}

/// Reproducibility metadata captured with benchmark results.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    /// Operating system.
    pub os: String,
    /// CPU architecture.
    pub arch: String,
    /// Optional CPU description.
    pub cpu: Option<String>,
    /// Rust compiler version if supplied by the runner.
    pub rust_version: Option<String>,
    /// Build profile.
    pub profile: String,
    /// Optional Git commit.
    pub git_commit: Option<String>,
}

/// One benchmark estimate and optional baseline comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkResult {
    /// Stable benchmark name.
    pub name: String,
    /// Measurement unit, preferably ns.
    pub unit: String,
    /// Lower confidence bound.
    pub low: f64,
    /// Mean or central estimate.
    pub mean: f64,
    /// Upper confidence bound.
    pub high: f64,
    /// Number of statistical outliers when known.
    pub outliers: Option<u32>,
    /// Comparison with a previous milestone.
    pub change: Option<BenchmarkChange>,
}

/// Baseline comparison attached to one result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkChange {
    /// Baseline milestone.
    pub baseline: String,
    /// Relative percentage change; negative means faster.
    pub percent: f64,
    /// Criterion p-value if available.
    pub p_value: Option<f64>,
    /// Human-friendly regression classification.
    pub classification: RegressionClass,
}

/// Performance-change classification used by CI and reports.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RegressionClass {
    Improved,
    Stable,
    Warning,
    Regression,
}
