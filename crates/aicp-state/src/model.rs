use aicp_core::{EngineKind, StorageStrategy};
use serde::{Deserialize, Serialize};

/// Health reported by an execution engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineHealth { Healthy, Degraded, Unavailable }

/// Resource snapshot reported by an engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSnapshot { pub cpu_percent: f64, pub memory_percent: f64, pub storage_percent: f64 }

/// State of one managed dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetState {
    pub name: String,
    pub storage_strategy: Option<StorageStrategy>,
    pub estimated_rows: u64,
    pub size_bytes: u64,
    pub p99_latency_ms: Option<f64>,
}

/// Normalized actual state of one execution engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedState {
    pub engine: EngineKind,
    pub version: String,
    pub health: EngineHealth,
    pub resources: ResourceSnapshot,
    pub datasets: Vec<DatasetState>,
    pub sequence: u64,
}
