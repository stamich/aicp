use aicp_core::{CandidatePlan, ExecutionPlan};

/// Full planning result including rejected candidates.
#[derive(Debug, Clone)]
pub struct PlanningResult { pub candidates: Vec<CandidatePlan>, pub selected: ExecutionPlan }
