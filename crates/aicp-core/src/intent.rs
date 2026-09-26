//! Canonical intent-domain model.

use serde::{Deserialize, Serialize};

/// Canonical normalized representation of a user intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentIr {
    /// Human-readable intent name.
    pub name: String,
    /// Monotonically increasing revision used for idempotency and plan fingerprints.
    pub revision: u64,
    /// Dataset or logical resource targeted by the intent.
    pub target: Target,
    /// Desired measurable outcomes.
    pub goals: Goals,
    /// Hard requirements that no selected plan may violate.
    pub constraints: Constraints,
    /// Soft optimization preferences used to rank feasible plans.
    pub preferences: Preferences,
}

/// Logical resource targeted by an intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    /// Dataset name used by execution engines.
    pub dataset: String,
}

/// Measurable outcomes requested by the user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Goals {
    /// Maximum allowed p99 latency in milliseconds.
    pub max_p99_latency_ms: Option<u64>,
    /// Minimum required availability percentage.
    pub min_availability_percent: Option<f64>,
}

/// Hard constraints defining the feasible solution space.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Constraints {
    /// Required durability level.
    pub durability: Option<Durability>,
    /// Allowed data residency regions or jurisdictions.
    pub residency: Vec<String>,
}

/// Soft objectives used to rank plans that already satisfy hard constraints.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preferences {
    /// Ordered optimization objectives; earlier objectives receive higher weight.
    pub minimize: Vec<Objective>,
}

/// Durability levels understood by AICP 0.3.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Durability {
    /// Acknowledged writes must survive the supported failure model.
    Strong,
    /// Weaker guarantee accepted explicitly by the intent owner.
    Relaxed,
}

/// Optimization objectives supported by the planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Objective {
    /// Minimize monetary or abstract infrastructure cost.
    Cost,
    /// Minimize request latency.
    Latency,
    /// Minimize storage footprint.
    Storage,
    /// Minimize CPU consumption.
    Cpu,
    /// Minimize network traffic.
    Network,
    /// Minimize adaptation or migration overhead.
    Migration,
}
