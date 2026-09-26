use aicp_core::{CandidatePlan, ExecutionPlan};

/// Result of a planning pass including every candidate and the winner.
#[derive(Debug, Clone)]
pub struct PlanningResult {
    /// All considered candidates, including rejected ones.
    pub candidates: Vec<CandidatePlan>,
    /// Selected immutable execution plan.
    pub selected: ExecutionPlan,
}
