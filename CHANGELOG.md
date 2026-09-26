# Changelog

## 0.2.1

### Refactored
- Reserved every library `src/lib.rs` for `pub mod` declarations and `pub use` re-exports only.
- Split `aicp-core` into intent, engine, plan, telemetry and assurance modules.
- Split planner orchestration, candidate generation, feasibility, adaptation, explanations, result and error concerns.
- Split runtime state into model, drift, fingerprint and adaptation-policy modules.
- Split plan receipts, validation and estimates.
- Split AdaptiveDB transport contract, in-memory client, adapter and state serialization.
- Split benchmark report model, classification and writer.
- Replaced milestone-specific `CapabilityRegistry::milestone_0_2()` with neutral `CapabilityRegistry::baseline()`.
- Reworked assurance evaluation to track hard violations explicitly instead of inferring semantics from human-readable strings.
- Added a structural quality gate for `lib.rs` API-index compliance.
- Fixed the 0.2 benchmark JSON exporter unit bug and added the corrected 0.2 baseline for 0.2.1 comparisons.

### Compatibility
- Public crate-level imports are preserved through re-exports wherever possible.
- Planning, execution, assurance and benchmark behavior remain milestone-0.2 compatible.

## 0.2.0

### Added
- Observed state, engine health, dataset state and drift models.
- Adaptation hysteresis/cooldown policy.
- Intent revision and stable action IDs.
- Migration-aware cost objective and plan estimates.
- Versioned dynamic capability provider contract.
- Adapter SPI v2 with observe/validate/estimate/idempotent execute/rollback.
- AdaptiveDB adapter with transport-neutral `AdaptiveDbClient` contract.
- Executable in-memory AdaptiveDB client for demo and integration tests.
- Immutable execution receipts and plan fingerprints.
- `Degraded` assurance state.
- Ranked plan explanation and why-not reasoning.
- Versioned benchmark JSON model and 0.1 baseline exporter.
- Capability discovery, observation and state-aware planner benchmarks.

### Changed
- Planner now accepts observed state.
- Executor validates actions before apply and rolls back previous actions after downstream failure.
- Demo now exercises a concrete AdaptiveDB adapter plus mock ACE/GraphNet adapters.

## 0.1.0
- Initial Intent -> IR -> Validation -> Capabilities -> Planning -> Explain -> Mock Execute -> Assurance vertical slice.
