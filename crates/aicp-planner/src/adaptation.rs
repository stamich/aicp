//! Hysteresis decision for replacing an existing plan.

use aicp_cost::relative_improvement;
use aicp_state::AdaptationPolicy;

/// Determines whether the new plan improves score enough to cross the hysteresis threshold.
pub fn should_adapt(previous_score: Option<f64>, new_score: f64, policy: AdaptationPolicy) -> bool {
    match previous_score {
        None => true,
        Some(old) => relative_improvement(old, new_score) >= policy.minimum_improvement_ratio,
    }
}
