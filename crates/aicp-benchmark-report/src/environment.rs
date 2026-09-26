//! Runtime/toolchain environment discovery.

use aicp_benchmark_model::EnvironmentFingerprint;
use std::{fs, process::Command};

/// Collects best-effort reproducibility metadata without failing a benchmark run.
pub fn collect_environment() -> EnvironmentFingerprint {
    EnvironmentFingerprint {
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        cpu: read_cpu_model().or_else(|| std::env::var("AICP_BENCH_CPU").ok()),
        logical_cpus: std::thread::available_parallelism().ok().map(|x| x.get()),
        rust_version: command_version("rustc", "--version"),
        cargo_version: command_version("cargo", "--version"),
        build_profile: "release".into(),
        git_commit: command_output("git", &["rev-parse", "--short", "HEAD"]),
        target_triple: std::env::var("TARGET").ok(),
        criterion_version: Some("0.5".into()),
    }
}

/// Runs a command and returns trimmed stdout if successful.
fn command_version(program: &str, arg: &str) -> Option<String> {
    command_output(program, &[arg])
}

/// Runs a command and returns trimmed stdout if successful.
fn command_output(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|x| x.status.success())
        .and_then(|x| String::from_utf8(x.stdout).ok())
        .map(|x| x.trim().to_owned())
        .filter(|x| !x.is_empty())
}

/// Reads a Linux CPU model when procfs is available.
fn read_cpu_model() -> Option<String> {
    let text = fs::read_to_string("/proc/cpuinfo").ok()?;
    text.lines()
        .find_map(|line| line.strip_prefix("model name\t: ").map(str::to_owned))
}
