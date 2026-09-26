//! Planner orchestration.

use crate::{
    budget::{AdaptationBudget, PlanningBudget},
    candidate::{cheap_prune, generate_cross_engine_candidates},
    error::PlannerError,
    feasibility::{ensure_required_capabilities, evaluate_feasibility},
    result::PlanningResult,
};
use aicp_capability::CapabilityRegistry;
use aicp_core::{ExecutionPlan, IntentIr};
use aicp_cost::score;
use aicp_decision::{DecisionGraph, ReasonKind};
use aicp_state::{fingerprint, ObservedState};
use uuid::Uuid;

/// Plans with default budgets and optional observed AdaptiveDB state.
pub fn plan(
    intent: &IntentIr,
    capabilities: &CapabilityRegistry,
    observed: Option<&ObservedState>,
) -> Result<PlanningResult, PlannerError> {
    plan_with_budget(
        intent,
        capabilities,
        observed,
        PlanningBudget::default(),
        AdaptationBudget::default(),
    )
}

/// Produces cross-engine AdaptiveDB × ACE candidates under explicit planning budgets.
pub fn plan_with_budget(
    intent: &IntentIr,
    capabilities: &CapabilityRegistry,
    observed: Option<&ObservedState>,
    planning_budget: PlanningBudget,
    adaptation_budget: AdaptationBudget,
) -> Result<PlanningResult, PlannerError> {
    ensure_required_capabilities(capabilities)?;
    let mut all = generate_cross_engine_candidates(intent, observed);
    cheap_prune(&mut all, planning_budget.max_candidates);
    let mut graph = DecisionGraph::new();

    for candidate in &mut all {
        evaluate_feasibility(intent, candidate, capabilities, adaptation_budget);
        if candidate.feasible {
            candidate.score = Some(score(&intent.preferences, &candidate.estimate));
            graph.push(
                ReasonKind::ConstraintSatisfied,
                format!("{} satisfies hard constraints", candidate.name),
                vec![],
            );
        } else {
            graph.push(
                ReasonKind::ConstraintRejected,
                format!(
                    "{} rejected: {}",
                    candidate.name,
                    candidate.rejection_reasons.join("; ")
                ),
                vec![],
            );
        }
    }

    let best = all
        .iter()
        .filter(|c| c.feasible)
        .min_by(|a, b| a.score.unwrap().total_cmp(&b.score.unwrap()))
        .ok_or(PlannerError::NoFeasiblePlan)?;
    graph.push(
        ReasonKind::Selected,
        format!(
            "{} selected with score {:.4}",
            best.name,
            best.score.unwrap()
        ),
        vec![],
    );
    let state_part = observed
        .map(|s| format!("{:?}", s))
        .unwrap_or_else(|| "none".into());
    let actions_part = format!("{:?}", best.actions);
    let fp = fingerprint(&[
        &intent.name,
        &intent.revision.to_string(),
        &state_part,
        &actions_part,
    ]);
    let selected = ExecutionPlan {
        id: Uuid::new_v4(),
        intent_name: intent.name.clone(),
        intent_revision: intent.revision,
        strategy_name: best.name.clone(),
        actions: best.actions.clone(),
        expected: best.estimate,
        score: best.score.unwrap(),
        fingerprint: fp,
    };
    Ok(PlanningResult {
        candidates: all,
        selected,
        decision_graph: graph,
    })
}
