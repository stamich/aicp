//! Adapter error model.

use thiserror::Error;

/// Errors returned by execution-engine adapters.
#[derive(Debug, Error)]
pub enum AdapterError {
    /// Adapter rejected an unsupported operation.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    /// Adapter could not communicate with its engine.
    #[error("engine unavailable: {0}")]
    Unavailable(String),
    /// Engine rejected a validly encoded action.
    #[error("execution failed: {0}")]
    Execution(String),
}
