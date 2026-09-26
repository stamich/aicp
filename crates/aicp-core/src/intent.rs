use serde::{Deserialize, Serialize};

/// Canonical normalized representation of a user intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentIr {
    /// Human-readable intent name.
    pub name: String,
    /// Dataset or logical resource targeted by the intent.
    pub target: Target,
    /// Desired measurable outcomes.
    pub goals: Goals,
    /// Hard requirements that no selected plan may violate.
    pub constraints: Constraints,
    /// Soft optimization preferences used for ranking feasible plans.
    pub preferences: Preferences,
}

/// Logical resource targeted by an intent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Target {
    /// Dataset name used by the execution engines.
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

/// Hard constraints that define the feasible solution space.
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
    /// Ordered optimization objectives; earlier items receive higher weight.
    pub minimize: Vec<Objective>,
}

/// Durability levels understood by the baseline AICP domain model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Durability {
    /// Strong durability; acknowledged writes must survive the supported failure model.
    Strong,
    /// Relaxed durability for workloads that explicitly accept weaker guarantees.
    Relaxed,
}

/// Optimization objectives supported by the deterministic baseline planner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Objective {
    /// Minimize estimated monetary or abstract infrastructure cost.
    Cost,
    /// Minimize estimated request latency.
    Latency,
    /// Minimize estimated storage footprint.
    Storage,
    /// Minimize estimated CPU consumption.
    Cpu,
    /// Minimize estimated network traffic.
    Network,
}
