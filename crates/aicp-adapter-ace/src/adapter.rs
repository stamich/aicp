//! ACE implementation of the generic IntentTarget contract.

use crate::client::AceClient;
use aicp_adapter_api::{AdapterError, IntentTarget};
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, EngineOperation, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt, ExecutionStatus};
use aicp_state::{fingerprint, DatasetState, EngineHealth, ObservedState, ResourceSnapshot};
use std::collections::HashSet;

/// AICP execution adapter backed by an `AceClient` implementation.
pub struct AceAdapter<C: AceClient> {
    client: C,
    applied: HashSet<String>,
}

impl<C: AceClient> AceAdapter<C> {
    /// Wraps a concrete ACE client implementation.
    pub fn new(client: C) -> Self {
        Self {
            client,
            applied: HashSet::new(),
        }
    }

    /// Exposes the wrapped client for deterministic test assertions.
    pub fn client(&self) -> &C {
        &self.client
    }
}

/// Produces a stable fingerprint for a normalized state envelope.
fn state_fingerprint(state: &ObservedState) -> String {
    fingerprint(&[&format!("{:?}", state)])
}

impl<C: AceClient> IntentTarget for AceAdapter<C> {
    /// Identifies this adapter as ACE.
    fn kind(&self) -> EngineKind {
        EngineKind::Ace
    }

    /// Discovers supported compression profiles.
    fn capabilities(&self) -> Result<EngineCapabilities, AdapterError> {
        Ok(EngineCapabilities::Ace {
            version: self.client.version().map_err(AdapterError::Unavailable)?,
            profiles: self.client.profiles().map_err(AdapterError::Unavailable)?,
        })
    }

    /// Maps compression state into the common observed-state envelope.
    fn observe(&self) -> Result<ObservedState, AdapterError> {
        let ace = self
            .client
            .observe_datasets()
            .map_err(AdapterError::Unavailable)?;
        let datasets = ace
            .into_iter()
            .map(|x| DatasetState {
                name: x.dataset,
                storage_strategy: None,
                estimated_rows: 0,
                size_bytes: x.compressed_bytes,
                p99_latency_ms: Some(x.decode_latency_us / 1_000.0),
            })
            .collect();
        Ok(ObservedState {
            engine: EngineKind::Ace,
            version: self.client.version().map_err(AdapterError::Unavailable)?,
            health: EngineHealth::Healthy,
            resources: ResourceSnapshot {
                cpu_percent: 33.0,
                memory_percent: 12.0,
                storage_percent: 48.0,
            },
            datasets,
            sequence: 1,
        })
    }

    /// Validates that the requested profile is supported by this ACE instance.
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError> {
        if action.engine != EngineKind::Ace {
            return Ok(ActionValidation::reject(
                "action targets a different engine",
            ));
        }
        match action.operation {
            EngineOperation::SetCompression { profile } => {
                let supported = self
                    .client
                    .profiles()
                    .map_err(AdapterError::Unavailable)?
                    .contains(&profile);
                Ok(if supported {
                    ActionValidation::allow("compression profile supported")
                } else {
                    ActionValidation::reject("compression profile unsupported")
                })
            }
            _ => Ok(ActionValidation::reject(
                "operation is not an ACE compression action",
            )),
        }
    }

    /// Estimates compression migration cost and impact.
    fn estimate(&self, action: &PlanAction) -> Result<ActionEstimate, AdapterError> {
        match action.operation {
            EngineOperation::SetCompression { profile } => self
                .client
                .estimate_profile_change(&action.dataset, profile)
                .map_err(AdapterError::Execution),
            _ => Err(AdapterError::Unsupported("not an ACE action".into())),
        }
    }

    /// Applies a compression action idempotently.
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
                engine: EngineKind::Ace,
                status: ExecutionStatus::AlreadyApplied,
                previous_state: None,
                resulting_state: None,
            });
        }
        let before = state_fingerprint(&self.observe()?);
        match action.operation {
            EngineOperation::SetCompression { profile } => self
                .client
                .set_profile(&action.dataset, profile)
                .map_err(AdapterError::Execution)?,
            _ => return Err(AdapterError::Unsupported("not an ACE action".into())),
        }
        self.applied.insert(key);
        let after = state_fingerprint(&self.observe()?);
        Ok(ExecutionReceipt {
            plan_id: plan_id.into(),
            action_id: action.id.clone(),
            engine: EngineKind::Ace,
            status: ExecutionStatus::Applied,
            previous_state: Some(before),
            resulting_state: Some(after),
        })
    }

    /// Rolls back idempotency bookkeeping; a production ACE client should additionally restore the prior profile.
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError> {
        self.applied
            .remove(&format!("{}:{}", receipt.plan_id, receipt.action_id.0));
        let mut out = receipt.clone();
        out.status = ExecutionStatus::RolledBack;
        Ok(out)
    }
}
