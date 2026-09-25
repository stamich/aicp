//! Execution orchestration for immutable AICP plans.

use aicp_adapter_api::{AdapterError, AppliedAction, IntentTarget};
use aicp_core::{EngineKind, ExecutionPlan, PlanAction};
use std::collections::HashMap;
use thiserror::Error;

/// Successful execution receipt.
#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    /// Number of actions successfully applied.
    pub applied_actions: usize,
}

/// Errors raised while validating or applying a plan.
#[derive(Debug, Error)]
pub enum ExecutorError {
    /// No adapter was supplied for a required engine.
    #[error("missing adapter for {0}")]
    MissingAdapter(EngineKind),
    /// Adapter validation or execution failed. Best-effort rollback is attempted first.
    #[error("adapter failure: {0}")]
    Adapter(#[from] AdapterError),
}

/// Executes a plan action-by-action with reverse-order best-effort rollback on failure.
pub fn execute(
    plan: &ExecutionPlan,
    adapters: &[Box<dyn IntentTarget>],
) -> Result<ExecutionReceipt, ExecutorError> {
    let map: HashMap<EngineKind, &dyn IntentTarget> =
        adapters.iter().map(|a| (a.engine(), a.as_ref())).collect();
    let mut applied: Vec<(&dyn IntentTarget, &PlanAction, AppliedAction)> = Vec::new();
    for action in &plan.actions {
        let adapter = map
            .get(&action.engine)
            .copied()
            .ok_or(ExecutorError::MissingAdapter(action.engine))?;
        adapter.validate(action)?;
        match adapter.execute(action) {
            Ok(token) => applied.push((adapter, action, token)),
            Err(error) => {
                for (a, act, token) in applied.iter().rev() {
                    let _ = a.rollback(act, token);
                }
                return Err(ExecutorError::Adapter(error));
            }
        }
    }
    Ok(ExecutionReceipt {
        applied_actions: applied.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aicp_adapter_mock::{MockAdapter, MockLog};
    use aicp_capability::CapabilityRegistry;
    use aicp_core::{
        CompressionProfile, CoordinationStrategy, EngineOperation, PlanEstimate, StorageStrategy,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    /// Verifies already-applied actions are rolled back when a later engine fails.
    #[test]
    fn rolls_back_on_failure() {
        let registry = CapabilityRegistry::milestone_0_1();
        let log = Arc::new(MockLog::default());
        let adapters: Vec<Box<dyn IntentTarget>> = vec![
            Box::new(MockAdapter::new(
                EngineKind::AdaptiveDb,
                registry.get(EngineKind::AdaptiveDb).unwrap().clone(),
                Arc::clone(&log),
                false,
            )),
            Box::new(MockAdapter::new(
                EngineKind::Ace,
                registry.get(EngineKind::Ace).unwrap().clone(),
                Arc::clone(&log),
                true,
            )),
            Box::new(MockAdapter::new(
                EngineKind::GraphNet,
                registry.get(EngineKind::GraphNet).unwrap().clone(),
                Arc::clone(&log),
                false,
            )),
        ];
        let plan = ExecutionPlan {
            id: Uuid::new_v4(),
            intent_name: "x".into(),
            strategy_name: "t".into(),
            score: 0.0,
            expected: PlanEstimate {
                p99_latency_ms: 1.0,
                cost_units: 1.0,
                storage_units: 1.0,
                cpu_units: 1.0,
                network_units: 1.0,
                availability_percent: 99.0,
                strong_durability: true,
            },
            actions: vec![
                PlanAction {
                    engine: EngineKind::AdaptiveDb,
                    dataset: "d".into(),
                    operation: EngineOperation::SetStorage {
                        strategy: StorageStrategy::Row,
                    },
                },
                PlanAction {
                    engine: EngineKind::Ace,
                    dataset: "d".into(),
                    operation: EngineOperation::SetCompression {
                        profile: CompressionProfile::Fast,
                    },
                },
                PlanAction {
                    engine: EngineKind::GraphNet,
                    dataset: "d".into(),
                    operation: EngineOperation::SetCoordination {
                        strategy: CoordinationStrategy::Raft,
                    },
                },
            ],
        };
        assert!(execute(&plan, &adapters).is_err());
        assert!(log.entries().iter().any(|e| e.starts_with("ROLLBACK")));
    }
}
