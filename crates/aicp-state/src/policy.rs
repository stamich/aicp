use serde::{Deserialize, Serialize};

/// Policy preventing rapid oscillation between nearly equivalent plans.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AdaptationPolicy {
    pub minimum_improvement_ratio: f64,
    pub cooldown_seconds: u64,
}

impl Default for AdaptationPolicy {
    fn default() -> Self { Self { minimum_improvement_ratio: 0.10, cooldown_seconds: 60 } }
}
