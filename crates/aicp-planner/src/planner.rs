use crate::{
    candidate::generate_candidates,
    feasibility::{ensure_required_capabilities, evaluate_feasibility},
    scoring::score,
    error::PlannerError, result::PlanningResult,
};
use aicp_capability::CapabilityRegistry;
use aicp_core::{ExecutionPlan, IntentIr};
use uuid::Uuid;

/// Generates, filters, scores and selects the best deterministic execution plan.
pub fn plan(
    intent: &IntentIr,
    capabilities: &CapabilityRegistry,
) -> Result<PlanningResult, PlannerError> {
    ensure_required_capabilities(capabilities)?;
    let mut candidates = generate_candidates(intent);

    for candidate in &mut candidates {
        evaluate_feasibility(intent, candidate, capabilities);
        if candidate.feasible {
            candidate.score = Some(score(intent, &candidate.estimate));
        }
    }

    let best = candidates
        .iter()
        .filter(|candidate| candidate.feasible)
        .min_by(|left, right| {
            left.score
                .expect("feasible candidate must have a score")
                .total_cmp(&right.score.expect("feasible candidate must have a score"))
        })
        .ok_or(PlannerError::NoFeasiblePlan)?;

    let selected = ExecutionPlan {
        id: Uuid::new_v4(),
        intent_name: intent.name.clone(),
        strategy_name: best.name.clone(),
        actions: best.actions.clone(),
        expected: best.estimate,
        score: best.score.expect("selected candidate must have a score"),
    };

    Ok(PlanningResult {
        candidates,
        selected,
    })
}

#[cfg(test)]
mod tests {
    use super::plan;
    use crate::PlannerError;
    use aicp_capability::CapabilityRegistry;
    use aicp_core::{Constraints, Durability, Goals, IntentIr, Objective, Preferences, Target};

    /// Builds a compact intent fixture for planner unit tests.
    fn intent(max_latency_ms: u64, strong_durability: bool) -> IntentIr {
        IntentIr {
            name: "x".into(),
            target: Target {
                dataset: "orders".into(),
            },
            goals: Goals {
                max_p99_latency_ms: Some(max_latency_ms),
                min_availability_percent: None,
            },
            constraints: Constraints {
                durability: strong_durability.then_some(Durability::Strong),
                residency: vec![],
            },
            preferences: Preferences {
                minimize: vec![Objective::Cost, Objective::Latency],
            },
        }
    }

    #[test]
    /// Verifies a feasible low-latency plan is selected under a strict latency bound.
    fn chooses_feasible_low_latency_plan() {
        let result = plan(&intent(10, true), &CapabilityRegistry::baseline()).unwrap();
        assert!(result.selected.expected.p99_latency_ms <= 10.0);
        assert_ne!(result.selected.strategy_name, "cost-storage-first");
    }

    #[test]
    /// Verifies impossible hard constraints fail instead of being weakened.
    fn impossible_intent_has_no_plan() {
        let error = plan(&intent(1, true), &CapabilityRegistry::baseline()).unwrap_err();
        assert!(matches!(error, PlannerError::NoFeasiblePlan));
    }
}
