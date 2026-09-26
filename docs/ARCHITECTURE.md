# AICP 0.2.1 Architecture

## Control-plane invariant

AICP decides **what should happen**. Execution engines decide **how the engine-specific operation is performed**. Core and planner crates never import AdaptiveDB, ACE or GraphNet implementation APIs.

## Safety invariants

1. Hard constraints are evaluated before scoring.
2. An infeasible plan can never become selected because of a low cost score.
3. Every mutable execution action has an immutable ID and receipt.
4. Adapter retries are idempotent for the same `(plan_id, action_id)` pair.
5. The executor validates every action before mutating an engine.
6. If a later action fails, earlier actions are rolled back best-effort in reverse order.
7. AICP does not fabricate successful rollback state when the underlying engine cannot prove it.
8. Planner fingerprints include intent revision, observed state and selected actions.
9. Hysteresis is represented explicitly to prevent plan oscillation in future closed-loop execution.

## AdaptiveDB adapter

`AdaptiveDbAdapter<C>` implements the general `IntentTarget` SPI. The generic `AdaptiveDbClient` trait is intentionally narrow:

- `version`,
- `storage_capabilities`,
- `observe_datasets`,
- `set_storage`,
- `estimate_storage_change`.

A concrete AdaptiveDB FFI or RPC client can implement this trait in a later integration milestone without leaking wire-specific types into AICP.

## 0.2.1 source layout

The 0.2.1 hardening release makes the source layout follow responsibility boundaries while keeping crate boundaries unchanged.

```text
crates/
├── aicp-core/src/
│   ├── lib.rs          # public API index only
│   ├── intent.rs
│   ├── engine.rs
│   ├── plan.rs
│   ├── telemetry.rs
│   └── assurance.rs
├── aicp-state/src/
│   ├── lib.rs
│   ├── model.rs
│   ├── drift.rs
│   ├── fingerprint.rs
│   └── policy.rs
├── aicp-plan/src/
│   ├── lib.rs
│   ├── estimate.rs
│   ├── validation.rs
│   └── receipt.rs
├── aicp-planner/src/
│   ├── lib.rs
│   ├── candidate.rs
│   ├── feasibility.rs
│   ├── planner.rs
│   ├── adaptation.rs
│   ├── explain.rs
│   ├── result.rs
│   └── error.rs
└── aicp-adapter-adaptive-db/src/
    ├── lib.rs
    ├── client.rs
    ├── in_memory.rs
    ├── adapter.rs
    └── serialization.rs
```

The same `lib.rs = API index` rule is applied to every other library crate. Internal modules import sibling implementation modules directly; crate-root re-exports exist for consumers, not as an internal dependency mechanism.
