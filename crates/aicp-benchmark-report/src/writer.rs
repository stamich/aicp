use crate::model::BenchmarkReport;
use std::{fs, path::Path};

/// Serializes a report as pretty JSON to the requested path.
pub fn write_json(path: impl AsRef<Path>, report: &BenchmarkReport) -> std::io::Result<()> {
    if let Some(parent) = path.as_ref().parent() { fs::create_dir_all(parent)?; }
    let bytes = serde_json::to_vec_pretty(report).expect("benchmark report model is serializable");
    fs::write(path, bytes)
}
