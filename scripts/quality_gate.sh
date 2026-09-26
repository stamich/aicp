#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

python3 - <<'PYQ'
from pathlib import Path
import re, sys
errors=[]
for lib in Path('crates').glob('*/src/lib.rs'):
    for no,line in enumerate(lib.read_text().splitlines(),1):
        s=line.strip()
        if not s or s.startswith('//!') or s.startswith('//'):
            continue
        if re.match(r'^pub\s+mod\s+[A-Za-z0-9_]+;$', s):
            continue
        if s.startswith('pub use '):
            continue
        errors.append(f'{lib}:{no}: implementation/non-facade content: {s}')
for src in Path('crates').glob('*/src/**/*.rs'):
    if '#[cfg(test)]' in src.read_text() or '#[test]' in src.read_text():
        errors.append(f'{src}: tests must live under the crate tests/ directory')
if errors:
    print('\n'.join(errors), file=sys.stderr)
    sys.exit(1)
print('structural gate: lib.rs façades and external tests OK')
PYQ

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p aicp-benchmarks --no-run
