use thiserror::Error;

/// Errors returned by execution-engine adapters.
#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("unsupported operation: {0}")]
    Unsupported(String),
    #[error("engine unavailable: {0}")]
    Unavailable(String),
    #[error("execution failed: {0}")]
    Execution(String),
}
