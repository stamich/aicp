//! Candidate and executable plan domain model.

use crate::engine::{CompressionProfile, CoordinationStrategy, EngineKind, StorageStrategy};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identity of an individual plan action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub String);

/// One executable operation sent to an engine adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanAction {
    /// Stable action identifier within the plan.
    pub id: ActionId,
    /// Target engine.
    pub engine: EngineKind,
    /// Logical target dataset.
    pub dataset: String,
    /// Typed operation to execute.
    pub operation: EngineOperation,
}

/// Typed operations understood by AICP adapters.
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

/// Planner estimate used for scoring and explainability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanEstimate {
    /// Estimated p99 latency in milliseconds.
    pub p99_latency_ms: f64,
    /// Abstract infrastructure-cost units.
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
    /// Abstract one-time migration/adaptation cost.
    pub migration_cost_units: f64,
}

/// Candidate strategy considered by the planner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidatePlan {
    /// Stable candidate name useful for explanations and tests.
    pub name: String,
    /// Ordered actions required to realize the strategy.
    pub actions: Vec<PlanAction>,
    /// Planner estimate for the candidate.
    pub estimate: PlanEstimate,
    /// Feasibility result after hard-constraint evaluation.
    pub feasible: bool,
    /// Human-readable reasons for rejection.
    pub rejection_reasons: Vec<String>,
    /// Weighted score; lower is better.
    pub score: Option<f64>,
}

/// Immutable selected plan ready for execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    /// Unique plan identifier.
    pub id: Uuid,
    /// Source intent name.
    pub intent_name: String,
    /// Source intent revision.
    pub intent_revision: u64,
    /// Selected candidate name.
    pub strategy_name: String,
    /// Ordered actions to execute.
    pub actions: Vec<PlanAction>,
    /// Planner expectation associated with this plan.
    pub expected: PlanEstimate,
    /// Final planner score.
    pub score: f64,
    /// Deterministic fingerprint of intent, state and plan actions.
    pub fingerprint: String,
}
