use crate::{error::ExecutorError, receipt::ExecutionReceipt};
use aicp_adapter_api::{AppliedAction, IntentTarget};
use aicp_core::{EngineKind, ExecutionPlan, PlanAction};
use std::collections::HashMap;

/// Executes a plan action-by-action with reverse-order best-effort rollback on failure.
pub fn execute(
    plan: &ExecutionPlan,
    adapters: &[Box<dyn IntentTarget>],
) -> Result<ExecutionReceipt, ExecutorError> {
    let adapters_by_engine = index_adapters(adapters);
    let mut applied = Vec::new();

    for action in &plan.actions {
        let adapter = adapters_by_engine
            .get(&action.engine)
            .copied()
            .ok_or(ExecutorError::MissingAdapter(action.engine))?;
        adapter.validate(action)?;
        match adapter.execute(action) {
            Ok(token) => applied.push((adapter, action, token)),
            Err(error) => {
                rollback(&applied);
                return Err(ExecutorError::Adapter(error));
            }
        }
    }

    Ok(ExecutionReceipt {
        applied_actions: applied.len(),
    })
}

/// Indexes supplied adapters by engine for constant-time action dispatch.
fn index_adapters(adapters: &[Box<dyn IntentTarget>]) -> HashMap<EngineKind, &dyn IntentTarget> {
    adapters
        .iter()
        .map(|adapter| (adapter.engine(), adapter.as_ref()))
        .collect()
}

/// Performs reverse-order best-effort rollback of already applied actions.
fn rollback(applied: &[(&dyn IntentTarget, &PlanAction, AppliedAction)]) {
    for (adapter, action, token) in applied.iter().rev() {
        let _ = adapter.rollback(action, token);
    }
}

#[cfg(test)]
mod tests {
    use super::execute;
    use aicp_adapter_api::IntentTarget;
    use aicp_adapter_mock::{MockAdapter, MockLog};
    use aicp_capability::CapabilityRegistry;
    use aicp_core::{
        CompressionProfile, CoordinationStrategy, EngineKind, EngineOperation, ExecutionPlan,
        PlanAction, PlanEstimate, StorageStrategy,
    };
    use std::sync::Arc;
    use uuid::Uuid;

    #[test]
    /// Verifies a later execution failure rolls back previously applied actions.
    fn rolls_back_on_failure() {
        let registry = CapabilityRegistry::baseline();
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
        assert!(log.entries().iter().any(|entry| entry.starts_with("ROLLBACK")));
    }
}
