//! Deterministic milestone-0.1 planner.
//!
//! The planner deliberately uses transparent heuristics instead of ML. Its role in
//! 0.1 is to prove candidate generation, feasibility filtering, multi-objective
//! scoring and explainability.

use aicp_capability::{CapabilityRegistry, EngineCapabilities};
use aicp_core::{
    CandidatePlan, CompressionProfile, CoordinationStrategy, Durability, EngineKind,
    EngineOperation, ExecutionPlan, IntentIr, Objective, PlanAction, PlanEstimate, StorageStrategy,
};
use thiserror::Error;
use uuid::Uuid;

/// Planner failures that prevent any execution plan from being selected.
#[derive(Debug, Error)]
pub enum PlannerError {
    /// No feasible candidate satisfies the hard constraints.
    #[error("no feasible plan satisfies the intent constraints")]
    NoFeasiblePlan,
    /// Required engine capabilities are missing.
    #[error("missing capabilities for engine {0}")]
    MissingCapabilities(EngineKind),
}

/// Result of a planning pass including every candidate and the winner.
#[derive(Debug, Clone)]
pub struct PlanningResult {
    /// All considered candidates, including rejected ones.
    pub candidates: Vec<CandidatePlan>,
    /// Selected immutable execution plan.
    pub selected: ExecutionPlan,
}

/// Generates candidates, filters them, scores feasible plans and selects the best one.
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
        .filter(|c| c.feasible)
        .min_by(|a, b| a.score.unwrap().total_cmp(&b.score.unwrap()))
        .ok_or(PlannerError::NoFeasiblePlan)?;
    let selected = ExecutionPlan {
        id: Uuid::new_v4(),
        intent_name: intent.name.clone(),
        strategy_name: best.name.clone(),
        actions: best.actions.clone(),
        expected: best.estimate,
        score: best.score.unwrap(),
    };
    Ok(PlanningResult {
        candidates,
        selected,
    })
}

/// Produces a human-readable explanation of why a plan was selected.
pub fn explain(result: &PlanningResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Selected {} ({})\n",
        result.selected.strategy_name, result.selected.id
    ));
    out.push_str(&format!("score: {:.3}\n", result.selected.score));
    out.push_str(&format!(
        "expected p99: {:.1} ms\n",
        result.selected.expected.p99_latency_ms
    ));
    out.push_str(&format!(
        "expected cost: {:.1} units\n\n",
        result.selected.expected.cost_units
    ));
    out.push_str("Candidates:\n");
    for c in &result.candidates {
        if c.feasible {
            out.push_str(&format!(
                "- {}: feasible, score {:.3}\n",
                c.name,
                c.score.unwrap()
            ));
        } else {
            out.push_str(&format!(
                "- {}: rejected: {}\n",
                c.name,
                c.rejection_reasons.join("; ")
            ));
        }
    }
    out
}

/// Builds three transparent baseline strategies used by milestone 0.1.
fn generate_candidates(intent: &IntentIr) -> Vec<CandidatePlan> {
    let d = intent.target.dataset.clone();
    vec![
        candidate(
            "latency-first",
            &d,
            StorageStrategy::Row,
            CompressionProfile::Fast,
            CoordinationStrategy::Raft,
            PlanEstimate {
                p99_latency_ms: 6.0,
                cost_units: 130.0,
                storage_units: 125.0,
                cpu_units: 70.0,
                network_units: 80.0,
                availability_percent: 99.999,
                strong_durability: true,
            },
        ),
        candidate(
            "balanced-adaptive",
            &d,
            StorageStrategy::Hybrid,
            CompressionProfile::Balanced,
            CoordinationStrategy::GraphScoped,
            PlanEstimate {
                p99_latency_ms: 8.0,
                cost_units: 95.0,
                storage_units: 82.0,
                cpu_units: 82.0,
                network_units: 62.0,
                availability_percent: 99.995,
                strong_durability: true,
            },
        ),
        candidate(
            "cost-storage-first",
            &d,
            StorageStrategy::Column,
            CompressionProfile::Dense,
            CoordinationStrategy::Partition,
            PlanEstimate {
                p99_latency_ms: 24.0,
                cost_units: 62.0,
                storage_units: 48.0,
                cpu_units: 115.0,
                network_units: 55.0,
                availability_percent: 99.95,
                strong_durability: false,
            },
        ),
    ]
}

