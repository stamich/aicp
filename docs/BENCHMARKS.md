# Benchmark plan

AICP is a control plane, so the milestone-0.1 benchmark objective is **low decision overhead and regression detection**, not comparison with database engines.

## Benchmarks

| Benchmark | Measures |
|---|---|
| `intent_parse_normalize` | YAML decoding + canonical IR creation |
| `intent_validate_ir` | semantic IR checks |
| `planner_three_candidates` | generation, constraint filtering, scoring and selection |
| `assurance_evaluate` | expected-vs-observed verification |
| `full_in_memory_pipeline` | parse + validate + plan + assurance |

## Run

```bash
cargo bench -p aicp-benchmarks
```

Criterion writes detailed statistical reports under `target/criterion/`.

## Suggested acceptance targets for 0.1

These are engineering targets, not measured results in this archive:

- planner median: `< 100 µs` on a modern desktop CPU;
- full in-memory pipeline median: `< 500 µs` for the sample intent;
- no regression > 20% without explanation.

The generation environment for this archive did not contain the Rust toolchain, so no fabricated benchmark numbers are included.
