//! Semantic validation of normalized intents.

use crate::error::IntentError;
use aicp_core::IntentIr;

/// Performs semantic validation after normalization.
pub fn validate_ir(ir: &IntentIr) -> Result<(), IntentError> {
    if ir.name.trim().is_empty() { return Err(IntentError::Semantic("metadata.name must not be empty".into())); }
    if ir.revision == 0 { return Err(IntentError::Semantic("metadata.revision must be >= 1".into())); }
    if ir.target.dataset.trim().is_empty() { return Err(IntentError::Semantic("spec.target.dataset must not be empty".into())); }
    if let Some(v) = ir.goals.max_p99_latency_ms { if v == 0 { return Err(IntentError::Semantic("p99LatencyMs.max must be > 0".into())); } }
    if let Some(v) = ir.goals.min_availability_percent { if !(0.0..=100.0).contains(&v) { return Err(IntentError::Semantic("availabilityPercent.min must be in [0,100]".into())); } }
    Ok(())
}
