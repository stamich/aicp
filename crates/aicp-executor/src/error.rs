use aicp_adapter_api::AdapterError;
use aicp_core::EngineKind;
use thiserror::Error;

/// Errors raised while validating or applying a plan.
#[derive(Debug, Error)]
pub enum ExecutorError {
    /// No adapter was supplied for a required engine.
    #[error("missing adapter for {0}")]
    MissingAdapter(EngineKind),
    /// Adapter validation or execution failed after best-effort rollback.
    #[error("adapter failure: {0}")]
    Adapter(#[from] AdapterError),
}
