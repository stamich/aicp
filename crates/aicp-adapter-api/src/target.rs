use crate::{AdapterError, AppliedAction};
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, PlanAction};

/// Abstraction implemented by every AICP execution target.
pub trait IntentTarget: Send + Sync {
    /// Returns the engine represented by this adapter.
    fn engine(&self) -> EngineKind;

    /// Returns capabilities currently supported by the engine.
    fn capabilities(&self) -> EngineCapabilities;

    /// Validates an action without changing engine state.
    fn validate(&self, action: &PlanAction) -> Result<(), AdapterError>;

    /// Applies an action and returns a rollback/audit token.
    fn execute(&self, action: &PlanAction) -> Result<AppliedAction, AdapterError>;

    /// Performs best-effort rollback for a previously applied action.
    fn rollback(
        &self,
        action: &PlanAction,
        applied: &AppliedAction,
    ) -> Result<(), AdapterError>;
}
