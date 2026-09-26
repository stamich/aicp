//! Action estimate model.

use serde::{Deserialize, Serialize};

/// Adapter-provided estimate of one action's incremental cost and benefit.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ActionEstimate {
    /// Estimated migration/adaptation cost units.
    pub migration_cost_units: f64,
    /// Estimated latency delta in milliseconds; negative means improvement.
    pub latency_delta_ms: f64,
    /// Estimated storage delta percentage.
    pub storage_delta_percent: f64,
    /// Confidence in the estimate from 0.0 to 1.0.
    pub confidence: f64,
}
