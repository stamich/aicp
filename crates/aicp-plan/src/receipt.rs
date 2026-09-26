use aicp_core::{ActionId, EngineKind};
use serde::{Deserialize, Serialize};

/// Result status of one idempotent adapter action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Applied,
    AlreadyApplied,
    Failed,
    RolledBack,
}

/// Immutable receipt produced by an engine adapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionReceipt {
    pub plan_id: String,
    pub action_id: ActionId,
    pub engine: EngineKind,
    pub status: ExecutionStatus,
    pub previous_state: Option<String>,
    pub resulting_state: Option<String>,
}
