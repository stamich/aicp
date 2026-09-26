use crate::model::EngineCapabilities;
use aicp_core::{CompressionProfile, CoordinationStrategy, EngineKind, StorageStrategy};
use std::collections::HashMap;

/// In-memory registry of engine capabilities.
#[derive(Debug, Clone, Default)]
pub struct CapabilityRegistry {
    engines: HashMap<EngineKind, EngineCapabilities>,
}

impl CapabilityRegistry {
    /// Creates an empty capability registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the deterministic static registry used by the baseline demo.
    pub fn baseline() -> Self {
        let mut registry = Self::new();
        registry.register(
            EngineKind::AdaptiveDb,
            EngineCapabilities::AdaptiveDb {
                storage: vec![
                    StorageStrategy::Row,
                    StorageStrategy::Column,
                    StorageStrategy::Hybrid,
                ],
                strong_durability: true,
            },
        );
        registry.register(
            EngineKind::Ace,
            EngineCapabilities::Ace {
                profiles: vec![
                    CompressionProfile::Fast,
                    CompressionProfile::Balanced,
                    CompressionProfile::Dense,
                ],
            },
        );
        registry.register(
            EngineKind::GraphNet,
            EngineCapabilities::GraphNet {
                coordination: vec![
                    CoordinationStrategy::Local,
                    CoordinationStrategy::Partition,
                    CoordinationStrategy::Raft,
                    CoordinationStrategy::GraphScoped,
                ],
                strong_consistency: true,
            },
        );
        registry
    }

    /// Registers or replaces capabilities for an engine.
    pub fn register(&mut self, kind: EngineKind, capabilities: EngineCapabilities) {
        self.engines.insert(kind, capabilities);
    }

    /// Returns capabilities for an engine, if registered.
    pub fn get(&self, kind: EngineKind) -> Option<&EngineCapabilities> {
        self.engines.get(&kind)
    }
}
