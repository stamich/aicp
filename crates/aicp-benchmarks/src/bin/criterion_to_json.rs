//! Converts Criterion 0.2.1 run artifacts into the stable AICP JSON benchmark schema.
//!
//! Run this binary after `cargo bench -p aicp-benchmarks`. Criterion estimates are
//! already expressed in nanoseconds; the converter therefore preserves the values
//! without the erroneous `/ 1000` scaling present in the original 0.2 exporter.

use aicp_benchmark_report::{
    classify, write_json, BenchmarkChange, BenchmarkReport, BenchmarkResult, Environment,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

/// Returns benchmark names emitted by the milestone-0.2.1 Criterion harness.
fn benchmark_names() -> &'static [&'static str] {
    &[
        "intent_parse_normalize",
        "intent_validate_ir",
        "adaptive_db_capability_discovery",
        "adaptive_db_observe_state",
        "planner_with_observed_state",
        "assurance_evaluate",
        "full_in_memory_pipeline",
    ]
}

/// Reads the Criterion mean confidence interval in nanoseconds.
fn read_estimate(path: &Path) -> Result<(f64, f64, f64), Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mean = &value["mean"];
    let low = mean["confidence_interval"]["lower_bound"]
        .as_f64()
        .ok_or("missing lower_bound")?;
    let point = mean["point_estimate"]
        .as_f64()
        .ok_or("missing point_estimate")?;
    let high = mean["confidence_interval"]["upper_bound"]
        .as_f64()
        .ok_or("missing upper_bound")?;
    Ok((low, point, high))
}

/// Loads central 0.2 estimates keyed by benchmark name.
fn load_baseline(path: &Path) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let value: Value = serde_json::from_slice(&fs::read(path)?)?;
    let mut out = HashMap::new();
    for item in value["benchmarks"]
        .as_array()
        .ok_or("baseline benchmarks missing")?
    {
        let Some(name) = item["name"].as_str() else {
            continue;
        };
        let mean = item["normalized"]["meanNs"]
            .as_f64()
            .or_else(|| item["raw"]["mean"].as_f64())
            .or_else(|| item["mean"].as_f64());
        if let Some(mean) = mean {
            out.insert(name.to_owned(), mean);
        }
    }
    Ok(out)
}

/// Builds a best-effort environment description without platform-specific dependencies.
fn environment() -> Environment {
    Environment {
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        cpu: std::env::var("AICP_BENCH_CPU").ok(),
        rust_version: std::env::var("AICP_RUST_VERSION").ok(),
        profile: "release".into(),
        git_commit: std::env::var("GIT_COMMIT").ok(),
    }
}

/// Converts the most recent Criterion run into `benchmark-results/aicp-0.2.1.json`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let baseline_path = workspace.join("benchmark-results/aicp-0.2-baseline.json");
    let criterion_root = workspace.join("target/criterion");
    let baseline = load_baseline(&baseline_path)?;
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
        let (low, mean, high) = read_estimate(&estimate_path)?;
        let change = baseline.get(*name).map(|old| {
            let percent = if *old <= f64::EPSILON {
                0.0
            } else {
                (mean - *old) / *old * 100.0
            };
            BenchmarkChange {
                baseline: "0.2".into(),
                percent,
                p_value: None,
                classification: classify(percent, None),
            }
        });
        results.push(BenchmarkResult {
            name: (*name).into(),
            unit: "ns".into(),
            low,
            mean,
            high,
            outliers: None,
            change,
        });
    }

    if results.is_empty() {
        return Err(
            "no Criterion estimates found; run cargo bench -p aicp-benchmarks first".into(),
        );
    }

    let report = BenchmarkReport {
        schema_version: "1.0".into(),
        project: "AICP".into(),
        milestone: "0.2.1".into(),
        environment: environment(),
        benchmarks: results,
    };
    let output = workspace.join("benchmark-results/aicp-0.2.1.json");
    write_json(&output, &report)?;
    println!("wrote {}", output.display());
    Ok(())
}
