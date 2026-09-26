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
