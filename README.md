yes# AICP 0.3.1

AICP 0.3.1 is a structural refactor of milestone 0.3 and a direct continuation of 0.2.1. Runtime semantics remain those of 0.3; the release focuses on maintainability, explicit responsibility boundaries, and test layout.

## Structural rules

- Every library `src/lib.rs` is a public API façade containing only `pub mod`, private `mod` declarations when required for internal serde models, and `pub use`.
- Implementation lives in responsibility-focused module files.
- Tests live under each crate's `tests/` directory, never in `src/*.rs`.
- Internal modules import their owning module directly instead of depending on crate-root re-exports.
- The design follows SOLID, KISS, DRY and YAGNI: modules are split by responsibility, not by arbitrary line counts.

## Milestone 0.3 functionality retained

# AICP — Adaptive Intent Control Plane — Milestone 0.3

AICP 0.3 extends the 0.2 state-aware control plane with **benchmark correctness hardening**, a transport-neutral **Adaptive Compression Engine adapter**, and the first **cross-engine AdaptiveDB + ACE planner**.

## What 0.3 proves

The milestone demonstrates the closed loop:

```text
Intent YAML
   ↓
Intent IR + validation
   ↓
Capabilities + observed state
   ↓
Bounded ADB × ACE candidate generation
   ↓
Hard constraints + adaptation budget
   ↓
Shared cost normalization + scoring
   ↓
Decision reason graph
   ↓
ExecutionPlan
   ↓
AdaptiveDB adapter + ACE adapter + mock GraphNet
   ↓
Execution receipts
   ↓
Telemetry / assurance
   ↓
Satisfied / Degraded / Violated → replan recommendation
```

## Workspace modules

- `aicp-core` — canonical intent, actions, plans and telemetry.
- `aicp-intent` — YAML parsing, normalization and semantic validation.
- `aicp-capability` — versioned engine capability registry.
- `aicp-state` — observed state, drift and stabilization primitives.
- `aicp-cost` — shared cross-engine cost vectors and normalization.
- `aicp-decision` — structured decision reason graph.
- `aicp-plan` — validation, estimates and execution receipts.
- `aicp-planner` — bounded ADB×ACE candidate generation and selection.
- `aicp-adapter-api` — stable execution-engine SPI.
- `aicp-adapter-adaptive-db` — AdaptiveDB client boundary and executable in-memory contract implementation.
- `aicp-adapter-ace` — ACE client boundary and executable in-memory contract implementation.
- `aicp-adapter-mock` — GraphNet mock used until the 0.4 integration.
- `aicp-executor` — idempotent execution and best-effort rollback.
- `aicp-assurance` — Satisfied/Degraded/Violated/Unknown evaluation.
- `aicp-benchmark-model` — benchmark schema 1.1 and unit normalization.
- `aicp-benchmark-compare` — environment compatibility and regression sanity checks.
- `aicp-benchmark-report` — environment fingerprinting and JSON writer.
- `aicp-cli` — plan/explain/observe commands.
- `aicp-demo` — end-to-end ADB+ACE demonstration.
- `aicp-benchmarks` — Criterion benchmarks and JSON converter.

## Benchmark bug fixed from 0.2

The 0.2 converter divided Criterion's estimate values by `1000` but kept the output unit labelled `ns`. This produced false ~99.9% improvements. AICP 0.3:

1. preserves Criterion values as raw nanoseconds;
2. stores both raw and canonical normalized values;
3. compares only normalized nanoseconds;
4. marks >100× scale jumps as `suspicious`;
5. captures environment metadata before claiming a regression/improvement.

The supplied 0.2 result is retained at `benchmark-results/source/aicp-0.2-original.json`. The auditable corrected baseline is `benchmark-results/aicp-0.2-corrected-baseline.json`.

## Build and tests

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Or:

```bash
scripts/quality_gate.sh
```

## Demo

```bash
cargo run -p aicp-demo
```

The demo starts with AdaptiveDB column storage and ACE Fast compression, builds cross-engine alternatives, executes the selected plan, checks assurance and then injects a workload-latency violation.

## CLI

```bash
cargo run -p aicp-cli -- plan examples/intents/low-latency-orders.yaml
cargo run -p aicp-cli -- why-not examples/intents/low-latency-orders.yaml row-dense
cargo run -p aicp-cli -- observe orders
```

Candidate names are generated from the pair `<storage>-<compression>` in lowercase, for example `hybrid-balanced`.

## Benchmarks and JSON

```bash
scripts/run_benchmarks_json.sh
```

This runs Criterion and writes:

```text
benchmark-results/aicp-0.3.1.json
```

Schema 1.1 keeps:

```json
{
  "raw": { "mean": 15769.0, "unit": "nanoseconds" },
  "normalized": { "meanNs": 15769.0 }
}
```

This deliberate duplication makes unit mistakes visible and testable.

## Performance budgets (engineering gates, not product SLA)

- intent parse + normalize: `< 30 µs`
- validation: `< 100 ns`
- ADB+ACE planner, 4 candidates: `< 30 µs`
- ADB+ACE planner, 9 candidates: `< 60 µs`
- assurance path: `< 2 µs`
- full in-memory ADB+ACE pipeline: `< 250 µs`

## Integration boundary

The repository intentionally does **not** invent an AdaptiveDB or ACE network/FFI wire protocol. `AdaptiveDbClient` and `AceClient` are stable AICP-facing contracts. The included in-memory implementations execute the same adapter path used by a future real transport implementation.
