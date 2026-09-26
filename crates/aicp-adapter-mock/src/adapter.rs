use aicp_adapter_api::{AdapterError, IntentTarget};
use aicp_capability::EngineCapabilities;
use aicp_core::{CompressionProfile, CoordinationStrategy, EngineKind, PlanAction};
use aicp_plan::{ActionEstimate, ActionValidation, ExecutionReceipt, ExecutionStatus};
use aicp_state::{EngineHealth, ObservedState, ResourceSnapshot};
use std::collections::HashSet;

/// Generic deterministic mock adapter for ACE or GraphNet.
pub struct MockAdapter {
    kind: EngineKind,
    applied: HashSet<String>,
}

impl MockAdapter {
    /// Creates a mock adapter for the requested engine kind.
    pub fn new(kind: EngineKind) -> Self {
        Self {
            kind,
            applied: HashSet::new(),
        }
    }
}

impl IntentTarget for MockAdapter {
    /// Returns the configured engine kind.
    fn kind(&self) -> EngineKind {
        self.kind
    }

    /// Returns deterministic demo capabilities.
    fn capabilities(&self) -> Result<EngineCapabilities, AdapterError> {
        match self.kind {
            EngineKind::Ace => Ok(EngineCapabilities::Ace {
                version: "0.1-mock".into(),
                profiles: vec![
                    CompressionProfile::Fast,
                    CompressionProfile::Balanced,
                    CompressionProfile::Dense,
                ],
            }),
            EngineKind::GraphNet => Ok(EngineCapabilities::GraphNet {
                version: "0.x-mock".into(),
                coordination: vec![
                    CoordinationStrategy::Local,
                    CoordinationStrategy::Partition,
                    CoordinationStrategy::Raft,
                    CoordinationStrategy::GraphScoped,
                ],
            }),
            EngineKind::AdaptiveDb => Err(AdapterError::Unsupported(
                "use aicp-adapter-adaptive-db".into(),
            )),
        }
    }

    /// Returns a healthy empty state suitable for demo planning.
    fn observe(&self) -> Result<ObservedState, AdapterError> {
        Ok(ObservedState {
            engine: self.kind,
            version: "mock".into(),
            health: EngineHealth::Healthy,
            resources: ResourceSnapshot {
                cpu_percent: 10.0,
                memory_percent: 10.0,
                storage_percent: 10.0,
            },
            datasets: vec![],
            sequence: 1,
        })
    }

    /// Accepts actions addressed to this adapter.
    fn validate(&self, action: &PlanAction) -> Result<ActionValidation, AdapterError> {
        Ok(if action.engine == self.kind {
            ActionValidation::allow("mock action supported")
        } else {
            ActionValidation::reject("wrong engine")
        })
    }

    /// Returns a small deterministic estimate.
    fn estimate(&self, _action: &PlanAction) -> Result<ActionEstimate, AdapterError> {
        Ok(ActionEstimate {
            migration_cost_units: 2.0,
            latency_delta_ms: -1.0,
            storage_delta_percent: 0.0,
            confidence: 0.8,
        })
    }

    /// Applies an action idempotently.
    fn execute(
        &mut self,
        plan_id: &str,
        action: &PlanAction,
    ) -> Result<ExecutionReceipt, AdapterError> {
        let key = format!("{}:{}", plan_id, action.id.0);
        let status = if self.applied.insert(key) {
            ExecutionStatus::Applied
        } else {
            ExecutionStatus::AlreadyApplied
        };
        Ok(ExecutionReceipt {
            plan_id: plan_id.into(),
            action_id: action.id.clone(),
            engine: self.kind,
            status,
            previous_state: None,
            resulting_state: Some("mock-state".into()),
        })
    }

    /// Removes the idempotency key and returns a rollback receipt.
    fn rollback(&mut self, receipt: &ExecutionReceipt) -> Result<ExecutionReceipt, AdapterError> {
        self.applied
            .remove(&format!("{}:{}", receipt.plan_id, receipt.action_id.0));
        let mut out = receipt.clone();
        out.status = ExecutionStatus::RolledBack;
        Ok(out)
    }
}
