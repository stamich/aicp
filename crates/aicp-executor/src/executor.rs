//! Idempotent plan execution and rollback orchestration.

use crate::error::ExecutorError;
use aicp_adapter_api::IntentTarget;
use aicp_core::{EngineKind, ExecutionPlan};
use aicp_plan::ExecutionReceipt;
use std::collections::HashMap;

/// Executes an immutable plan against registered adapters.
pub fn execute_plan(
    plan: &ExecutionPlan,
    adapters: &mut HashMap<EngineKind, Box<dyn IntentTarget>>,
) -> Result<Vec<ExecutionReceipt>, ExecutorError> {
    let mut receipts = Vec::new();
    for action in &plan.actions {
        let validation = {
            let adapter = adapters
                .get_mut(&action.engine)
                .ok_or(ExecutorError::MissingAdapter(action.engine))?;
            adapter.validate(action)?
        };
        if !validation.allowed {
            rollback_all(adapters, &receipts);
            return Err(ExecutorError::Validation(validation.reason));
        }
        let execution = {
            let adapter = adapters
                .get_mut(&action.engine)
                .ok_or(ExecutorError::MissingAdapter(action.engine))?;
            adapter.execute(&plan.id.to_string(), action)
        };
        match execution {
            Ok(receipt) => receipts.push(receipt),
            Err(error) => {
                rollback_all(adapters, &receipts);
                return Err(ExecutorError::Adapter(error));
            }
        }
    }
    Ok(receipts)
}

/// Rolls back already applied actions in reverse order and ignores secondary rollback failures.
fn rollback_all(
    adapters: &mut HashMap<EngineKind, Box<dyn IntentTarget>>,
    receipts: &[ExecutionReceipt],
) {
    for receipt in receipts.iter().rev() {
        if let Some(adapter) = adapters.get_mut(&receipt.engine) {
            let _ = adapter.rollback(receipt);
        }
    }
}
