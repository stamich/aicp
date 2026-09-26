//! Criterion benchmarks for AICP 0.3 benchmark hardening and cross-engine planning.

use aicp_adapter_ace::{AceAdapter, InMemoryAceClient};
use aicp_adapter_adaptive_db::{AdaptiveDbAdapter, InMemoryAdaptiveDbClient};
use aicp_adapter_api::IntentTarget;
use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::{CompressionProfile, StorageStrategy, TelemetrySnapshot};
use aicp_cost::{normalize, CostVector};
use aicp_intent::{parse_and_normalize, validate_ir};
use aicp_planner::{plan, plan_with_budget, AdaptationBudget, PlanningBudget};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Registers parser, validation, adapters, planner-size, assurance and end-to-end benchmarks.
fn benchmarks(c: &mut Criterion) {
    let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
    let intent = parse_and_normalize(yaml).expect("embedded benchmark intent must be valid");
    let registry = CapabilityRegistry::baseline();
    let adb = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
        "orders",
        StorageStrategy::Column,
        18.0,
    ));
    let ace = AceAdapter::new(InMemoryAceClient::with_dataset(
        "orders",
        CompressionProfile::Fast,
        1_000_000_000,
    ));
    let observed = adb.observe().unwrap();

    c.bench_function("intent_parse_normalize", |b| {
        b.iter(|| black_box(parse_and_normalize(black_box(yaml)).unwrap()))
    });
    c.bench_function("intent_validate_ir", |b| {
        b.iter(|| black_box(validate_ir(black_box(&intent)).unwrap()))
    });
    c.bench_function("adaptive_db_capability_discovery", |b| {
        b.iter(|| black_box(adb.capabilities().unwrap()))
    });
    c.bench_function("adaptive_db_observe_state", |b| {
        b.iter(|| black_box(adb.observe().unwrap()))
    });
    c.bench_function("ace_capability_discovery", |b| {
        b.iter(|| black_box(ace.capabilities().unwrap()))
    });
    c.bench_function("ace_observe_state", |b| {
        b.iter(|| black_box(ace.observe().unwrap()))
    });
    c.bench_function("cost_vector_normalization", |b| {
        b.iter(|| {
            black_box(normalize(black_box(CostVector {
                latency_ms: 8.0,
                cpu_units: 70.0,
                memory_units: 40.0,
                storage_units: 80.0,
                network_units: 50.0,
                monetary_units: 90.0,
                migration_units: 12.0,
            })))
        })
    });
    c.bench_function("planner_adb_ace_4_candidates", |b| {
        b.iter(|| {
            black_box(
                plan_with_budget(
                    black_box(&intent),
                    black_box(&registry),
                    Some(black_box(&observed)),
                    PlanningBudget { max_candidates: 4 },
                    AdaptationBudget::default(),
                )
                .unwrap(),
            )
        })
    });
    c.bench_function("planner_adb_ace_9_candidates", |b| {
        b.iter(|| {
            black_box(
                plan(
                    black_box(&intent),
                    black_box(&registry),
                    Some(black_box(&observed)),
                )
                .unwrap(),
            )
        })
    });

    let satisfied = TelemetrySnapshot {
        p99_latency_ms: Some(8.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(90.0),
    };
    let degraded = TelemetrySnapshot {
        p99_latency_ms: Some(9.5),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(90.0),
    };
    let violated = TelemetrySnapshot {
        p99_latency_ms: Some(16.0),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(90.0),
    };
    c.bench_function("assurance_satisfied", |b| {
        b.iter(|| black_box(assure(black_box(&intent), black_box(&satisfied))))
    });
    c.bench_function("assurance_degraded", |b| {
        b.iter(|| black_box(assure(black_box(&intent), black_box(&degraded))))
    });
    c.bench_function("assurance_violated", |b| {
        b.iter(|| black_box(assure(black_box(&intent), black_box(&violated))))
    });

    c.bench_function("full_adb_ace_pipeline", |b| {
        b.iter(|| {
            let ir = parse_and_normalize(black_box(yaml)).unwrap();
            black_box(validate_ir(&ir).unwrap());
            let state = black_box(adb.observe().unwrap());
            let planned = black_box(plan(&ir, &registry, Some(&state)).unwrap());
            let report = black_box(assure(&ir, &satisfied));
            black_box((planned, report))
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
