use serde::{Deserialize, Serialize};
use std::fmt;

/// Execution-engine identifiers known to AICP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineKind { AdaptiveDb, Ace, GraphNet }

impl fmt::Display for EngineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self { Self::AdaptiveDb => "adaptive-db", Self::Ace => "ace", Self::GraphNet => "graphnet" };
        write!(f, "{value}")
    }
}

/// Physical data strategy selected for AdaptiveDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageStrategy { Row, Column, Hybrid }

/// Compression policy selected for ACE.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompressionProfile { Fast, Balanced, Dense }

/// Coordination strategy selected for GraphNet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinationStrategy { Local, Partition, Raft, GraphScoped }

/// Typed operations understood by AICP adapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EngineOperation {
    SetStorage { strategy: StorageStrategy },
    SetCompression { profile: CompressionProfile },
    SetCoordination { strategy: CoordinationStrategy },
}
