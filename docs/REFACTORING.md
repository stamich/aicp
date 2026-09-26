# AICP 0.2.1 Refactoring

AICP 0.2.1 is a structural hardening release derived from AICP 0.2 and follows the same refactoring convention introduced for the corrected AICP 0.1.1 line.

## Rule for library roots

Every `crates/*/src/lib.rs` is an API index only:

```rust
pub mod ...;
pub use ...;
```

Implementation code, domain types, helper functions, trait implementations and tests belong to responsibility-oriented module files.

## Main mappings

- `aicp-core`: `intent.rs`, `engine.rs`, `plan.rs`, `telemetry.rs`, `assurance.rs`.
- `aicp-intent`: `model.rs`, `parser.rs`, `validate.rs`, `error.rs`.
- `aicp-capability`: `model.rs`, `registry.rs`, `provider.rs`.
- `aicp-state`: `model.rs`, `drift.rs`, `fingerprint.rs`, `policy.rs`.
- `aicp-plan`: `estimate.rs`, `validation.rs`, `receipt.rs`.
- `aicp-planner`: `candidate.rs`, `feasibility.rs`, `planner.rs`, `adaptation.rs`, `explain.rs`, `result.rs`, `error.rs`.
- `aicp-adapter-adaptive-db`: `client.rs`, `in_memory.rs`, `adapter.rs`, `serialization.rs`.
- `aicp-benchmark-report`: `model.rs`, `classification.rs`, `writer.rs`.

## Design rules

- **SRP/SOLID:** a module owns one cohesive responsibility.
- **KISS:** no additional framework or abstraction layer is introduced without a current use case.
- **DRY:** shared benchmark fixtures and domain primitives are centralized.
- **YAGNI:** the refactor does not introduce plugin containers, DI frameworks, generic repositories, or speculative async abstractions.
- **Dependency direction:** implementation modules import their owning sibling modules directly rather than depending on `lib.rs` re-exports.

## Behavioral changes intentionally included

Two small cleanups remove structural smells without changing product semantics:

1. `CapabilityRegistry::baseline()` replaces the release-number-specific `milestone_0_2()` constructor.
2. Assurance tracks hard violations explicitly; it no longer derives `Violated` by searching rendered explanation strings for words such as `exceeds` or `below`.
