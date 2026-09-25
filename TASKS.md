# AICP 0.1 — implementation task order

## Task 1 — Bootstrap the workspace

Create the Cargo workspace, common dependency versions, README, changelog, license and module boundaries.

**Exit criterion:** all crates are visible to Cargo and no crate has a circular dependency.

## Task 2 — Define the canonical domain model (`aicp-core`)

Implement:

- `IntentIr`, `Target`, `Goals`, `Constraints`, `Preferences`;
- typed enums for durability, storage, compression and coordination strategies;
- `CandidatePlan`, `ExecutionPlan`, `PlanAction`;
- `TelemetrySnapshot` and assurance result types.

**Exit criterion:** the rest of the system can depend on a stable typed IR rather than YAML.

## Task 3 — Implement intent ingestion (`aicp-intent`)

Implement:

- v1alpha1 YAML schema;
- parser;
- normalization into `IntentIr`;
- semantic validator;
- errors with actionable messages.

**Exit criterion:** valid YAML becomes a normalized IR; invalid intent is rejected before planning.

## Task 4 — Implement capabilities (`aicp-capability`)

Implement a static registry describing what the milestone-0.1 execution engines can do.

**Exit criterion:** planner and executor can query capabilities without hard-coding feature support.

## Task 5 — Define adapter SPI (`aicp-adapter-api`)

Implement engine adapters with:

- capability discovery;
- action validation;
- action execution;
- rollback.

**Exit criterion:** control-plane code has no dependency on engine implementations.

## Task 6 — Implement mock adapters (`aicp-adapter-mock`)

Create deterministic AdaptiveDB, ACE and GraphNet mocks.

**Exit criterion:** a complete plan can be executed without real engines.

## Task 7 — Implement planner (`aicp-planner`)

Implement:

- candidate generation;
- feasibility filtering;
- simple cost/latency/storage estimates;
- weighted scoring;
- selected-plan explanation.

**Exit criterion:** at least three candidate strategies are considered and rejected plans carry reasons.

## Task 8 — Implement executor (`aicp-executor`)

Execute actions in order, validating them against adapters. If any action fails, attempt reverse-order rollback.

**Exit criterion:** success and partial-failure paths are deterministic and testable.

## Task 9 — Implement assurance (`aicp-assurance`)

Compare telemetry with the declared intent and return `Satisfied`, `Violated`, or `Unknown` plus explicit reasons and replan recommendation.

**Exit criterion:** an intent violation can close the feedback loop.

## Task 10 — CLI and end-to-end demo

Expose `plan`, `explain`, `apply`, `assure`, and `demo` commands. Add sample intents and standalone demo executable.

**Exit criterion:** a user can inspect the whole control loop without reading source code.

## Task 11 — Tests

Add unit tests for parsing, validation, planning, constraint rejection, executor rollback, and assurance.

**Exit criterion:** every control-plane stage has at least one positive and one meaningful negative test.

## Task 12 — Benchmarks

Criterion benchmarks for parser, validator, planner, assurance, and full in-memory pipeline.

**Exit criterion:** future milestones can detect control-plane regressions.

## Task 13 — Quality gate

Run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p aicp-benchmarks
```

**Note:** the current ChatGPT execution environment used to generate this archive did not include `rustc`/`cargo`, so these commands must be executed on a Rust-enabled machine before tagging the release.
