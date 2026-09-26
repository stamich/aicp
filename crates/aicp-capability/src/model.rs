use aicp_core::{CompressionProfile, CoordinationStrategy, EngineKind, StorageStrategy};
use serde::{Deserialize, Serialize};

/// Versioned capabilities advertised by one engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "engine", rename_all = "snake_case")]
pub enum EngineCapabilities {
    AdaptiveDb { version: String, storage: Vec<StorageStrategy>, projections: bool },
    Ace { version: String, profiles: Vec<CompressionProfile> },
    GraphNet { version: String, coordination: Vec<CoordinationStrategy> },
}

impl EngineCapabilities {
    /// Returns the engine represented by this capability document.
    pub fn kind(&self) -> EngineKind {
        match self { Self::AdaptiveDb { .. } => EngineKind::AdaptiveDb, Self::Ace { .. } => EngineKind::Ace, Self::GraphNet { .. } => EngineKind::GraphNet }
    }
}
