use aicp_core::EngineKind;
use thiserror::Error;

/// Planner failures that prevent an execution plan from being selected.
#[derive(Debug, Error)]
pub enum PlannerError {
    /// No feasible candidate satisfies the hard constraints.
    #[error("no feasible plan satisfies the intent constraints")]
    NoFeasiblePlan,
    /// Required engine capabilities are missing.
    #[error("missing capabilities for engine {0}")]
    MissingCapabilities(EngineKind),
}
