//! Execution-engine identifiers and engine-specific strategy enums.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Execution-engine identifiers known to AICP.
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
    /// Formats an engine identifier for logs and CLI output.
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
    /// Row-oriented layout optimized for point operations.
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
    /// Fast encode/decode profile.
    Fast,
    /// Balanced general-purpose profile.
    Balanced,
    /// Storage-efficient profile accepting extra CPU.
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
    /// Majority-based Raft-style coordination.
    Raft,
    /// Coordination over an affected graph sub-scope.
    GraphScoped,
}
