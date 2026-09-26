use crate::{candidate::generate_candidates, error::PlannerError, feasibility::{ensure_required_capabilities, evaluate_feasibility}, result::PlanningResult};
use aicp_capability::CapabilityRegistry;
use aicp_core::{ExecutionPlan, IntentIr};
use aicp_cost::score;
use aicp_state::{fingerprint, ObservedState};
use uuid::Uuid;

/// Generates and selects a state-aware deterministic plan.
pub fn plan(intent: &IntentIr, capabilities: &CapabilityRegistry, observed: Option<&ObservedState>) -> Result<PlanningResult, PlannerError> {
    ensure_required_capabilities(capabilities)?;
    let mut candidates = generate_candidates(intent, observed);
    for candidate in &mut candidates {
        evaluate_feasibility(intent, candidate, capabilities);
        if candidate.feasible { candidate.score = Some(score(&intent.preferences, &candidate.estimate)); }
    }
    let best = candidates.iter().filter(|c| c.feasible).min_by(|a,b| a.score.unwrap().total_cmp(&b.score.unwrap())).ok_or(PlannerError::NoFeasiblePlan)?;
    let state_part = observed.map(|s| format!("{:?}", s)).unwrap_or_else(|| "none".into());
    let actions_part = format!("{:?}", best.actions);
    let revision = intent.revision.to_string();
    let fp = fingerprint(&[&intent.name, &revision, &state_part, &actions_part]);
    let selected = ExecutionPlan { id: Uuid::new_v4(), intent_name: intent.name.clone(), intent_revision: intent.revision, strategy_name: best.name.clone(), actions: best.actions.clone(), expected: best.estimate, score: best.score.unwrap(), fingerprint: fp };
    Ok(PlanningResult { candidates, selected })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptation::should_adapt;
    use aicp_core::{Constraints, Durability, Goals, Objective, Preferences, StorageStrategy, Target};
    use aicp_state::{AdaptationPolicy, DatasetState, EngineHealth, ResourceSnapshot};
    fn intent() -> IntentIr { IntentIr { name: "orders".into(), revision: 2, target: Target { dataset: "orders".into() }, goals: Goals { max_p99_latency_ms: Some(10), min_availability_percent: Some(99.99) }, constraints: Constraints { durability: Some(Durability::Strong), residency: vec!["EU".into()] }, preferences: Preferences { minimize: vec![Objective::Cost, Objective::Latency, Objective::Migration] } } }
    fn observed() -> ObservedState { ObservedState { engine: aicp_core::EngineKind::AdaptiveDb, version: "test".into(), health: EngineHealth::Healthy, resources: ResourceSnapshot { cpu_percent: 20.0, memory_percent: 20.0, storage_percent: 20.0 }, datasets: vec![DatasetState { name: "orders".into(), storage_strategy: Some(StorageStrategy::Column), estimated_rows: 1000, size_bytes: 1000, p99_latency_ms: Some(18.0) }], sequence: 1 } }
    #[test] fn hard_constraints_win_over_cost() { let r=plan(&intent(), &CapabilityRegistry::baseline(), Some(&observed())).unwrap(); assert_ne!(r.selected.strategy_name,"cost-storage-first"); assert!(r.selected.expected.p99_latency_ms <= 10.0); assert!(r.selected.expected.strong_durability); }
    #[test] fn fingerprint_is_stable() { let a=plan(&intent(), &CapabilityRegistry::baseline(), Some(&observed())).unwrap(); let b=plan(&intent(), &CapabilityRegistry::baseline(), Some(&observed())).unwrap(); assert_eq!(a.selected.fingerprint,b.selected.fingerprint); }
    #[test] fn hysteresis_suppresses_small_changes() { assert!(!should_adapt(Some(1.0),0.95,AdaptationPolicy::default())); assert!(should_adapt(Some(1.0),0.80,AdaptationPolicy::default())); }
}
