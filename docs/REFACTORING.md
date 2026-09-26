# AICP 0.3.1 refactoring

## Objective
Milestone 0.3.1 is a structural continuation of 0.2.1. The goal is to keep milestone 0.3 behavior while making crate boundaries and source files reflect one clear responsibility.

## Crate-root rule
Library `src/lib.rs` files are public API indices. They contain module declarations and re-exports only. `aicp-intent` additionally declares a private `model` module because serde input DTOs are an implementation detail.

## Test rule
Tests live in `<crate>/tests/`. Production source contains no `#[cfg(test)]` or `#[test]`. This keeps source modules focused and makes public-API testing explicit.

## Main mappings
- `aicp-core`: `intent.rs`, `engine.rs`, `plan.rs`, `telemetry.rs`, `assurance.rs`.
- `aicp-planner`: `budget.rs`, `candidate.rs`, `feasibility.rs`, `planner.rs`, `adaptation.rs`, `explain.rs`, `result.rs`, `error.rs`.
- `aicp-adapter-adaptive-db`: `client.rs`, `in_memory.rs`, `adapter.rs`.
- `aicp-adapter-ace`: `model.rs`, `client.rs`, `in_memory.rs`, `adapter.rs`.
- `aicp-benchmark-model`: unit/estimate/classification/result/environment/report.
- `aicp-benchmark-compare`: environment/classification/comparison.

## Design principles
SRP drives file boundaries; dependency direction stays toward domain abstractions. KISS avoids framework-style factories or generic abstractions not required by 0.3. DRY keeps shared models and scoring in their existing dedicated crates. YAGNI prevents introducing new runtime features during a structural milestone.
