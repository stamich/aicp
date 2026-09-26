use aicp_adapter_api::AdapterError;
use aicp_core::EngineKind;
use thiserror::Error;

/// Errors produced while validating or executing a plan.
#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("missing adapter for {0}")]
    MissingAdapter(EngineKind),
    #[error(transparent)]
    Adapter(#[from] AdapterError),
    #[error("action rejected: {0}")]
    Validation(String),
}
