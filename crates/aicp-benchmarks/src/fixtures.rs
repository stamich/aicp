use aicp_core::{Constraints, Durability, EngineKind, Goals, IntentIr, Objective, Preferences, StorageStrategy, Target};
use aicp_state::{DatasetState, EngineHealth, ObservedState, ResourceSnapshot};

/// Creates the representative benchmark intent shared by benchmark families.
pub fn sample_intent() -> IntentIr {
    IntentIr { name: "orders".into(), revision: 2, target: Target { dataset: "orders".into() }, goals: Goals { max_p99_latency_ms: Some(10), min_availability_percent: Some(99.99) }, constraints: Constraints { durability: Some(Durability::Strong), residency: vec!["EU".into()] }, preferences: Preferences { minimize: vec![Objective::Cost, Objective::Latency, Objective::Migration] } }
}

/// Creates the representative observed state shared by benchmark families.
pub fn sample_observed_state() -> ObservedState {
    ObservedState { engine: EngineKind::AdaptiveDb, version: "bench".into(), health: EngineHealth::Healthy, resources: ResourceSnapshot { cpu_percent: 20.0, memory_percent: 20.0, storage_percent: 20.0 }, datasets: vec![DatasetState { name: "orders".into(), storage_strategy: Some(StorageStrategy::Column), estimated_rows: 1_000_000, size_bytes: 512*1024*1024, p99_latency_ms: Some(18.0) }], sequence: 1 }
}
