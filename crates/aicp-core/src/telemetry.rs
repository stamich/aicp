//! Runtime telemetry model.

use serde::{Deserialize, Serialize};

/// Runtime telemetry used by assurance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    /// Observed p99 latency in milliseconds.
    pub p99_latency_ms: Option<f64>,
    /// Observed availability percentage.
    pub availability_percent: Option<f64>,
    /// Whether runtime durability is known to be strong.
    pub strong_durability: Option<bool>,
    /// Observed abstract cost units.
    pub cost_units: Option<f64>,
}
