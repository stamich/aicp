use aicp_core::{IntentIr, Objective, PlanEstimate};

/// Computes a normalized weighted score; smaller values are better.
pub(crate) fn score(intent: &IntentIr, estimate: &PlanEstimate) -> f64 {
    let objectives = if intent.preferences.minimize.is_empty() {
        vec![Objective::Cost]
    } else {
        intent.preferences.minimize.clone()
    };

    let (weighted_sum, weight_sum) = objectives
        .iter()
        .enumerate()
        .map(|(index, objective)| {
            let weight = 1.0 / (index as f64 + 1.0);
            (weight * normalized_value(*objective, estimate), weight)
        })
        .fold((0.0, 0.0), |(value_sum, weight_sum), (value, weight)| {
            (value_sum + value, weight_sum + weight)
        });

    weighted_sum / weight_sum
}

/// Normalizes one objective-specific estimate to the baseline scoring scale.
fn normalized_value(objective: Objective, estimate: &PlanEstimate) -> f64 {
    match objective {
        Objective::Cost => estimate.cost_units / 150.0,
        Objective::Latency => estimate.p99_latency_ms / 50.0,
        Objective::Storage => estimate.storage_units / 150.0,
        Objective::Cpu => estimate.cpu_units / 150.0,
        Objective::Network => estimate.network_units / 150.0,
    }
}
