use thiserror::Error;

/// Errors produced while parsing or validating an intent document.
#[derive(Debug, Error)]
pub enum IntentError {
    /// YAML syntax or deserialization failure.
    #[error("invalid YAML intent: {0}")]
    Parse(#[from] serde_yaml::Error),
    /// Semantic validation failure.
    #[error("invalid intent: {0}")]
    Validation(String),
}