/// Constructs one candidate from engine-specific strategy choices.
fn candidate(
    name: &str,
    dataset: &str,
    storage: StorageStrategy,
    profile: CompressionProfile,
    coordination: CoordinationStrategy,
    estimate: PlanEstimate,
) -> CandidatePlan {
    CandidatePlan {
        name: name.to_owned(),
        actions: vec![
            PlanAction {
                engine: EngineKind::AdaptiveDb,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetStorage { strategy: storage },
            },
            PlanAction {
                engine: EngineKind::Ace,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetCompression { profile },
            },
            PlanAction {
                engine: EngineKind::GraphNet,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetCoordination {
                    strategy: coordination,
                },
            },
        ],
        estimate,
        feasible: true,
        rejection_reasons: Vec::new(),
        score: None,
    }
}

/// Applies hard intent constraints and capability checks to a candidate.
fn evaluate_feasibility(
    intent: &IntentIr,
    candidate: &mut CandidatePlan,
    registry: &CapabilityRegistry,
) {
    candidate.rejection_reasons.clear();
    if let Some(max) = intent.goals.max_p99_latency_ms {
        if candidate.estimate.p99_latency_ms > max as f64 {
            candidate.rejection_reasons.push(format!(
                "p99 {:.1}ms exceeds {}ms",
                candidate.estimate.p99_latency_ms, max
            ));
        }
    }
    if let Some(min) = intent.goals.min_availability_percent {
        if candidate.estimate.availability_percent < min {
            candidate.rejection_reasons.push(format!(
                "availability {:.3}% is below {:.3}%",
                candidate.estimate.availability_percent, min
            ));
        }
    }
    if intent.constraints.durability == Some(Durability::Strong)
        && !candidate.estimate.strong_durability
    {
        candidate
            .rejection_reasons
            .push("strong durability would be violated".into());
    }
    for action in &candidate.actions {
        if !operation_supported(action, registry) {
            candidate.rejection_reasons.push(format!(
                "{} does not support requested operation",
                action.engine
            ));
        }
    }
    candidate.feasible = candidate.rejection_reasons.is_empty();
}

/// Returns whether a typed engine operation is supported by the static capability registry.
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

/// Computes a normalized weighted score; smaller values are better.
fn score(intent: &IntentIr, e: &PlanEstimate) -> f64 {
    let objectives = if intent.preferences.minimize.is_empty() {
        vec![Objective::Cost]
    } else {
        intent.preferences.minimize.clone()
    };
    let mut total = 0.0;
    let mut weight_sum = 0.0;
    for (index, objective) in objectives.iter().enumerate() {
        let weight = 1.0 / (index as f64 + 1.0);
        let value = match objective {
            Objective::Cost => e.cost_units / 150.0,
            Objective::Latency => e.p99_latency_ms / 50.0,
            Objective::Storage => e.storage_units / 150.0,
            Objective::Cpu => e.cpu_units / 150.0,
            Objective::Network => e.network_units / 150.0,
        };
        total += weight * value;
        weight_sum += weight;
    }
    total / weight_sum
}

/// Ensures all milestone-0.1 engines are represented in the capability registry.
fn ensure_required_capabilities(registry: &CapabilityRegistry) -> Result<(), PlannerError> {
    for kind in [
        EngineKind::AdaptiveDb,
        EngineKind::Ace,
        EngineKind::GraphNet,
    ] {
        if registry.get(kind).is_none() {
            return Err(PlannerError::MissingCapabilities(kind));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aicp_core::{Constraints, Goals, Preferences, Target};

    /// Builds a compact intent for planner tests.
    fn intent(max: u64, strong: bool) -> IntentIr {
        IntentIr {
            name: "x".into(),
            target: Target {
                dataset: "orders".into(),
            },
            goals: Goals {
                max_p99_latency_ms: Some(max),
                min_availability_percent: None,
            },
            constraints: Constraints {
                durability: if strong {
                    Some(Durability::Strong)
                } else {
                    None
                },
                residency: vec![],
            },
            preferences: Preferences {
                minimize: vec![Objective::Cost, Objective::Latency],
            },
        }
    }

    /// Verifies hard latency constraints reject slower cheap plans.
    #[test]
    fn chooses_feasible_low_latency_plan() {
        let r = plan(&intent(10, true), &CapabilityRegistry::milestone_0_1()).unwrap();
        assert!(r.selected.expected.p99_latency_ms <= 10.0);
        assert_ne!(r.selected.strategy_name, "cost-storage-first");
    }

    /// Verifies an impossible bound fails safely instead of weakening the intent.
    #[test]
    fn impossible_intent_has_no_plan() {
        let e = plan(&intent(1, true), &CapabilityRegistry::milestone_0_1()).unwrap_err();
        assert!(matches!(e, PlannerError::NoFeasiblePlan));
    }
}
