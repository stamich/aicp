//! Execution status and immutable receipt model.

use aicp_core::{ActionId, EngineKind};
use serde::{Deserialize, Serialize};

/// Result status of one idempotent adapter action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    /// Action was newly applied.
    Applied,
    /// Adapter recognized that the same action had already been applied.
    AlreadyApplied,
    /// Action failed before completion.
    Failed,
    /// Action was rolled back after a later failure.
    RolledBack,
}

/// Immutable receipt produced by an engine adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    /// Plan identifier encoded as text to keep the receipt transport-neutral.
    pub plan_id: String,
    /// Action identifier.
    pub action_id: ActionId,
    /// Engine that executed the action.
    pub engine: EngineKind,
    /// Final execution status.
    pub status: ExecutionStatus,
    /// State fingerprint before execution when available.
    pub previous_state: Option<String>,
    /// State fingerprint after execution when available.
    pub resulting_state: Option<String>,
}
