use aicp_capability::{CapabilityRegistry, EngineCapabilities};
use aicp_core::{CandidatePlan, Durability, EngineKind, EngineOperation, IntentIr, PlanAction};

use crate::error::PlannerError;

/// Ensures every engine required by the baseline planner has registered capabilities.
pub(crate) fn ensure_required_capabilities(
    registry: &CapabilityRegistry,
) -> Result<(), PlannerError> {
    for kind in [EngineKind::AdaptiveDb, EngineKind::Ace, EngineKind::GraphNet] {
        if registry.get(kind).is_none() {
            return Err(PlannerError::MissingCapabilities(kind));
        }
    }
    Ok(())
}

/// Applies hard intent constraints and capability checks to a candidate in place.
pub(crate) fn evaluate_feasibility(
    intent: &IntentIr,
    candidate: &mut CandidatePlan,
    registry: &CapabilityRegistry,
) {
    candidate.rejection_reasons.clear();
    reject_latency_violation(intent, candidate);
    reject_availability_violation(intent, candidate);
    reject_durability_violation(intent, candidate);
    reject_unsupported_actions(candidate, registry);
    candidate.feasible = candidate.rejection_reasons.is_empty();
}

/// Adds a rejection reason when the candidate violates the latency goal.
fn reject_latency_violation(intent: &IntentIr, candidate: &mut CandidatePlan) {
    if let Some(max) = intent.goals.max_p99_latency_ms {
        if candidate.estimate.p99_latency_ms > max as f64 {
            candidate.rejection_reasons.push(format!(
                "p99 {:.1}ms exceeds {}ms",
                candidate.estimate.p99_latency_ms, max
            ));
        }
    }
}

/// Adds a rejection reason when the candidate violates the availability goal.
fn reject_availability_violation(intent: &IntentIr, candidate: &mut CandidatePlan) {
    if let Some(min) = intent.goals.min_availability_percent {
        if candidate.estimate.availability_percent < min {
            candidate.rejection_reasons.push(format!(
                "availability {:.3}% is below {:.3}%",
                candidate.estimate.availability_percent, min
            ));
        }
    }
}

/// Adds a rejection reason when strong durability is required but unavailable.
fn reject_durability_violation(intent: &IntentIr, candidate: &mut CandidatePlan) {
    if intent.constraints.durability == Some(Durability::Strong)
        && !candidate.estimate.strong_durability
    {
        candidate
            .rejection_reasons
            .push("strong durability would be violated".into());
    }
}

/// Adds rejection reasons for actions unsupported by the capability registry.
fn reject_unsupported_actions(candidate: &mut CandidatePlan, registry: &CapabilityRegistry) {
    let unsupported = candidate
        .actions
        .iter()
        .filter(|action| !operation_supported(action, registry))
        .map(|action| format!("{} does not support requested operation", action.engine))
        .collect::<Vec<_>>();
    candidate.rejection_reasons.extend(unsupported);
}

/// Returns whether the registry supports a concrete typed plan action.
fn operation_supported(action: &PlanAction, registry: &CapabilityRegistry) -> bool {
    match (&action.operation, registry.get(action.engine)) {
        (
            EngineOperation::SetStorage { strategy },
            Some(EngineCapabilities::AdaptiveDb { storage, .. }),
        ) => storage.contains(strategy),
        (
            EngineOperation::SetCompression { profile },
            Some(EngineCapabilities::Ace { profiles }),
        ) => profiles.contains(profile),
        (
            EngineOperation::SetCoordination { strategy },
            Some(EngineCapabilities::GraphNet { coordination, .. }),
        ) => coordination.contains(strategy),
        _ => false,
    }
}
