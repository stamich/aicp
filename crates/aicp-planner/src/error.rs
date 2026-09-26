//! Planner error model.

use aicp_core::EngineKind;
use thiserror::Error;

/// Planner failures that cannot safely be converted into an execution plan.
#[derive(Debug, Error)]
pub enum PlannerError {
    /// Required engine capabilities are missing.
    #[error("missing capabilities for {0}")]
    MissingCapabilities(EngineKind),
    /// No candidate satisfies all hard constraints and budgets.
    #[error("no feasible plan")]
    NoFeasiblePlan,
}
