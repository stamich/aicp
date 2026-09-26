use serde::{Deserialize, Serialize};

/// Runtime telemetry used by assurance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TelemetrySnapshot {
    pub p99_latency_ms: Option<f64>,
    pub availability_percent: Option<f64>,
    pub strong_durability: Option<bool>,
    pub cost_units: Option<f64>,
}
