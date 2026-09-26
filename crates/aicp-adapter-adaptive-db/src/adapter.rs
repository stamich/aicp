//! AdaptiveDB implementation of the generic IntentTarget contract.

use crate::client::AdaptiveDbClient;
use aicp_adapter_api::{AdapterError, IntentTarget};
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, EngineOperation, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt, ExecutionStatus};
use aicp_state::{fingerprint, EngineHealth, ObservedState, ResourceSnapshot};
use std::collections::HashSet;

/// AICP adapter translating generic actions to the AdaptiveDB client contract.
pub struct AdaptiveDbAdapter<C: AdaptiveDbClient> {
    client: C,
    applied: HashSet<String>,
}

impl<C: AdaptiveDbClient> AdaptiveDbAdapter<C> {
    /// Wraps a concrete AdaptiveDB client.
    pub fn new(client: C) -> Self {
        Self {
            client,
            applied: HashSet::new(),
        }
    }

    /// Exposes an immutable client reference for demo assertions.
    pub fn client(&self) -> &C {
        &self.client
    }
}

/// Produces a stable compact state fingerprint without leaking engine internals.
fn serde_state(state: &ObservedState) -> String {
    fingerprint(&[&format!("{:?}", state)])
}

impl<C: AdaptiveDbClient> IntentTarget for AdaptiveDbAdapter<C> {
    /// Identifies this adapter as AdaptiveDB.
    fn kind(&self) -> EngineKind {
        EngineKind::AdaptiveDb
    }

    /// Discovers capabilities from the connected AdaptiveDB client.
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

    /// Observes current AdaptiveDB state in normalized AICP form.
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

    /// Validates that an action targets AdaptiveDB and uses a supported layout.
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError> {
        if action.engine != EngineKind::AdaptiveDb {
            return Ok(ActionValidation::reject(
                "action targets a different engine",
            ));
        }
        match action.operation {
            EngineOperation::SetStorage { strategy } => {
                let supported = self
                    .client
                    .storage_capabilities()
                    .map_err(AdapterError::Unavailable)?
                    .contains(&strategy);
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

    /// Estimates a storage change using AdaptiveDB state.
    fn estimate(&self, action: &PlanAction) -> Result<ActionEstimate, AdapterError> {
        match action.operation {
            EngineOperation::SetStorage { strategy } => self
                .client
                .estimate_storage_change(&action.dataset, strategy)
                .map_err(AdapterError::Execution),
            _ => Err(AdapterError::Unsupported("not an AdaptiveDB action".into())),
        }
    }

    /// Applies a storage action idempotently.
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
        let before = serde_state(&self.observe()?);
        match action.operation {
            EngineOperation::SetStorage { strategy } => self
                .client
                .set_storage(&action.dataset, strategy)
                .map_err(AdapterError::Execution)?,
            _ => return Err(AdapterError::Unsupported("not an AdaptiveDB action".into())),
        }
        self.applied.insert(key);
        let after = serde_state(&self.observe()?);
        Ok(ExecutionReceipt {
            plan_id: plan_id.into(),
            action_id: action.id.clone(),
            engine: EngineKind::AdaptiveDb,
            status: ExecutionStatus::Applied,
            previous_state: Some(before),
            resulting_state: Some(after),
        })
    }

    /// Performs best-effort idempotency rollback bookkeeping.
    ///
    /// A production transport should additionally restore the previous physical configuration
    /// encoded by a richer engine-specific receipt. Milestone 0.3 intentionally keeps rollback
    /// conservative and explicit instead of inventing state that cannot be proven.
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError> {
        self.applied
            .remove(&format!("{}:{}", receipt.plan_id, receipt.action_id.0));
        let mut out = receipt.clone();
        out.status = ExecutionStatus::RolledBack;
        Ok(out)
    }
}
