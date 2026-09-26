#!/usr/bin/env bash
set -euo pipefail

# Library roots are API indexes only.
for lib in crates/*/src/lib.rs; do
  [ -f "$lib" ] || continue
  if grep -Ev '^[[:space:]]*(//.*|$|pub mod [A-Za-z0-9_]+;|pub use .+;)$' "$lib" | grep -q .; then
    echo "ERROR: $lib contains implementation; only pub mod/pub use are allowed" >&2
    grep -En -v '^[[:space:]]*(//.*|$|pub mod [A-Za-z0-9_]+;|pub use .+;)$' "$lib" >&2 || true
    exit 1
  fi
done

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p aicp-benchmarks --no-run
