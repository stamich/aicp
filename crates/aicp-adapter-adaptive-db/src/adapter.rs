use crate::{client::AdaptiveDbClient, serialization::state_fingerprint};
use aicp_adapter_api::{AdapterError, IntentTarget};
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, EngineOperation, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt, ExecutionStatus};
use aicp_state::{EngineHealth, ObservedState, ResourceSnapshot};
use std::collections::HashSet;

/// AICP adapter translating generic actions to the AdaptiveDB client contract.
pub struct AdaptiveDbAdapter<C: AdaptiveDbClient> {
    client: C,
    applied: HashSet<String>,
}

impl<C: AdaptiveDbClient> AdaptiveDbAdapter<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
            applied: HashSet::new(),
        }
    }
    pub fn client(&self) -> &C {
        &self.client
    }
}

impl<C: AdaptiveDbClient> IntentTarget for AdaptiveDbAdapter<C> {
    fn kind(&self) -> EngineKind {
        EngineKind::AdaptiveDb
    }
    fn capabilities(&self) -> Result<EngineCapabilities, AdapterError> {
        Ok(EngineCapabilities::AdaptiveDb {
            version: self.client.version().map_err(AdapterError::Unavailable)?,
            storage: self
                .client
                .storage_capabilities()
                .map_err(AdapterError::Unavailable)?,
            projections: false,
        })
    }
    fn observe(&self) -> Result<ObservedState, AdapterError> {
        Ok(ObservedState {
            engine: EngineKind::AdaptiveDb,
            version: self.client.version().map_err(AdapterError::Unavailable)?,
            health: EngineHealth::Healthy,
            resources: ResourceSnapshot {
                cpu_percent: 42.0,
                memory_percent: 36.0,
                storage_percent: 51.0,
            },
            datasets: self
                .client
                .observe_datasets()
                .map_err(AdapterError::Unavailable)?,
            sequence: 1,
        })
    }
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError> {
        if action.engine != EngineKind::AdaptiveDb {
            return Ok(ActionValidation::reject(
                "action targets a different engine",
            ));
        }
        match &action.operation {
            EngineOperation::SetStorage { strategy } => {
                let supported = self
                    .client
                    .storage_capabilities()
                    .map_err(AdapterError::Unavailable)?
                    .contains(strategy);
                Ok(if supported {
                    ActionValidation::allow("storage strategy supported")
                } else {
                    ActionValidation::reject("storage strategy unsupported")
                })
            }
            _ => Ok(ActionValidation::reject(
                "operation is not an AdaptiveDB storage action",
            )),
        }
    }
    fn estimate(&self, action: &PlanAction) -> Result<ActionEstimate, AdapterError> {
        match &action.operation {
            EngineOperation::SetStorage { strategy } => self
                .client
                .estimate_storage_change(&action.dataset, *strategy)
                .map_err(AdapterError::Execution),
            _ => Err(AdapterError::Unsupported("not an AdaptiveDB action".into())),
        }
    }
    fn execute(
        &mut self,
        plan_id: &str,
        action: &PlanAction,
    ) -> Result<ExecutionReceipt, AdapterError> {
        let key = format!("{}:{}", plan_id, action.id.0);
        if self.applied.contains(&key) {
            return Ok(ExecutionReceipt {
                plan_id: plan_id.into(),
                action_id: action.id.clone(),
                engine: EngineKind::AdaptiveDb,
                status: ExecutionStatus::AlreadyApplied,
                previous_state: None,
                resulting_state: None,
            });
        }
        let before = state_fingerprint(&self.observe()?);
        match &action.operation {
            EngineOperation::SetStorage { strategy } => self
                .client
                .set_storage(&action.dataset, *strategy)
                .map_err(AdapterError::Execution)?,
            _ => return Err(AdapterError::Unsupported("not an AdaptiveDB action".into())),
        }
        self.applied.insert(key);
        let after = state_fingerprint(&self.observe()?);
        Ok(ExecutionReceipt {
            plan_id: plan_id.into(),
            action_id: action.id.clone(),
            engine: EngineKind::AdaptiveDb,
            status: ExecutionStatus::Applied,
            previous_state: Some(before),
            resulting_state: Some(after),
        })
    }
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError> {
        self.applied
            .remove(&format!("{}:{}", receipt.plan_id, receipt.action_id.0));
        let mut out = receipt.clone();
        out.status = ExecutionStatus::RolledBack;
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::in_memory::InMemoryAdaptiveDbClient;
    use aicp_core::{ActionId, EngineOperation, StorageStrategy};
    #[test]
    fn execute_is_idempotent() {
        let client =
            InMemoryAdaptiveDbClient::with_dataset("orders", StorageStrategy::Column, 18.0);
        let mut adapter = AdaptiveDbAdapter::new(client);
        let action = PlanAction {
            id: ActionId("a1".into()),
            engine: EngineKind::AdaptiveDb,
            dataset: "orders".into(),
            operation: EngineOperation::SetStorage {
                strategy: StorageStrategy::Hybrid,
            },
        };
        let first = adapter.execute("p1", &action).unwrap();
        let second = adapter.execute("p1", &action).unwrap();
        assert_eq!(first.status, ExecutionStatus::Applied);
        assert_eq!(second.status, ExecutionStatus::AlreadyApplied);
    }
    #[test]
    fn execute_changes_observed_layout() {
        let client =
            InMemoryAdaptiveDbClient::with_dataset("orders", StorageStrategy::Column, 18.0);
        let mut adapter = AdaptiveDbAdapter::new(client);
        let action = PlanAction {
            id: ActionId("a2".into()),
            engine: EngineKind::AdaptiveDb,
            dataset: "orders".into(),
            operation: EngineOperation::SetStorage {
                strategy: StorageStrategy::Row,
            },
        };
        adapter.execute("p2", &action).unwrap();
        let state = adapter.observe().unwrap();
        assert_eq!(
            state.datasets[0].storage_strategy,
            Some(StorageStrategy::Row)
        );
    }
}
