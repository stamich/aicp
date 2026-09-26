#!/usr/bin/env bash
set -euo pipefail
cargo bench -p aicp-benchmarks
cargo run -p aicp-benchmarks --release --bin criterion_to_json
printf '\nAICP 0.3.1 JSON report: benchmark-results/aicp-0.3.1.json\n'
