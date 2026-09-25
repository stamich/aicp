//! Deterministic mock adapters for AdaptiveDB, ACE and GraphNet.

use aicp_adapter_api::{AdapterError, AppliedAction, IntentTarget};
use aicp_capability::{CapabilityRegistry, EngineCapabilities};
use aicp_core::{EngineKind, EngineOperation, PlanAction};
use std::sync::{Arc, Mutex};

/// Shared in-memory log used by mock adapters and tests.
#[derive(Debug, Default)]
pub struct MockLog {
    entries: Mutex<Vec<String>>,
}

impl MockLog {
    /// Appends one audit line.
    pub fn push(&self, value: String) {
        self.entries
            .lock()
            .expect("mock log mutex poisoned")
            .push(value);
    }
    /// Returns a snapshot of all audit lines.
    pub fn entries(&self) -> Vec<String> {
        self.entries
            .lock()
            .expect("mock log mutex poisoned")
            .clone()
    }
}

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
}

impl IntentTarget for MockAdapter {
    fn engine(&self) -> EngineKind {
        self.engine
    }
    fn capabilities(&self) -> EngineCapabilities {
        self.capabilities.clone()
    }
    fn validate(&self, action: &PlanAction) -> Result<(), AdapterError> {
        if action.engine != self.engine {
            return Err(AdapterError::Unsupported(format!(
                "adapter {} cannot execute {} action",
                self.engine, action.engine
            )));
        }
        let ok = match (&self.capabilities, &action.operation) {
            (
                EngineCapabilities::AdaptiveDb { storage, .. },
                EngineOperation::SetStorage { strategy },
            ) => storage.contains(strategy),
            (EngineCapabilities::Ace { profiles }, EngineOperation::SetCompression { profile }) => {
                profiles.contains(profile)
            }
            (
                EngineCapabilities::GraphNet { coordination, .. },
                EngineOperation::SetCoordination { strategy },
            ) => coordination.contains(strategy),
            _ => false,
        };
        if ok {
            Ok(())
        } else {
            Err(AdapterError::Unsupported(
                "operation type does not match adapter".into(),
            ))
        }
    }
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
    fn rollback(&self, _action: &PlanAction, applied: &AppliedAction) -> Result<(), AdapterError> {
        self.log.push(format!("ROLLBACK {}", applied.token));
        Ok(())
    }
}

/// Creates the three standard milestone-0.1 mock adapters plus their shared audit log.
pub fn standard_mock_adapters() -> (Vec<Box<dyn IntentTarget>>, Arc<MockLog>) {
    let registry = CapabilityRegistry::milestone_0_1();
    let log = Arc::new(MockLog::default());
    let adapters: Vec<Box<dyn IntentTarget>> = [
        EngineKind::AdaptiveDb,
        EngineKind::Ace,
        EngineKind::GraphNet,
    ]
    .into_iter()
    .map(|kind| {
        Box::new(MockAdapter::new(
            kind,
            registry
                .get(kind)
                .expect("static registry complete")
                .clone(),
            Arc::clone(&log),
            false,
        )) as Box<dyn IntentTarget>
    })
    .collect();
    (adapters, log)
}
