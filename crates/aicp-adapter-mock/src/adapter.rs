use crate::log::MockLog;
use aicp_adapter_api::{AdapterError, AppliedAction, IntentTarget};
use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, EngineOperation, PlanAction};
use std::sync::Arc;

/// Generic deterministic mock for one AICP engine.
pub struct MockAdapter {
    engine: EngineKind,
    capabilities: EngineCapabilities,
    log: Arc<MockLog>,
    fail_execution: bool,
}

impl MockAdapter {
    /// Creates a mock adapter with explicit behavior.
    pub fn new(
        engine: EngineKind,
        capabilities: EngineCapabilities,
        log: Arc<MockLog>,
        fail_execution: bool,
    ) -> Self {
        Self {
            engine,
            capabilities,
            log,
            fail_execution,
        }
    }

    /// Checks whether this mock advertises support for the supplied operation.
    fn supports(&self, operation: &EngineOperation) -> bool {
        match (&self.capabilities, operation) {
            (
                EngineCapabilities::AdaptiveDb { storage, .. },
                EngineOperation::SetStorage { strategy },
            ) => storage.contains(strategy),
            (
                EngineCapabilities::Ace { profiles },
                EngineOperation::SetCompression { profile },
            ) => profiles.contains(profile),
            (
                EngineCapabilities::GraphNet { coordination, .. },
                EngineOperation::SetCoordination { strategy },
            ) => coordination.contains(strategy),
            _ => false,
        }
    }
}

impl IntentTarget for MockAdapter {
    /// Returns the engine represented by this mock adapter.
    fn engine(&self) -> EngineKind {
        self.engine
    }

    /// Returns the capabilities advertised by this mock adapter.
    fn capabilities(&self) -> EngineCapabilities {
        self.capabilities.clone()
    }

    /// Validates that the action targets this engine and is capability-compatible.
    fn validate(&self, action: &PlanAction) -> Result<(), AdapterError> {
        if action.engine != self.engine {
            return Err(AdapterError::Unsupported(format!(
                "adapter {} cannot execute {} action",
                self.engine, action.engine
            )));
        }
        if self.supports(&action.operation) {
            Ok(())
        } else {
            Err(AdapterError::Unsupported(
                "operation type does not match adapter".into(),
            ))
        }
    }

    /// Executes a validated action or returns the configured deterministic failure.
    fn execute(&self, action: &PlanAction) -> Result<AppliedAction, AdapterError> {
        self.validate(action)?;
        if self.fail_execution {
            return Err(AdapterError::Execution(format!(
                "injected failure for {}",
                self.engine
            )));
        }
        let token = format!("{}:{}:{:?}", self.engine, action.dataset, action.operation);
        self.log.push(format!("APPLY {token}"));
        Ok(AppliedAction { token })
    }

    /// Records a best-effort rollback for a previously applied action.
    fn rollback(
        &self,
        _action: &PlanAction,
        applied: &AppliedAction,
    ) -> Result<(), AdapterError> {
        self.log.push(format!("ROLLBACK {}", applied.token));
        Ok(())
    }
}
