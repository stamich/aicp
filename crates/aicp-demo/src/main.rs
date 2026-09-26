//! End-to-end AICP 0.3 demonstration of cross-engine AdaptiveDB + ACE optimization.

use aicp_adapter_ace::{AceAdapter, InMemoryAceClient};
use aicp_adapter_adaptive_db::{AdaptiveDbAdapter, InMemoryAdaptiveDbClient};
use aicp_adapter_api::IntentTarget;
use aicp_adapter_mock::MockAdapter;
use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::{CompressionProfile, EngineKind, StorageStrategy, TelemetrySnapshot};
use aicp_executor::execute_plan;
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan};
use std::collections::HashMap;

/// Demonstrates observe -> cross-engine plan -> execute -> assure -> drift/replan recommendation.
fn main() -> anyhow::Result<()> {
    let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
    let intent = parse_and_normalize(yaml)?;

    let adaptive = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
        "orders",
        StorageStrategy::Column,
        18.0,
    ));
    let ace = AceAdapter::new(InMemoryAceClient::with_dataset(
        "orders",
        CompressionProfile::Fast,
        1_000_000_000,
    ));
    let initial_adb = adaptive.observe()?;
    let initial_ace = ace.observe()?;
    println!("=== INITIAL ADAPTIVEDB STATE ===\n{initial_adb:#?}\n");
    println!("=== INITIAL ACE STATE ===\n{initial_ace:#?}\n");

    let result = plan(&intent, &CapabilityRegistry::baseline(), Some(&initial_adb))?;
    println!("=== CROSS-ENGINE PLAN ===\n{}", explain(&result));

    let mut adapters: HashMap<EngineKind, Box<dyn IntentTarget>> = HashMap::new();
    adapters.insert(EngineKind::AdaptiveDb, Box::new(adaptive));
    adapters.insert(EngineKind::Ace, Box::new(ace));
    adapters.insert(
        EngineKind::GraphNet,
        Box::new(MockAdapter::new(EngineKind::GraphNet)),
    );
    let receipts = execute_plan(&result.selected, &mut adapters)?;
    println!("=== EXECUTION RECEIPTS ===\n{receipts:#?}\n");

    let healthy = TelemetrySnapshot {
        p99_latency_ms: Some(result.selected.expected.p99_latency_ms),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(result.selected.expected.cost_units),
    };
    println!(
        "=== ASSURANCE AFTER APPLY ===\n{:#?}\n",
        assure(&intent, &healthy)
    );

    let drifted = TelemetrySnapshot {
        p99_latency_ms: Some(16.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(result.selected.expected.cost_units),
    };
    println!(
        "=== SIMULATED WORKLOAD DRIFT ===\n{:#?}",
        assure(&intent, &drifted)
    );
    Ok(())
}
