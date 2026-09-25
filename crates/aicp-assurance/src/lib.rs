//! Assurance engine comparing declared intent with observed runtime telemetry.

use aicp_core::{AssuranceReport, AssuranceStatus, Durability, IntentIr, TelemetrySnapshot};

/// Evaluates whether current telemetry satisfies every observable hard requirement.
pub fn assure(intent: &IntentIr, observed: &TelemetrySnapshot) -> AssuranceReport {
    let mut reasons = Vec::new();
    let mut unknown = false;

    if let Some(max) = intent.goals.max_p99_latency_ms {
        match observed.p99_latency_ms {
            Some(actual) if actual > max as f64 => {
                reasons.push(format!("p99 latency {:.1}ms exceeds {}ms", actual, max))
            }
            Some(actual) => reasons.push(format!(
                "p99 latency {:.1}ms satisfies <= {}ms",
                actual, max
            )),
            None => {
                reasons.push("p99 latency is not observable".into());
                unknown = true;
            }
        }
    }
    if let Some(min) = intent.goals.min_availability_percent {
        match observed.availability_percent {
            Some(actual) if actual < min => {
                reasons.push(format!("availability {:.3}% is below {:.3}%", actual, min))
            }
            Some(actual) => reasons.push(format!(
                "availability {:.3}% satisfies >= {:.3}%",
                actual, min
            )),
            None => {
                reasons.push("availability is not observable".into());
                unknown = true;
            }
        }
    }
    if intent.constraints.durability == Some(Durability::Strong) {
        match observed.strong_durability {
            Some(false) => reasons.push("strong durability constraint is violated".into()),
            Some(true) => reasons.push("strong durability constraint is satisfied".into()),
            None => {
                reasons.push("durability is not observable".into());
                unknown = true;
            }
        }
    }

    let violated = reasons
        .iter()
        .any(|r| r.contains("exceeds") || r.contains("below") || r.contains("violated"));
    let status = if violated {
        AssuranceStatus::Violated
    } else if unknown {
        AssuranceStatus::Unknown
    } else {
        AssuranceStatus::Satisfied
    };
    AssuranceReport {
        status,
        reasons,
        recommend_replan: status == AssuranceStatus::Violated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aicp_core::{Constraints, Goals, Preferences, Target};

    /// Verifies the assurance engine recommends replanning on a latency breach.
    #[test]
    fn latency_violation_recommends_replan() {
        let intent = IntentIr {
            name: "x".into(),
            target: Target {
                dataset: "d".into(),
            },
            goals: Goals {
                max_p99_latency_ms: Some(10),
                min_availability_percent: None,
            },
            constraints: Constraints::default(),
            preferences: Preferences::default(),
        };
        let report = assure(
            &intent,
            &TelemetrySnapshot {
                p99_latency_ms: Some(15.0),
                ..Default::default()
            },
        );
        assert_eq!(report.status, AssuranceStatus::Violated);
        assert!(report.recommend_replan);
    }
}
