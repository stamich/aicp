use serde::{Deserialize, Serialize};
use std::fmt;

/// Engine identifiers known to AICP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind {
    /// Adaptive Database execution engine.
    AdaptiveDb,
    /// Adaptive Compression Engine.
    Ace,
    /// GraphNet coordination engine.
    GraphNet,
}

impl fmt::Display for EngineKind {
    /// Writes the stable human-readable engine identifier.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::AdaptiveDb => "adaptive-db",
            Self::Ace => "ace",
            Self::GraphNet => "graphnet",
        };
        write!(f, "{value}")
    }
}

/// Physical data strategy selected for AdaptiveDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageStrategy {
    /// Row-oriented layout optimized for point reads and writes.
    Row,
    /// Column-oriented layout optimized for scans and compression.
    Column,
    /// Hybrid layout balancing transactional and analytical access.
    Hybrid,
}

/// Compression policy selected for ACE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionProfile {
    /// Fast encode/decode profile with lower compression ratio.
    Fast,
    /// Balanced profile for general-purpose workloads.
    Balanced,
    /// Storage-efficient profile accepting extra CPU work.
    Dense,
}

/// Coordination strategy selected for GraphNet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationStrategy {
    /// Coordination limited to directly affected entities where permitted.
    Local,
    /// Partition-scoped coordination.
    Partition,
    /// Majority-based coordination suitable for strong guarantees.
    Raft,
    /// Graph-scoped coordination over the affected subgraph.
    GraphScoped,
}

/// Typed operations understood by baseline AICP adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineOperation {
    /// Select an AdaptiveDB storage strategy.
    SetStorage { strategy: StorageStrategy },
    /// Select an ACE compression profile.
    SetCompression { profile: CompressionProfile },
    /// Select a GraphNet coordination strategy.
    SetCoordination { strategy: CoordinationStrategy },
}

#[cfg(test)]
mod tests {
    use super::EngineKind;

    #[test]
    /// Verifies stable display names used by CLI and explanations.
    fn engine_kind_display_is_stable() {
        assert_eq!(EngineKind::AdaptiveDb.to_string(), "adaptive-db");
        assert_eq!(EngineKind::Ace.to_string(), "ace");
        assert_eq!(EngineKind::GraphNet.to_string(), "graphnet");
    }
}
