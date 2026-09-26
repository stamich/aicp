# AICP 0.3.1 architecture

0.3.1 keeps the 0.3 runtime architecture and refactors source ownership. See `REFACTORING.md` for the source-module map. Library crate roots are API façades; implementation and tests are separated.

# AICP 0.3 architecture

## Control-plane boundary

AICP owns intent interpretation, constraints, planning, explanation, execution orchestration and assurance. AdaptiveDB and ACE own engine-specific state changes. GraphNet remains mocked in 0.3.

```text
                   Intent / Policy
                        │
                        ▼
                 AICP control plane
                        │
      ┌─────────────────┼─────────────────┐
      ▼                 ▼                 ▼
AdaptiveDbAdapter    AceAdapter      MockGraphNet
      │                 │                 │
AdaptiveDbClient      AceClient        mock state
```

## Invariants

1. Hard constraints are never traded for a better score.
2. Raw benchmark units are never compared directly.
3. Engine internals do not leak into the planner.
4. Every executed action returns an immutable receipt.
5. Repeated `(plan_id, action_id)` execution is idempotent at the adapter boundary.
6. Candidate generation is bounded by `PlanningBudget`.
7. Migration cost is bounded by `AdaptationBudget`.
8. Explainability is represented structurally by a decision graph.
9. AICP 0.3 does not claim production rollback of engine state when the backing engine contract cannot prove restoration semantics.
