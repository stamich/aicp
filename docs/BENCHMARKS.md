# AICP 0.2.1 Benchmarks

## 0.1 supplied baseline

The supplied Criterion output is preserved as the historical baseline:

| benchmark | central estimate |
|---|---:|
| intent_parse_normalize | 14.883 µs |
| intent_validate_ir | 14.633 ns |
| planner_three_candidates | 1.7455 µs |
| assurance_evaluate | 568.54 ns |
| full_in_memory_pipeline | 18.428 µs |

The original run reported a small statistically significant validator change of +2.1562%, stable planner/assurance, and an 8.3056% improvement of the full pipeline relative to its Criterion baseline.

## 0.2 benchmark groups

- `intent_parse_normalize`
- `intent_validate_ir`
- `adaptive_db_capability_discovery`
- `adaptive_db_observe_state`
- `planner_with_observed_state`
- `assurance_evaluate`
- `full_in_memory_pipeline`

These deliberately separate parser/validator microbenchmarks from adapter observation and the end-to-end in-memory control-plane path.

## JSON reporting

AICP 0.2.1 preserves schema version `1.0` with:

- project and milestone,
- OS and architecture,
- optional CPU, Rust version and Git commit,
- low/mean/high estimates,
- outlier count,
- baseline percentage and p-value,
- `improved`, `stable`, `warning`, `regression` classification.

Run:

```bash
scripts/run_benchmarks_json.sh
```

A future CI wrapper can parse Criterion's `target/criterion/**/estimates.json` and populate the same `BenchmarkReport` model automatically. Keeping schema generation inside a standalone crate avoids coupling Criterion's internal output layout to the control-plane domain.

## Engineering budgets

Initial non-SLA regression budgets for 0.2:

- parser + normalize: < 30 µs,
- planner: < 10 µs,
- assurance: < 2 µs,
- full in-memory control-plane path: < 100 µs.

These are regression guardrails, not customer-facing SLOs.


## 0.2.1 baseline

The refactor compares against `benchmark-results/aicp-0.2-baseline.json`, which contains the corrected 0.2 Criterion nanosecond values. The original 0.2 converter incorrectly divided Criterion estimates by 1000; 0.2.1 removes that conversion.
