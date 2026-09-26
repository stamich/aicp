use crate::engine::{EngineKind, EngineOperation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// Planner estimates used both for scoring and explainability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanEstimate {
    /// Estimated p99 latency in milliseconds.
    pub p99_latency_ms: f64,
    /// Abstract monthly-cost units used by the demo planner.
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
