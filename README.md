# AICP 0.1 — Adaptive Intent Control Plane

AICP 0.1 is the first vertical slice of an **Adaptive Intent Control Plane** shared by AdaptiveDB, Adaptive Compression Engine (ACE), and GraphNet.

The milestone proves the complete control-loop:

```text
Intent YAML
   ↓
Parse + normalize to IntentIR
   ↓
Semantic validation
   ↓
Capability registry
   ↓
Candidate plan generation
   ↓
Constraint filtering + scoring
   ↓
Explain selected plan
   ↓
Mock execution against ADB / ACE / GraphNet adapters
   ↓
Telemetry observation
   ↓
Assurance
   ↓
Satisfied ── yes → stable
   └── no → replan
```

## Scope

Implemented in 0.1:

- YAML intent model `aicp/v1alpha1`.
- Canonical `IntentIr` representation.
- Semantic validation.
- Static capability registry.
- Three candidate plans produced by a deterministic planner.
- Hard-constraint filtering.
- Weighted multi-objective scoring.
- Human-readable plan explanation.
- Mock adapters for AdaptiveDB, ACE, and GraphNet.
- Synchronous executor with rollback-on-failure semantics.
- Telemetry snapshot model.
- Assurance evaluation and replan recommendation.
- CLI.
- End-to-end demo.
- Criterion benchmarks.
- Unit and integration tests.

Explicitly out of scope for 0.1:

- ML/LLM planning.
- Distributed control plane.
- Real AdaptiveDB/ACE/GraphNet integrations.
- OPA integration.
- General-purpose constraint solver.
- Production RBAC/security.
- Persistent state store.

## Workspace

| Crate | Responsibility |
|---|---|
| `aicp-core` | Shared domain model: intents, plans, telemetry, assurance |
| `aicp-intent` | YAML parsing, normalization, semantic validation |
| `aicp-capability` | Static capability model and registry |
| `aicp-planner` | Candidate generation, feasibility checks, scoring, explain |
| `aicp-adapter-api` | Adapter SPI used by execution engines |
| `aicp-adapter-mock` | Mock ADB/ACE/GraphNet adapters for 0.1 |
| `aicp-executor` | Plan execution and rollback orchestration |
| `aicp-assurance` | Expected-vs-observed intent verification |
| `aicp-cli` | `plan`, `explain`, `apply`, `assure`, `demo` commands |
| `aicp-demo` | Standalone end-to-end demo executable |
| `aicp-benchmarks` | Criterion microbenchmarks |

## Build

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Run demo

```bash
cargo run -p aicp-demo
```

or via the CLI:

```bash
cargo run -p aicp-cli -- demo examples/intents/low-latency-orders.yaml
```

## CLI examples

```bash
cargo run -p aicp-cli -- plan examples/intents/low-latency-orders.yaml
cargo run -p aicp-cli -- explain examples/intents/low-latency-orders.yaml
cargo run -p aicp-cli -- apply examples/intents/low-latency-orders.yaml
cargo run -p aicp-cli -- assure examples/intents/low-latency-orders.yaml --observed-p99 15
```

## Benchmarks

```bash
cargo bench -p aicp-benchmarks
```

The benchmark suite measures:

1. parse + normalize of the example intent;
2. semantic validation;
3. candidate planning and selection;
4. assurance evaluation;
5. full in-memory pipeline excluding I/O.

0.1 benchmark results are intentionally not hard-coded in the repository because they depend on CPU/compiler/platform. Record them on the target workstation and commit the generated report separately if desired.

## Safety properties in 0.1

- Hard constraints are checked before a plan can be selected.
- The planner never silently weakens durability.
- Execution validates every action against adapter capabilities.
- On execution failure, already executed actions are rolled back in reverse order where possible.
- Assurance distinguishes `Satisfied`, `Violated`, and `Unknown` rather than guessing.
- Decisions are deterministic for the same intent, capabilities, and planner configuration.

## License

Apache-2.0. See `LICENSE`.
