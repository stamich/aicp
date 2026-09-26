#!/usr/bin/env bash
set -euo pipefail
cargo bench -p aicp-benchmarks
cargo run -p aicp-benchmarks --bin criterion_to_json
