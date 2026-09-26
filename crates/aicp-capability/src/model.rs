use aicp_core::{CompressionProfile, CoordinationStrategy, StorageStrategy};
use serde::{Deserialize, Serialize};

/// Capabilities advertised by a single execution engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EngineCapabilities {
    /// AdaptiveDB capabilities.
    AdaptiveDb {
        /// Supported storage strategies.
        storage: Vec<StorageStrategy>,
        /// Whether the engine can preserve strong durability.
        strong_durability: bool,
    },
    /// ACE capabilities.
    Ace {
        /// Supported compression profiles.
        profiles: Vec<CompressionProfile>,
    },
    /// GraphNet capabilities.
    GraphNet {
        /// Supported coordination strategies.
        coordination: Vec<CoordinationStrategy>,
        /// Whether strong consistency can be provided.
        strong_consistency: bool,
    },
}
