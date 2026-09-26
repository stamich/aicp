use serde::{Deserialize, Serialize};

/// Adapter-provided estimate of one action's incremental cost and benefit.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ActionEstimate {
    pub migration_cost_units: f64,
    pub latency_delta_ms: f64,
    pub storage_delta_percent: f64,
    pub confidence: f64,
}
