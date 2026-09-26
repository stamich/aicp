use crate::{error::IntentError, model::IntentDocument};
use aicp_core::IntentIr;

/// Validates semantic invariants in a decoded v1alpha1 intent document.
pub fn validate_document(document: &IntentDocument) -> Result<(), IntentError> {
    if document.api_version != "aicp/v1alpha1" {
        return Err(IntentError::Validation(format!(
            "unsupported apiVersion '{}'",
            document.api_version
        )));
    }
    if document.kind != "DataIntent" {
        return Err(IntentError::Validation(format!(
            "unsupported kind '{}'",
            document.kind
        )));
    }
    if document.metadata.name.trim().is_empty() {
        return Err(IntentError::Validation(
            "metadata.name must not be empty".into(),
        ));
    }
    if document.spec.target.dataset.trim().is_empty() {
        return Err(IntentError::Validation(
            "spec.target.dataset must not be empty".into(),
        ));
    }
    if document
        .spec
        .goals
        .p99_latency_ms
        .as_ref()
        .is_some_and(|value| value.max == 0)
    {
        return Err(IntentError::Validation(
            "p99 latency must be greater than zero".into(),
        ));
    }
    if document
        .spec
        .goals
        .availability_percent
        .as_ref()
        .is_some_and(|value| !(0.0..=100.0).contains(&value.min))
    {
        return Err(IntentError::Validation(
            "availability must be between 0 and 100".into(),
        ));
    }
    Ok(())
}

/// Re-validates invariants on an already normalized intent IR.
pub fn validate_ir(intent: &IntentIr) -> Result<(), IntentError> {
    if intent.name.trim().is_empty() {
        return Err(IntentError::Validation(
            "intent name must not be empty".into(),
        ));
    }
    if intent.target.dataset.trim().is_empty() {
        return Err(IntentError::Validation(
            "target dataset must not be empty".into(),
        ));
    }
    if intent.goals.max_p99_latency_ms == Some(0) {
        return Err(IntentError::Validation(
            "p99 latency must be greater than zero".into(),
        ));
    }
    if intent
        .goals
        .min_availability_percent
        .is_some_and(|value| !(0.0..=100.0).contains(&value))
    {
        return Err(IntentError::Validation(
            "availability must be between 0 and 100".into(),
        ));
    }
    Ok(())
}
