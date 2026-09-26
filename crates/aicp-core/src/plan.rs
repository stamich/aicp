use crate::engine::{EngineKind, EngineOperation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Stable identity of an individual plan action.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(pub String);

/// One executable operation sent to an engine adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanAction {
    pub id: ActionId,
    pub engine: EngineKind,
    pub dataset: String,
    pub operation: EngineOperation,
}

/// Planner estimate used for scoring and explainability.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlanEstimate {
    pub p99_latency_ms: f64,
    pub cost_units: f64,
    pub storage_units: f64,
    pub cpu_units: f64,
    pub network_units: f64,
    pub availability_percent: f64,
    pub strong_durability: bool,
    pub migration_cost_units: f64,
}

/// Candidate strategy considered by the planner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidatePlan {
    pub name: String,
    pub actions: Vec<PlanAction>,
    pub estimate: PlanEstimate,
    pub feasible: bool,
    pub rejection_reasons: Vec<String>,
    pub score: Option<f64>,
}

/// Immutable selected plan ready for execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionPlan {
    pub id: Uuid,
    pub intent_name: String,
    pub intent_revision: u64,
    pub strategy_name: String,
    pub actions: Vec<PlanAction>,
    pub expected: PlanEstimate,
    pub score: f64,
    pub fingerprint: String,
}
