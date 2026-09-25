//! Capability registry used by planners and adapters.

use aicp_core::{CompressionProfile, CoordinationStrategy, EngineKind, StorageStrategy};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Capabilities advertised by a single execution engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EngineCapabilities {
    /// AdaptiveDB capabilities.
    AdaptiveDb {
        storage: Vec<StorageStrategy>,
        strong_durability: bool,
    },
    /// ACE capabilities.
    Ace { profiles: Vec<CompressionProfile> },
    /// GraphNet capabilities.
    GraphNet {
        coordination: Vec<CoordinationStrategy>,
        strong_consistency: bool,
    },
}

/// In-memory registry of engine capabilities.
#[derive(Debug, Clone, Default)]
pub struct CapabilityRegistry {
    engines: HashMap<EngineKind, EngineCapabilities>,
}

impl CapabilityRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates the deterministic static registry used by milestone 0.1.
    pub fn milestone_0_1() -> Self {
        let mut r = Self::new();
        r.register(
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
        r.register(
            EngineKind::Ace,
            EngineCapabilities::Ace {
                profiles: vec![
                    CompressionProfile::Fast,
                    CompressionProfile::Balanced,
                    CompressionProfile::Dense,
                ],
            },
        );
        r.register(
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
        r
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
