//! Benchmark environment fingerprint.

use serde::{Deserialize, Serialize};

/// Runtime and tool metadata used to judge reproducibility.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentFingerprint {
    /// Operating system.
    pub os: String,
    /// CPU architecture.
    pub arch: String,
    /// Optional CPU model.
    pub cpu: Option<String>,
    /// Number of logical CPUs when available.
    pub logical_cpus: Option<usize>,
    /// Rust compiler version.
    pub rust_version: Option<String>,
    /// Cargo version.
    pub cargo_version: Option<String>,
    /// Build profile.
    pub build_profile: String,
    /// Optional Git commit.
    pub git_commit: Option<String>,
    /// Optional target triple.
    pub target_triple: Option<String>,
    /// Criterion version when known.
    pub criterion_version: Option<String>,
}
