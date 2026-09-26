use aicp_assurance::assure;
use aicp_core::{AssuranceStatus, Constraints, Durability, Goals, IntentIr, Preferences, Target, TelemetrySnapshot};

fn intent() -> IntentIr {
    IntentIr { name: "x".into(), revision: 1, target: Target { dataset: "orders".into() }, goals: Goals { max_p99_latency_ms: Some(10), min_availability_percent: None }, constraints: Constraints { durability: Some(Durability::Strong), residency: vec![] }, preferences: Preferences::default() }
}

#[test]
fn degraded_near_boundary() {
    let report = assure(&intent(), &TelemetrySnapshot { p99_latency_ms: Some(9.5), availability_percent: None, strong_durability: Some(true), cost_units: None });
    assert_eq!(report.status, AssuranceStatus::Degraded);
    assert!(!report.recommend_replan);
}

#[test]
fn violation_recommends_replan() {
    let report = assure(&intent(), &TelemetrySnapshot { p99_latency_ms: Some(16.0), availability_percent: None, strong_durability: Some(true), cost_units: None });
    assert_eq!(report.status, AssuranceStatus::Violated);
    assert!(report.recommend_replan);
}
