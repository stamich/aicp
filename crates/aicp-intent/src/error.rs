use thiserror::Error;

/// Errors produced while parsing or validating an intent.
#[derive(Debug, Error)]
pub enum IntentError {
    #[error("invalid YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("invalid intent: {0}")]
    Semantic(String),
}
