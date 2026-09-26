# Changelog

## 0.3.1 - Structural refactor

### Changed
- Refactored all library crate roots into API-only façades (`pub mod` / `pub use`; `aicp-intent` keeps one private serde-model module).
- Moved core intent, engine, plan, telemetry and assurance types into responsibility-specific modules.
- Split planner orchestration, budgets, candidate generation, feasibility, adaptation, result and explanations.
- Split AdaptiveDB and ACE adapters into client, in-memory implementation, model/adapter responsibilities.
- Split benchmark schema, comparison and reporting crates into focused modules.
- Replaced milestone-numbered capability construction in active 0.3.1 code with `baseline()` / `legacy_baseline()`.
- Removed brittle assurance-status inference based on message text; violations are tracked explicitly.
- Moved all unit tests from `src/` into crate-local `tests/` integration-test directories.
- Strengthened `scripts/quality_gate.sh` to reject implementation in `lib.rs` and tests under `src/`.
- Updated workspace version and benchmark output identity to 0.3.1.

### Compatibility
- Retains milestone 0.3 planning, AdaptiveDB/ACE adapter, decision graph, executor, assurance and benchmark semantics.
- Public crate-root imports remain available through re-exports.


## 0.3.0

### Benchmark correctness
- Fixed the 0.2 Criterion unit-conversion bug.
- Added benchmark schema 1.1 with raw and normalized nanosecond estimates.
- Added suspicious-scale detection and environment compatibility checks.
- Added best-effort OS/arch/CPU/rustc/cargo/Git environment fingerprinting.
- Hardened assurance benchmarks with `black_box` inputs and outputs and separate status paths.
- Added an auditable corrected 0.2 baseline while retaining the supplied original JSON.

### Cross-engine planning
- Added transport-neutral `AceClient` and `AceAdapter`.
- Added ACE capability discovery, observation, estimation and idempotent execution.
- Added shared `CostVector` and normalization.
- Added AdaptiveDB × ACE candidate generation.
- Added deterministic candidate pruning, `PlanningBudget` and `AdaptationBudget`.
- Added structured decision reason graph.
- Added ADB+ACE demo and new integration benchmarks.

### Compatibility
- Existing intent schema remains compatible with 0.2.
- Existing `IntentTarget` SPI is retained.
- GraphNet remains mocked and is scheduled for a later milestone.

## 0.2.0
- Added observed-state planning, drift primitives, AdaptiveDB adapter contract, execution receipts, hysteresis and JSON benchmark export.

## 0.1.0
- Initial vertical slice: intent parsing, validation, capability registry, deterministic planning, mock execution and assurance.
