//! Criterion microbenchmarks for the AICP 0.2.1 control-plane path.

use aicp_adapter_adaptive_db::{AdaptiveDbAdapter, InMemoryAdaptiveDbClient};
use aicp_adapter_api::IntentTarget;
use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::{StorageStrategy, TelemetrySnapshot};
use aicp_intent::{parse_and_normalize, validate_ir};
use aicp_planner::plan;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Registers parser, validator, discovery, observation, planner, assurance and end-to-end benchmarks.
fn benchmarks(c: &mut Criterion) {
    let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
    let intent = parse_and_normalize(yaml).expect("embedded benchmark intent must be valid");
    let registry = CapabilityRegistry::baseline();
    let adapter = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
        "orders",
        StorageStrategy::Column,
        18.0,
    ));
    let observed = adapter.observe().unwrap();
    let telemetry = TelemetrySnapshot {
        p99_latency_ms: Some(9.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(100.0),
    };

    c.bench_function("intent_parse_normalize", |b| {
        b.iter(|| parse_and_normalize(black_box(yaml)).unwrap())
    });
    c.bench_function("intent_validate_ir", |b| {
        b.iter(|| validate_ir(black_box(&intent)).unwrap())
    });
    c.bench_function("adaptive_db_capability_discovery", |b| {
        b.iter(|| black_box(adapter.capabilities().unwrap()))
    });
    c.bench_function("adaptive_db_observe_state", |b| {
        b.iter(|| black_box(adapter.observe().unwrap()))
    });
    c.bench_function("planner_with_observed_state", |b| {
        b.iter(|| {
            plan(
                black_box(&intent),
                black_box(&registry),
                Some(black_box(&observed)),
            )
            .unwrap()
        })
    });
    c.bench_function("assurance_evaluate", |b| {
        b.iter(|| assure(black_box(&intent), black_box(&telemetry)))
    });
    c.bench_function("full_in_memory_pipeline", |b| {
        b.iter(|| {
            let ir = parse_and_normalize(black_box(yaml)).unwrap();
            validate_ir(&ir).unwrap();
            let state = adapter.observe().unwrap();
            let planned = plan(&ir, &registry, Some(&state)).unwrap();
            let report = assure(&ir, &telemetry);
            black_box((planned, report))
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
