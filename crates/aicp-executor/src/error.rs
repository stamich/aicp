//! Executor error model.

use aicp_adapter_api::AdapterError;
use aicp_core::EngineKind;
use thiserror::Error;

/// Errors produced while validating or executing a plan.
#[derive(Debug, Error)]
pub enum ExecutorError {
    /// Required adapter is missing.
    #[error("missing adapter for {0}")]
    MissingAdapter(EngineKind),
    /// Adapter rejected or failed an action.
    #[error(transparent)]
    Adapter(#[from] AdapterError),
    /// Pre-execution validation rejected an action.
    #[error("action rejected: {0}")]
    Validation(String),
}
