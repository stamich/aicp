use crate::model::IntentDocument;
use aicp_core::{Constraints, Goals, IntentIr, Preferences, Target};

/// Converts a validated external document into canonical AICP intent IR.
pub fn normalize(document: IntentDocument) -> IntentIr {
    IntentIr {
        name: document.metadata.name.trim().to_owned(),
        target: Target {
            dataset: document.spec.target.dataset.trim().to_owned(),
        },
        goals: Goals {
            max_p99_latency_ms: document.spec.goals.p99_latency_ms.map(|value| value.max),
            min_availability_percent: document
                .spec
                .goals
                .availability_percent
                .map(|value| value.min),
        },
        constraints: Constraints {
            durability: document.spec.constraints.durability,
            residency: document
                .spec
                .constraints
                .residency
                .into_iter()
                .map(|value| value.trim().to_uppercase())
                .collect(),
        },
        preferences: Preferences {
            minimize: document.spec.preferences.minimize,
        },
    }
}
