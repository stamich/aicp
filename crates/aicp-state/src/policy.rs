//! Adaptation stabilization policy.

use serde::{Deserialize, Serialize};

/// Policy preventing rapid oscillation between nearly equivalent plans.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AdaptationPolicy {
    /// Minimum relative score improvement required before switching plans.
    pub minimum_improvement_ratio: f64,
    /// Minimum number of seconds between adaptations.
    pub cooldown_seconds: u64,
}

impl Default for AdaptationPolicy {
    /// Returns conservative defaults suitable for the demo environment.
    fn default() -> Self {
        Self { minimum_improvement_ratio: 0.10, cooldown_seconds: 60 }
    }
}
