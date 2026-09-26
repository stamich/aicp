use aicp_core::{Objective, PlanEstimate, Preferences};

/// Computes a deterministic weighted score for a feasible plan estimate.
pub fn score(preferences: &Preferences, estimate: &PlanEstimate) -> f64 {
    let objectives = if preferences.minimize.is_empty() {
        vec![Objective::Cost]
    } else {
        preferences.minimize.clone()
    };
    let mut total = 0.0;
    let mut weight_sum = 0.0;
    for (index, objective) in objectives.iter().enumerate() {
        let weight = 1.0 / (index as f64 + 1.0);
        let normalized = match objective {
            Objective::Cost => estimate.cost_units / 150.0,
            Objective::Latency => estimate.p99_latency_ms / 50.0,
            Objective::Storage => estimate.storage_units / 150.0,
            Objective::Cpu => estimate.cpu_units / 150.0,
            Objective::Network => estimate.network_units / 150.0,
            Objective::Migration => estimate.migration_cost_units / 100.0,
        };
        total += weight * normalized;
        weight_sum += weight;
    }
    total / weight_sum
}

/// Returns the relative improvement from an old score to a new score.
pub fn relative_improvement(old_score: f64, new_score: f64) -> f64 {
    if old_score <= f64::EPSILON {
        0.0
    } else {
        (old_score - new_score) / old_score
    }
}
