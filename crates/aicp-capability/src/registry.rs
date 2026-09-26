use crate::model::EngineCapabilities;
use aicp_core::{CompressionProfile, CoordinationStrategy, EngineKind, StorageStrategy};
use std::collections::HashMap;

/// In-memory registry that can be refreshed from dynamic providers.
#[derive(Debug, Clone, Default)]
pub struct CapabilityRegistry { entries: HashMap<EngineKind, EngineCapabilities> }

impl CapabilityRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self { Self::default() }

    /// Creates the deterministic baseline registry used by milestone 0.2.x examples.
    pub fn baseline() -> Self {
        let mut registry = Self::new();
        registry.insert(EngineCapabilities::AdaptiveDb { version: "2.x-demo".into(), storage: vec![StorageStrategy::Row, StorageStrategy::Column, StorageStrategy::Hybrid], projections: true });
        registry.insert(EngineCapabilities::Ace { version: "0.1-mock".into(), profiles: vec![CompressionProfile::Fast, CompressionProfile::Balanced, CompressionProfile::Dense] });
        registry.insert(EngineCapabilities::GraphNet { version: "0.x-mock".into(), coordination: vec![CoordinationStrategy::Local, CoordinationStrategy::Partition, CoordinationStrategy::Raft, CoordinationStrategy::GraphScoped] });
        registry
    }

    /// Inserts or replaces capabilities for an engine.
    pub fn insert(&mut self, capabilities: EngineCapabilities) { self.entries.insert(capabilities.kind(), capabilities); }
    /// Returns capabilities for an engine.
    pub fn get(&self, kind: EngineKind) -> Option<&EngineCapabilities> { self.entries.get(&kind) }
}
