use crate::error::AdapterError;
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt};
use aicp_state::ObservedState;

/// Contract between the control plane and one execution engine.
pub trait IntentTarget {
    fn kind(&self) -> EngineKind;
    fn capabilities(&self) -> Result<EngineCapabilities, AdapterError>;
    fn observe(&self) -> Result<ObservedState, AdapterError>;
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError>;
    fn estimate(&self, action: &PlanAction) -> Result<ActionEstimate, AdapterError>;
    fn execute(
        &mut self,
        plan_id: &str,
        action: &PlanAction,
    ) -> Result<ExecutionReceipt, AdapterError>;
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError>;
}
