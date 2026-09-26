use crate::{adapter::MockAdapter, log::MockLog};
use aicp_adapter_api::IntentTarget;
use aicp_capability::CapabilityRegistry;
use aicp_core::EngineKind;
use std::sync::Arc;

/// Creates the three standard baseline mock adapters plus their shared audit log.
pub fn standard_mock_adapters() -> (Vec<Box<dyn IntentTarget>>, Arc<MockLog>) {
    let registry = CapabilityRegistry::baseline();
    let log = Arc::new(MockLog::default());
    let adapters = [EngineKind::AdaptiveDb, EngineKind::Ace, EngineKind::GraphNet]
        .into_iter()
        .map(|kind| {
            let capabilities = registry
                .get(kind)
                .expect("static registry must contain every baseline engine")
                .clone();
            Box::new(MockAdapter::new(
                kind,
                capabilities,
                Arc::clone(&log),
                false,
            )) as Box<dyn IntentTarget>
        })
        .collect();
    (adapters, log)
}
