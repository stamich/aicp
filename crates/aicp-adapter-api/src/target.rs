//! Engine adapter contract.

use crate::error::AdapterError;
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt};
use aicp_state::ObservedState;

/// Contract between the control plane and one execution engine.
pub trait IntentTarget {
    /// Returns the engine implemented by this adapter.
    fn kind(&self) -> EngineKind;
    /// Discovers versioned capabilities dynamically.
    fn capabilities(&self) -> Result<EngineCapabilities, AdapterError>;
    /// Observes normalized current engine state.
    fn observe(&self) -> Result<ObservedState, AdapterError>;
    /// Validates one action without changing engine state.
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError>;
    /// Estimates incremental adaptation cost and expected impact.
    fn estimate(&self, action: &PlanAction) -> Result<ActionEstimate, AdapterError>;
    /// Applies an action idempotently and returns an immutable receipt.
    fn execute(
        &mut self,
        plan_id: &str,
        action: &PlanAction,
    ) -> Result<ExecutionReceipt, AdapterError>;
    /// Best-effort rollback of a previously applied action.
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError>;
}
