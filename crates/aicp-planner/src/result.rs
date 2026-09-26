//! Planner result model.

use aicp_core::{CandidatePlan, ExecutionPlan};
use aicp_decision::DecisionGraph;

/// Full planning result including alternatives and a decision graph.
#[derive(Debug, Clone)]
pub struct PlanningResult {
    /// All candidates retained after the cheap pruning stage.
    pub candidates: Vec<CandidatePlan>,
    /// Selected immutable execution plan.
    pub selected: ExecutionPlan,
    /// Structured explanation of the decision.
    pub decision_graph: DecisionGraph,
}
