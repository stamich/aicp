//! Benchmark report serialization.

use aicp_benchmark_model::BenchmarkReport;
use std::{fs, path::Path};

/// Serializes a benchmark report as stable pretty JSON.
pub fn write_json(path: impl AsRef<Path>, report: &BenchmarkReport) -> std::io::Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(report).expect("benchmark report model must serialize");
    fs::write(path, bytes)
}
