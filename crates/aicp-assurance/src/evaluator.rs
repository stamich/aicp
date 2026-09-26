//! Assurance evaluation against intent objectives.

use aicp_core::{AssuranceReport, AssuranceStatus, Durability, IntentIr, TelemetrySnapshot};

/// Evaluates telemetry against intent goals and hard constraints.
pub fn assure(intent: &IntentIr, telemetry: &TelemetrySnapshot) -> AssuranceReport {
    let mut reasons = Vec::new();
    let mut unknown = false;
    let mut degraded = false;
    let mut violated = false;
    if let Some(max) = intent.goals.max_p99_latency_ms {
        match telemetry.p99_latency_ms {
            Some(actual) if actual > max as f64 => {
                violated = true;
                reasons.push(format!("p99 {actual:.2}ms exceeds {max}ms"));
            }
            Some(actual) if actual > max as f64 * 0.9 => {
                degraded = true;
                reasons.push(format!(
                    "p99 {actual:.2}ms is within 10% of the {max}ms limit"
                ));
            }
            Some(_) => {}
            None => unknown = true,
        }
    }
    if let Some(min) = intent.goals.min_availability_percent {
        match telemetry.availability_percent {
            Some(actual) if actual < min => {
                violated = true;
                reasons.push(format!("availability {actual:.3}% below {min:.3}%"));
            }
            Some(_) => {}
            None => unknown = true,
        }
    }
    if intent.constraints.durability == Some(Durability::Strong) {
        match telemetry.strong_durability {
            Some(false) => {
                violated = true;
                reasons.push("strong durability violated".into());
            }
            Some(true) => {}
            None => unknown = true,
        }
    }
    let status = if violated {
        AssuranceStatus::Violated
    } else if unknown {
        AssuranceStatus::Unknown
    } else if degraded {
        AssuranceStatus::Degraded
    } else {
        AssuranceStatus::Satisfied
    };
    AssuranceReport {
        status,
        reasons,
        recommend_replan: matches!(status, AssuranceStatus::Violated),
    }
}
