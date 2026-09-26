//! Portable top-level benchmark report.

use crate::{environment::EnvironmentFingerprint, result::BenchmarkResult};
use serde::{Deserialize, Serialize};

/// Top-level portable AICP benchmark report schema 1.1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkReport {
    /// Schema version; milestone 0.3.1 emits `1.1`.
    pub schema_version: String,
    /// Project name.
    pub project: String,
    /// Milestone identifier.
    pub milestone: String,
    /// Environment fingerprint.
    pub environment: EnvironmentFingerprint,
    /// Individual measurements.
    pub benchmarks: Vec<BenchmarkResult>,
}
