use aicp_capability::CapabilityRegistry;
use aicp_core::{
    Constraints, Durability, EngineKind, Goals, IntentIr, Objective, Preferences, Target,
};
use aicp_planner::{plan, plan_with_budget, AdaptationBudget, PlanningBudget};

fn intent() -> IntentIr {
    IntentIr {
        name: "orders".into(),
        revision: 1,
        target: Target {
            dataset: "orders".into(),
        },
        goals: Goals {
            max_p99_latency_ms: Some(10),
            min_availability_percent: None,
        },
        constraints: Constraints {
            durability: Some(Durability::Strong),
            residency: vec![],
        },
        preferences: Preferences {
            minimize: vec![Objective::Cost, Objective::Storage],
        },
    }
}

#[test]
fn plans_across_adb_and_ace() {
    let result = plan(&intent(), &CapabilityRegistry::baseline(), None).unwrap();
    assert!(result.candidates.len() >= 4);
    assert!(result
        .selected
        .actions
        .iter()
        .any(|a| a.engine == EngineKind::AdaptiveDb));
    assert!(result
        .selected
        .actions
        .iter()
        .any(|a| a.engine == EngineKind::Ace));
}

#[test]
fn respects_candidate_budget() {
    let result = plan_with_budget(
        &intent(),
        &CapabilityRegistry::baseline(),
        None,
        PlanningBudget { max_candidates: 4 },
        AdaptationBudget::default(),
    )
    .unwrap();
    assert_eq!(result.candidates.len(), 4);
}
