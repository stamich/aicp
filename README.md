# Adaptive Intent Control Plane — Milestone 0.2.1

AICP 0.2.1 is a structural refactor of milestone 0.2. It preserves the state-aware control-plane behavior and the AdaptiveDB adapter boundary while reorganizing implementation code into responsibility-oriented modules.

## What changed

- observed engine and dataset state,
- drift and adaptation policy,
- versioned capability discovery,
- Adapter SPI v2 (`observe`, `validate`, `estimate`, idempotent `execute`, `rollback`),
- AdaptiveDB adapter with a transport-neutral client contract,
- state-aware planner with migration cost,
- plan fingerprints and intent revisions,
- `Degraded` assurance state,
- ranked explanations and `why_not`,
- execution receipts,
- JSON benchmark report schema and the supplied 0.1 baseline.

## Architecture

```text
Intent YAML
   │
   ▼
IntentIr ─────► semantic validation
   │
   ├──────────► CapabilityRegistry ◄──── dynamic adapters
   │
   ├──────────► ObservedState      ◄──── AdaptiveDB adapter
   │
   ▼
State-aware Planner
   │
   ├── candidate generation
   ├── hard-constraint filtering
   ├── migration-aware scoring
   └── explanation / fingerprint
   │
   ▼
ExecutionPlan
   │
   ├──── AdaptiveDBAdapter ───► AdaptiveDbClient
   ├──── Mock ACE
   └──── Mock GraphNet
   │
   ▼
ExecutionReceipt[]
   │
   ▼
Telemetry / Assurance / Drift
```

## Build and test

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Demo

```bash
cargo run -p aicp-demo
```

or inspect a plan:

```bash
cargo run -p aicp-cli -- plan examples/intents/low-latency-orders.yaml
cargo run -p aicp-cli -- why-not examples/intents/low-latency-orders.yaml cost-storage-first
```

## Benchmarks

```bash
scripts/run_benchmarks_json.sh
```

The runner executes Criterion and then writes the current run to `benchmark-results/aicp-0.2.1.json`. The supplied AICP 0.1 baseline is preserved separately in `benchmark-results/aicp-0.1-baseline.json`. Criterion remains responsible for statistically rigorous timing collection. See `docs/BENCHMARKS.md` for the 0.2.1 workflow and JSON schema.

## AdaptiveDB integration boundary

The milestone does not hard-code an AdaptiveDB wire protocol that may change between AdaptiveDB milestones. Instead, `AdaptiveDbClient` is the stable boundary. `InMemoryAdaptiveDbClient` makes the repository runnable today; the production FFI/RPC implementation plugs into the same trait without changing AICP core/planner code.
