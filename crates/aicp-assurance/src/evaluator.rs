use aicp_core::{AssuranceReport, AssuranceStatus, Durability, IntentIr, TelemetrySnapshot};

/// Evaluates whether current telemetry satisfies every observable hard requirement.
pub fn assure(intent: &IntentIr, observed: &TelemetrySnapshot) -> AssuranceReport {
    let mut reasons = Vec::new();
    let mut unknown = false;
    let mut violated = false;

    evaluate_latency(intent, observed, &mut reasons, &mut unknown, &mut violated);
    evaluate_availability(intent, observed, &mut reasons, &mut unknown, &mut violated);
    evaluate_durability(intent, observed, &mut reasons, &mut unknown, &mut violated);

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

/// Evaluates the optional latency goal and updates aggregate assurance state.
fn evaluate_latency(
    intent: &IntentIr,
    observed: &TelemetrySnapshot,
    reasons: &mut Vec<String>,
    unknown: &mut bool,
    violated: &mut bool,
) {
    if let Some(max) = intent.goals.max_p99_latency_ms {
        match observed.p99_latency_ms {
            Some(actual) if actual > max as f64 => {
                *violated = true;
                reasons.push(format!("p99 latency {:.1}ms exceeds {}ms", actual, max));
            }
            Some(actual) => reasons.push(format!(
                "p99 latency {:.1}ms satisfies <= {}ms",
                actual, max
            )),
            None => {
                *unknown = true;
                reasons.push("p99 latency is not observable".into());
            }
        }
    }
}

/// Evaluates the optional availability goal and updates aggregate assurance state.
fn evaluate_availability(
    intent: &IntentIr,
    observed: &TelemetrySnapshot,
    reasons: &mut Vec<String>,
    unknown: &mut bool,
    violated: &mut bool,
) {
    if let Some(min) = intent.goals.min_availability_percent {
        match observed.availability_percent {
            Some(actual) if actual < min => {
                *violated = true;
                reasons.push(format!(
                    "availability {:.3}% is below {:.3}%",
                    actual, min
                ));
            }
            Some(actual) => reasons.push(format!(
                "availability {:.3}% satisfies >= {:.3}%",
                actual, min
            )),
            None => {
                *unknown = true;
                reasons.push("availability is not observable".into());
            }
        }
    }
}

/// Evaluates the strong-durability constraint and updates aggregate assurance state.
fn evaluate_durability(
    intent: &IntentIr,
    observed: &TelemetrySnapshot,
    reasons: &mut Vec<String>,
    unknown: &mut bool,
    violated: &mut bool,
) {
    if intent.constraints.durability != Some(Durability::Strong) {
        return;
    }
    match observed.strong_durability {
        Some(false) => {
            *violated = true;
            reasons.push("strong durability constraint is violated".into());
        }
        Some(true) => reasons.push("strong durability constraint is satisfied".into()),
        None => {
            *unknown = true;
            reasons.push("durability is not observable".into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::assure;
    use aicp_core::{
        AssuranceStatus, Constraints, Goals, IntentIr, Preferences, Target, TelemetrySnapshot,
    };

    #[test]
    /// Verifies a latency violation recommends replanning.
    fn latency_violation_recommends_replan() {
        let intent = IntentIr {
            name: "x".into(),
            target: Target { dataset: "d".into() },
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
