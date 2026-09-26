use aicp_capability::CapabilityRegistry;
use aicp_core::TelemetrySnapshot;

/// Returns the deterministic capability registry shared by benchmark scenarios.
pub fn benchmark_registry() -> CapabilityRegistry {
    CapabilityRegistry::baseline()
}

/// Returns the deterministic telemetry snapshot shared by benchmark scenarios.
pub fn benchmark_telemetry() -> TelemetrySnapshot {
    TelemetrySnapshot {
        p99_latency_ms: Some(9.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(100.0),
    }
}
