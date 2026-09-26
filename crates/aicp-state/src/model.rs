//! Observed-state and drift domain model.

use aicp_core::{EngineKind, StorageStrategy};
use serde::{Deserialize, Serialize};

/// Health reported by an execution engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineHealth {
    /// Engine is ready to serve control-plane requests.
    Healthy,
    /// Engine is reachable but impaired.
    Degraded,
    /// Engine is unavailable.
    Unavailable,
}

/// Resource snapshot reported by an engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceSnapshot {
    /// Approximate CPU utilization percentage.
    pub cpu_percent: f64,
    /// Approximate memory utilization percentage.
    pub memory_percent: f64,
    /// Approximate storage utilization percentage.
    pub storage_percent: f64,
}

/// State of one managed dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetState {
    /// Dataset name.
    pub name: String,
    /// Current AdaptiveDB layout if known.
    pub storage_strategy: Option<StorageStrategy>,
    /// Approximate row count.
    pub estimated_rows: u64,
    /// Approximate on-disk bytes.
    pub size_bytes: u64,
    /// Observed p99 latency in milliseconds.
    pub p99_latency_ms: Option<f64>,
}

/// Normalized actual state of one execution engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObservedState {
    /// Engine reporting the state.
    pub engine: EngineKind,
    /// Engine implementation version.
    pub version: String,
    /// Current health.
    pub health: EngineHealth,
    /// Current resources.
    pub resources: ResourceSnapshot,
    /// Managed datasets relevant to AICP.
    pub datasets: Vec<DatasetState>,
    /// Monotonic observation sequence supplied by the adapter.
    pub sequence: u64,
}

/// Kinds of drift recognized by milestone 0.3.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Drift {
    /// A measurable intent objective is violated.
    IntentViolation { detail: String },
    /// Actual physical configuration differs from the expected plan.
    ConfigurationDrift { detail: String },
    /// Previously available engine capability disappeared or changed.
    CapabilityDrift { detail: String },
    /// Performance moved materially away from the expected range.
    PerformanceDrift { detail: String },
}
