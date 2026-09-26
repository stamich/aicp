//! Environment compatibility checks.

use aicp_benchmark_model::EnvironmentFingerprint;

/// Compatibility result for two benchmark environments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentCompatibility {
    /// Whether the environments are sufficiently similar for regression classification.
    pub compatible: bool,
    /// Human-readable reasons for incompatibility.
    pub warnings: Vec<String>,
}

/// Checks whether two environments are safe to compare automatically.
pub fn compare_environments(
    baseline: &EnvironmentFingerprint,
    current: &EnvironmentFingerprint,
) -> EnvironmentCompatibility {
    let mut warnings = Vec::new();
    if baseline.os != current.os {
        warnings.push("operating system differs".into());
    }
    if baseline.arch != current.arch {
        warnings.push("architecture differs".into());
    }
    if baseline.build_profile != current.build_profile {
        warnings.push("build profile differs".into());
    }
    if baseline.cpu.is_some() && current.cpu.is_some() && baseline.cpu != current.cpu {
        warnings.push("CPU fingerprint differs".into());
    }
    EnvironmentCompatibility {
        compatible: warnings.is_empty(),
        warnings,
    }
}
