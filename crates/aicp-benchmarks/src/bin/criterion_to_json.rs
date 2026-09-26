//! Converts Criterion artifacts to AICP benchmark schema 1.1 without lossy unit conversion.

use aicp_benchmark_compare::compare_measurement;
use aicp_benchmark_model::{
    BenchmarkReport, BenchmarkResult, NormalizedEstimate, RawEstimate, TimeUnit,
};
use aicp_benchmark_report::{collect_environment, write_json};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Benchmarks emitted by the milestone-0.3 harness.
fn benchmark_names() -> &'static [&'static str] {
    &[
        "intent_parse_normalize",
        "intent_validate_ir",
        "adaptive_db_capability_discovery",
        "adaptive_db_observe_state",
        "ace_capability_discovery",
        "ace_observe_state",
        "cost_vector_normalization",
        "planner_adb_ace_4_candidates",
        "planner_adb_ace_9_candidates",
        "assurance_satisfied",
        "assurance_degraded",
        "assurance_violated",
        "full_adb_ace_pipeline",
    ]
}

/// Maps comparable 0.3 names to corrected 0.2 baseline names.
fn baseline_name(name: &str) -> Option<&str> {
    match name {
        "intent_parse_normalize" => Some("intent_parse_normalize"),
        "intent_validate_ir" => Some("intent_validate_ir"),
        "adaptive_db_capability_discovery" => Some("adaptive_db_capability_discovery"),
        "adaptive_db_observe_state" => Some("adaptive_db_observe_state"),
        _ => None,
    }
}

/// Reads Criterion's mean estimate.
///
/// Criterion 0.5 stores these time values in nanoseconds. Milestone 0.2 incorrectly divided
/// them by 1000 while still labelling them `ns`; 0.3 preserves them as raw nanoseconds.
fn read_estimate(path: &Path) -> Result<RawEstimate, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mean = &value["mean"];
    Ok(RawEstimate {
        low: mean["confidence_interval"]["lower_bound"]
            .as_f64()
            .ok_or("missing lower_bound")?,
        mean: mean["point_estimate"]
            .as_f64()
            .ok_or("missing point_estimate")?,
        high: mean["confidence_interval"]["upper_bound"]
            .as_f64()
            .ok_or("missing upper_bound")?,
        unit: TimeUnit::Nanoseconds,
    })
}

/// Loads corrected normalized 0.2 baseline values keyed by benchmark name.
fn load_baseline(
    path: &Path,
) -> Result<HashMap<String, NormalizedEstimate>, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mut out = HashMap::new();
    for item in value["benchmarks"]
        .as_array()
        .ok_or("baseline benchmarks missing")?
    {
        if let Some(name) = item["name"].as_str() {
            let normalized = &item["normalized"];
            if let (Some(low_ns), Some(mean_ns), Some(high_ns)) = (
                normalized["lowNs"].as_f64(),
                normalized["meanNs"].as_f64(),
                normalized["highNs"].as_f64(),
            ) {
                out.insert(
                    name.to_owned(),
                    NormalizedEstimate {
                        low_ns,
                        mean_ns,
                        high_ns,
                    },
                );
            }
        }
    }
    Ok(out)
}

/// Converts the most recent Criterion run into `benchmark-results/aicp-0.3.1.json`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let baseline =
        load_baseline(&workspace.join("benchmark-results/aicp-0.2-corrected-baseline.json"))?;
    let criterion_root = workspace.join("target/criterion");
    let mut results = Vec::new();

    for name in benchmark_names() {
        let estimate_path = criterion_root.join(name).join("new/estimates.json");
        if !estimate_path.exists() {
            eprintln!(
                "warning: skipping {name}; {} does not exist",
                estimate_path.display()
            );
            continue;
        }
        let raw = read_estimate(&estimate_path)?;
        let normalized = raw.normalize_to_ns();
        let comparison = baseline_name(name)
            .and_then(|baseline_key| baseline.get(baseline_key))
            .map(|old| compare_measurement("0.2-corrected", old, &normalized, None, true));
        results.push(BenchmarkResult {
            name: (*name).into(),
            raw,
            normalized,
            comparison,
        });
    }

    if results.is_empty() {
        return Err(
            "no Criterion estimates found; run cargo bench -p aicp-benchmarks first".into(),
        );
    }
    let report = BenchmarkReport {
        schema_version: "1.1".into(),
        project: "AICP".into(),
        milestone: "0.3.1".into(),
        environment: collect_environment(),
        benchmarks: results,
    };
    let output = workspace.join("benchmark-results/aicp-0.3.1.json");
    write_json(&output, &report)?;
    println!("wrote {}", output.display());
    Ok(())
}
