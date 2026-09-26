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
pub struct Target { pub dataset: String }

/// Measurable outcomes requested by the user.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Goals {
    pub max_p99_latency_ms: Option<u64>,
    pub min_availability_percent: Option<f64>,
}

/// Hard constraints defining the feasible solution space.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Constraints {
    pub durability: Option<Durability>,
    pub residency: Vec<String>,
}

/// Soft objectives used to rank feasible plans.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Preferences { pub minimize: Vec<Objective> }

/// Durability levels understood by AICP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Durability { Strong, Relaxed }

/// Optimization objectives supported by the planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Objective { Cost, Latency, Storage, Cpu, Network, Migration }
