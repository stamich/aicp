//! Cost normalization and deterministic scoring.

use crate::vector::{CostVector, NormalizedCostVector};
use aicp_core::{Objective, PlanEstimate, Preferences};

/// Converts a planner estimate into the shared cost-vector representation.
pub fn from_estimate(estimate: &PlanEstimate) -> CostVector {
    CostVector {
        latency_ms: estimate.p99_latency_ms,
        cpu_units: estimate.cpu_units,
        memory_units: 50.0,
        storage_units: estimate.storage_units,
        network_units: estimate.network_units,
        monetary_units: estimate.cost_units,
        migration_units: estimate.migration_cost_units,
    }
}

/// Normalizes heterogeneous dimensions into bounded engineering reference ranges.
pub fn normalize(value: CostVector) -> NormalizedCostVector {
    NormalizedCostVector {
        latency: value.latency_ms / 50.0,
        cpu: value.cpu_units / 150.0,
        memory: value.memory_units / 150.0,
        storage: value.storage_units / 150.0,
        network: value.network_units / 150.0,
        monetary: value.monetary_units / 150.0,
        migration: value.migration_units / 100.0,
    }
}

/// Computes a deterministic weighted score from normalized costs.
///
/// Lower values are better. Hard constraints must be checked before scoring.
pub fn score(preferences: &Preferences, estimate: &PlanEstimate) -> f64 {
    let n = normalize(from_estimate(estimate));
    let objectives = if preferences.minimize.is_empty() { vec![Objective::Cost] } else { preferences.minimize.clone() };
    let mut total = 0.0;
    let mut weight_sum = 0.0;
    for (index, objective) in objectives.iter().enumerate() {
        let weight = 1.0 / (index as f64 + 1.0);
        let value = match objective {
            Objective::Cost => n.monetary,
            Objective::Latency => n.latency,
            Objective::Storage => n.storage,
            Objective::Cpu => n.cpu,
            Objective::Network => n.network,
            Objective::Migration => n.migration,
        };
        total += weight * value;
        weight_sum += weight;
    }
    total / weight_sum
}

/// Returns the relative score improvement from an old plan to a new plan.
pub fn relative_improvement(old_score: f64, new_score: f64) -> f64 {
    if old_score <= f64::EPSILON { 0.0 } else { (old_score - new_score) / old_score }
}
