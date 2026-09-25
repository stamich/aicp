//! Canonical domain model shared by all AICP crates.
//!
//! This crate intentionally contains no YAML parsing, no engine-specific code and
//! no orchestration. It is the stable intermediate representation used between
//! the input, planning, execution and assurance stages.

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

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

/// Durability levels understood by milestone 0.1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Durability {
    /// Strong durability; acknowledged writes must survive the supported failure model.
    Strong,
    /// Relaxed durability for workloads that explicitly accept weaker guarantees.
    Relaxed,
}

/// Optimization objectives supported by the milestone-0.1 planner.
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

/// Engine identifiers known to AICP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind {
    /// Adaptive Database execution engine.
    AdaptiveDb,
    /// Adaptive Compression Engine.
    Ace,
    /// GraphNet coordination engine.
    GraphNet,
}

impl fmt::Display for EngineKind {
    /// Formats an engine identifier for human-readable output.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::AdaptiveDb => "adaptive-db",
            Self::Ace => "ace",
            Self::GraphNet => "graphnet",
        };
        write!(f, "{value}")
    }
}

/// Physical data strategy selected for AdaptiveDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageStrategy {
    /// Row-oriented layout optimized for point reads and writes.
    Row,
    /// Column-oriented layout optimized for scans and compression.
    Column,
    /// Hybrid layout balancing transactional and analytical access.
    Hybrid,
}

/// Compression policy selected for ACE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionProfile {
    /// Fast encode/decode profile with lower compression ratio.
    Fast,
    /// Balanced profile for general-purpose workloads.
    Balanced,
    /// Storage-efficient profile accepting extra CPU work.
    Dense,
}

/// Coordination strategy selected for GraphNet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationStrategy {
    /// Coordination limited to directly affected entities where permitted.
    Local,
    /// Partition-scoped coordination.
    Partition,
    /// Majority-based coordination suitable for strong guarantees.
    Raft,
    /// Graph-scoped coordination over the affected subgraph.
    GraphScoped,
}

/// One executable operation sent to an engine adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanAction {
    /// Target engine.
    pub engine: EngineKind,
    /// Logical target dataset.
    pub dataset: String,
    /// Typed operation to execute.
    pub operation: EngineOperation,
}

/// Typed operations understood by milestone-0.1 adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineOperation {
    /// Select an AdaptiveDB storage strategy.
    SetStorage { strategy: StorageStrategy },
    /// Select an ACE compression profile.
    SetCompression { profile: CompressionProfile },
    /// Select a GraphNet coordination strategy.
    SetCoordination { strategy: CoordinationStrategy },
}

/// Planner estimates used both for scoring and explainability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanEstimate {
    /// Estimated p99 latency in milliseconds.
    pub p99_latency_ms: f64,
    /// Abstract monthly-cost units used by the 0.1 demo planner.
    pub cost_units: f64,
    /// Abstract storage-footprint units.
    pub storage_units: f64,
    /// Abstract CPU-cost units.
    pub cpu_units: f64,
    /// Abstract network-cost units.
    pub network_units: f64,
    /// Expected availability percentage.
    pub availability_percent: f64,
    /// Whether the plan preserves strong durability.
    pub strong_durability: bool,
}

/// A candidate strategy considered by the planner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidatePlan {
    /// Stable candidate name useful for explanations and tests.
    pub name: String,
    /// Ordered actions required to realize the strategy.
    pub actions: Vec<PlanAction>,
    /// Planner estimates for the candidate.
    pub estimate: PlanEstimate,
    /// Feasibility result after hard-constraint evaluation.
    pub feasible: bool,
    /// Human-readable reasons for rejection, if any.
    pub rejection_reasons: Vec<String>,
    /// Weighted score; lower values are better.
    pub score: Option<f64>,
}

/// Immutable selected plan ready for execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Unique plan identifier.
    pub id: Uuid,
    /// Name of the source intent.
    pub intent_name: String,
    /// Selected candidate name.
    pub strategy_name: String,
    /// Ordered actions to execute.
    pub actions: Vec<PlanAction>,
    /// Planner expectation associated with this plan.
    pub expected: PlanEstimate,
    /// Final planner score.
    pub score: f64,
}

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

/// High-level assurance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceStatus {
    /// All observable required conditions are satisfied.
    Satisfied,
    /// At least one required condition is violated.
    Violated,
    /// Required data is unavailable, so the result cannot be determined safely.
    Unknown,
}

/// Result of comparing observed state with the requested intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssuranceReport {
    /// Overall assurance state.
    pub status: AssuranceStatus,
    /// Human-readable evidence or violations.
    pub reasons: Vec<String>,
    /// Whether the control plane should generate a fresh plan.
    pub recommend_replan: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the display form used by CLI and explanation output.
    #[test]
    fn engine_kind_display_is_stable() {
        assert_eq!(EngineKind::AdaptiveDb.to_string(), "adaptive-db");
        assert_eq!(EngineKind::Ace.to_string(), "ace");
        assert_eq!(EngineKind::GraphNet.to_string(), "graphnet");
    }
}
