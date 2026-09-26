use crate::error::PlannerError;
use aicp_capability::{CapabilityRegistry, EngineCapabilities};
use aicp_core::{CandidatePlan, Durability, EngineKind, EngineOperation, IntentIr, PlanAction};

/// Applies hard intent and capability constraints.
pub(crate) fn evaluate_feasibility(intent: &IntentIr, candidate: &mut CandidatePlan, registry: &CapabilityRegistry) {
    candidate.rejection_reasons.clear();
    if let Some(max) = intent.goals.max_p99_latency_ms { if candidate.estimate.p99_latency_ms > max as f64 { candidate.rejection_reasons.push(format!("p99 {:.1}ms exceeds {}ms", candidate.estimate.p99_latency_ms, max)); } }
    if let Some(min) = intent.goals.min_availability_percent { if candidate.estimate.availability_percent < min { candidate.rejection_reasons.push(format!("availability {:.3}% is below {:.3}%", candidate.estimate.availability_percent, min)); } }
    if intent.constraints.durability == Some(Durability::Strong) && !candidate.estimate.strong_durability { candidate.rejection_reasons.push("strong durability would be violated".into()); }
    for action in &candidate.actions { if !operation_supported(action, registry) { candidate.rejection_reasons.push(format!("{} does not support requested operation", action.engine)); } }
    candidate.feasible = candidate.rejection_reasons.is_empty();
}

fn operation_supported(action: &PlanAction, registry: &CapabilityRegistry) -> bool {
    match (&action.operation, registry.get(action.engine)) {
        (EngineOperation::SetStorage { strategy }, Some(EngineCapabilities::AdaptiveDb { storage, .. })) => storage.contains(strategy),
        (EngineOperation::SetCompression { profile }, Some(EngineCapabilities::Ace { profiles, .. })) => profiles.contains(profile),
        (EngineOperation::SetCoordination { strategy }, Some(EngineCapabilities::GraphNet { coordination, .. })) => coordination.contains(strategy),
        _ => false,
    }
}

/// Ensures every engine required by generated plans is represented.
pub(crate) fn ensure_required_capabilities(registry: &CapabilityRegistry) -> Result<(), PlannerError> {
    for kind in [EngineKind::AdaptiveDb, EngineKind::Ace, EngineKind::GraphNet] { if registry.get(kind).is_none() { return Err(PlannerError::MissingCapabilities(kind)); } }
    Ok(())
}
