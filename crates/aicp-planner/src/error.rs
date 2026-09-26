use aicp_core::EngineKind;
use thiserror::Error;

/// Planner failures that cannot safely be converted into a plan.
#[derive(Debug, Error)]
pub enum PlannerError {
    #[error("missing capabilities for {0}")] MissingCapabilities(EngineKind),
    #[error("no feasible plan")] NoFeasiblePlan,
}
