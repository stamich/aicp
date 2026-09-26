//! Emits a portable AICP benchmark JSON file.
//!
//! Criterion remains the source of statistical timing data. This helper establishes the
//! stable AICP JSON schema and can be fed by CI tooling or manually updated from Criterion
//! output. It also ships the milestone-0.1 baseline for immediate cross-version comparison.

use aicp_benchmark_report::{
    write_json, BenchmarkChange, BenchmarkReport, BenchmarkResult, Environment, RegressionClass,
};

/// Writes the checked-in 0.1 baseline in the same schema used by 0.2 and later milestones.
fn main() -> std::io::Result<()> {
    let report = BenchmarkReport {
        schema_version: "1.0".into(),
        project: "AICP".into(),
        milestone: "0.1-baseline".into(),
        environment: Environment {
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            cpu: None,
            rust_version: option_env!("RUSTC_VERSION").map(str::to_owned),
            profile: "release".into(),
            git_commit: option_env!("GIT_COMMIT").map(str::to_owned),
        },
        benchmarks: vec![
            BenchmarkResult {
                name: "intent_parse_normalize".into(),
                unit: "ns".into(),
                low: 14802.0,
                mean: 14883.0,
                high: 14978.0,
                outliers: Some(7),
                change: Some(BenchmarkChange {
                    baseline: "criterion-previous".into(),
                    percent: -7.4948,
                    p_value: Some(0.0),
                    classification: RegressionClass::Improved,
                }),
            },
            BenchmarkResult {
                name: "intent_validate_ir".into(),
                unit: "ns".into(),
                low: 14.520,
                mean: 14.633,
                high: 14.765,
                outliers: Some(6),
                change: Some(BenchmarkChange {
                    baseline: "criterion-previous".into(),
                    percent: 2.1562,
                    p_value: Some(0.0),
                    classification: RegressionClass::Stable,
                }),
            },
            BenchmarkResult {
                name: "planner_three_candidates".into(),
                unit: "ns".into(),
                low: 1734.1,
                mean: 1745.5,
                high: 1759.6,
                outliers: Some(6),
                change: Some(BenchmarkChange {
                    baseline: "criterion-previous".into(),
                    percent: 0.8544,
                    p_value: Some(0.12),
                    classification: RegressionClass::Stable,
                }),
            },
            BenchmarkResult {
                name: "assurance_evaluate".into(),
                unit: "ns".into(),
                low: 563.10,
                mean: 568.54,
                high: 576.50,
                outliers: Some(7),
                change: Some(BenchmarkChange {
                    baseline: "criterion-previous".into(),
                    percent: -1.0452,
                    p_value: Some(0.24),
                    classification: RegressionClass::Stable,
                }),
            },
            BenchmarkResult {
                name: "full_in_memory_pipeline".into(),
                unit: "ns".into(),
                low: 18235.0,
                mean: 18428.0,
                high: 18670.0,
                outliers: Some(5),
                change: Some(BenchmarkChange {
                    baseline: "criterion-previous".into(),
                    percent: -8.3056,
                    p_value: Some(0.0),
                    classification: RegressionClass::Improved,
                }),
            },
        ],
    };
    write_json("benchmark-results/aicp-0.1-baseline.json", &report)
}
