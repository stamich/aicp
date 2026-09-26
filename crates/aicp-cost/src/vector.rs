//! Raw and normalized cost vectors.

/// Multi-dimensional cost representation used to compare cross-engine plans.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CostVector {
    /// End-to-end p99 latency in milliseconds.
    pub latency_ms: f64,
    /// Abstract CPU cost units.
    pub cpu_units: f64,
    /// Abstract memory cost units.
    pub memory_units: f64,
    /// Abstract storage cost units.
    pub storage_units: f64,
    /// Abstract network cost units.
    pub network_units: f64,
    /// Abstract monetary/infrastructure cost units.
    pub monetary_units: f64,
    /// One-time adaptation or migration cost.
    pub migration_units: f64,
}

/// Normalized dimensionless form of a cost vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NormalizedCostVector {
    /// Normalized latency.
    pub latency: f64,
    /// Normalized CPU.
    pub cpu: f64,
    /// Normalized memory.
    pub memory: f64,
    /// Normalized storage.
    pub storage: f64,
    /// Normalized network.
    pub network: f64,
    /// Normalized monetary cost.
    pub monetary: f64,
    /// Normalized migration cost.
    pub migration: f64,
}
