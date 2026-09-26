# AICP 0.3.1 benchmark methodology

## 0.2 defect

The 0.2 Criterion converter assumed source estimates were picoseconds and divided them by 1000, while still writing `unit = ns`. Criterion 0.5's stored estimate values used by this harness are already nanoseconds. Therefore 0.2 JSON values were 1000× too small.

0.3.1 retains the 0.3 corrected behavior and treats the source values as nanoseconds and stores both `raw` and `normalized` forms.

## Benchmark groups

### Micro

- `intent_parse_normalize`
- `intent_validate_ir`
- `adaptive_db_capability_discovery`
- `adaptive_db_observe_state`
- `ace_capability_discovery`
- `ace_observe_state`
- `cost_vector_normalization`
- `assurance_satisfied`
- `assurance_degraded`
- `assurance_violated`

### Planner

- `planner_adb_ace_4_candidates`
- `planner_adb_ace_9_candidates`

### End-to-end

- `full_adb_ace_pipeline`

All benchmark inputs **and outputs** are passed through Criterion `black_box` on critical paths to prevent dead-code elimination from producing implausible measurements.

## JSON schema 1.1

A result contains source values and canonical nanoseconds. Comparisons operate only on `normalized.meanNs`.

A change exceeding two orders of magnitude is classified `suspicious` until the measurement is inspected.

## Baselines

`aicp-0.2-corrected-baseline.json` was derived from the supplied `aicp-0.2.json` by reversing the known erroneous `/1000` conversion. The original file is retained under `benchmark-results/source` for auditability.
