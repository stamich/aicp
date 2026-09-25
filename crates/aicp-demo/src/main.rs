//! Standalone end-to-end demonstration of the AICP 0.1 closed control loop.

use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::TelemetrySnapshot;
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan};
use anyhow::Result;

/// Runs the complete milestone-0.1 demonstration against an embedded example intent.
fn main() -> Result<()> {
    let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
    let intent = parse_and_normalize(yaml)?;
    let registry = CapabilityRegistry::milestone_0_1();
    let planned = plan(&intent, &registry)?;

    println!("AICP 0.1 — closed-loop demo\n");
    println!(
        "1) Intent parsed: {} -> {}",
        intent.name, intent.target.dataset
    );
    println!("\n2) Planning\n{}", explain(&planned));

    let (adapters, log) = aicp_adapter_mock::standard_mock_adapters();
    let receipt = aicp_executor::execute(&planned.selected, &adapters)?;
    println!("3) Execution: {} actions applied", receipt.applied_actions);
    for entry in log.entries() {
        println!("   {entry}");
    }

    let observed = TelemetrySnapshot {
        p99_latency_ms: Some(15.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(planned.selected.expected.cost_units),
    };
    let report = assure(&intent, &observed);
    println!("\n4) Assurance: {:?}", report.status);
    for reason in &report.reasons {
        println!("   - {reason}");
    }
    println!("   recommend replan: {}", report.recommend_replan);

    if report.recommend_replan {
        let replanned = plan(&intent, &registry)?;
        println!(
            "\n5) Replan generated: {}",
            replanned.selected.strategy_name
        );
    }
    Ok(())
}
