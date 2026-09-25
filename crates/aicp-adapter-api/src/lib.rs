//! Engine adapter service-provider interface.

use aicp_capability::EngineCapabilities;
use aicp_core::{EngineKind, PlanAction};
use thiserror::Error;

/// Errors returned by an execution-engine adapter.
#[derive(Debug, Error, Clone)]
pub enum AdapterError {
    /// The action does not match the adapter or its capabilities.
    #[error("unsupported action: {0}")]
    Unsupported(String),
    /// The engine failed while applying an action.
    #[error("execution failed: {0}")]
    Execution(String),
    /// Best-effort rollback failed.
    #[error("rollback failed: {0}")]
    Rollback(String),
}

/// Result token returned after a successful engine operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedAction {
    /// Human-readable engine-specific token useful for auditing and rollback.
    pub token: String,
}

/// Abstraction implemented by every AICP execution target.
pub trait IntentTarget: Send + Sync {
    /// Returns the engine represented by this adapter.
    fn engine(&self) -> EngineKind;
    /// Returns capabilities currently supported by the engine.
    fn capabilities(&self) -> EngineCapabilities;
    /// Validates an action without changing engine state.
    fn validate(&self, action: &PlanAction) -> Result<(), AdapterError>;
    /// Applies an action and returns a rollback/audit token.
    fn execute(&self, action: &PlanAction) -> Result<AppliedAction, AdapterError>;
    /// Performs best-effort rollback for a previously applied action.
    fn rollback(&self, action: &PlanAction, applied: &AppliedAction) -> Result<(), AdapterError>;
}
