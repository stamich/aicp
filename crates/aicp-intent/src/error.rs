//! Intent parsing and validation errors.

use thiserror::Error;

/// Errors produced while parsing or validating an intent.
#[derive(Debug, Error)]
pub enum IntentError {
    /// YAML could not be parsed.
    #[error("invalid YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
    /// Semantic validation failed.
    #[error("invalid intent: {0}")]
    Semantic(String),
}
