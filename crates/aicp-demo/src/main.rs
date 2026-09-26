//! End-to-end AICP 0.2.1 demonstration.

use aicp_adapter_adaptive_db::{AdaptiveDbAdapter, InMemoryAdaptiveDbClient};
use aicp_adapter_api::IntentTarget;
use aicp_adapter_mock::MockAdapter;
use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::{EngineKind, StorageStrategy, TelemetrySnapshot};
use aicp_executor::execute_plan;
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan};
use std::collections::HashMap;

/// Demonstrates observe -> plan -> explain -> execute -> assure -> detect violation.
fn main() -> anyhow::Result<()> {
    let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
    let intent = parse_and_normalize(yaml)?;
    let adaptive = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
        "orders",
        StorageStrategy::Column,
        18.0,
    ));
    let initial = adaptive.observe()?;
    println!("=== INITIAL ADAPTIVEDB STATE ===\n{initial:#?}\n");

    let result = plan(&intent, &CapabilityRegistry::baseline(), Some(&initial))?;
    println!("=== PLAN ===\n{}", explain(&result));

    let mut adapters: HashMap<EngineKind, Box<dyn IntentTarget>> = HashMap::new();
    adapters.insert(EngineKind::AdaptiveDb, Box::new(adaptive));
    adapters.insert(EngineKind::Ace, Box::new(MockAdapter::new(EngineKind::Ace)));
    adapters.insert(
        EngineKind::GraphNet,
        Box::new(MockAdapter::new(EngineKind::GraphNet)),
    );
    let receipts = execute_plan(&result.selected, &mut adapters)?;
    println!("=== EXECUTION RECEIPTS ===\n{receipts:#?}\n");

    let adb_state = adapters.get(&EngineKind::AdaptiveDb).unwrap().observe()?;
    let p99 = adb_state
        .datasets
        .iter()
        .find(|x| x.name == "orders")
        .and_then(|x| x.p99_latency_ms);
    let healthy = TelemetrySnapshot {
        p99_latency_ms: p99,
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(95.0),
    };
    println!(
        "=== ASSURANCE AFTER APPLY ===\n{:#?}\n",
        assure(&intent, &healthy)
    );

    let drifted = TelemetrySnapshot {
        p99_latency_ms: Some(16.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(95.0),
    };
    println!(
        "=== SIMULATED PERFORMANCE DRIFT ===\n{:#?}",
        assure(&intent, &drifted)
    );
    Ok(())
}
