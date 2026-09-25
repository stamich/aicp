//! Command-line interface for the AICP 0.1 vertical slice.

use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::TelemetrySnapshot;
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

/// Top-level command-line arguments.
#[derive(Debug, Parser)]
#[command(name = "aicp", version, about = "Adaptive Intent Control Plane 0.1")]
struct Cli {
    /// Selected operation.
    #[command(subcommand)]
    command: Command,
}

/// Supported milestone-0.1 CLI operations.
#[derive(Debug, Subcommand)]
enum Command {
    /// Parses an intent and prints all candidate plans plus the selected plan.
    Plan {
        /// Intent YAML file.
        intent: PathBuf,
    },
    /// Prints a human-readable explanation of the planning decision.
    Explain {
        /// Intent YAML file.
        intent: PathBuf,
    },
    /// Applies the selected plan to deterministic mock adapters.
    Apply {
        /// Intent YAML file.
        intent: PathBuf,
    },
    /// Evaluates an observed p99 latency against the declared intent.
    Assure {
        /// Intent YAML file.
        intent: PathBuf,
        /// Observed p99 latency in milliseconds.
        #[arg(long)]
        observed_p99: f64,
    },
    /// Runs plan, explain, apply and an intentional assurance violation.
    Demo {
        /// Intent YAML file.
        intent: PathBuf,
    },
}

/// Program entry point.
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Plan { intent } => {
            let (ir, result) = load_and_plan(&intent)?;
            println!("Intent IR:\n{}", serde_json::to_string_pretty(&ir)?);
            println!("\n{}", explain(&result));
        }
        Command::Explain { intent } => {
            let (_, result) = load_and_plan(&intent)?;
            println!("{}", explain(&result));
        }
        Command::Apply { intent } => {
            let (_, result) = load_and_plan(&intent)?;
            let (adapters, log) = aicp_adapter_mock::standard_mock_adapters();
            let receipt = aicp_executor::execute(&result.selected, &adapters)?;
            println!("Applied {} actions", receipt.applied_actions);
            for entry in log.entries() {
                println!("{entry}");
            }
        }
        Command::Assure {
            intent,
            observed_p99,
        } => {
            let yaml = read(&intent)?;
            let ir = parse_and_normalize(&yaml)?;
            let report = assure(
                &ir,
                &TelemetrySnapshot {
                    p99_latency_ms: Some(observed_p99),
                    availability_percent: ir.goals.min_availability_percent,
                    strong_durability: Some(true),
                    cost_units: None,
                },
            );
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
        Command::Demo { intent } => run_demo(&intent)?,
    }
    Ok(())
}

/// Loads, parses and plans one intent file.
fn load_and_plan(path: &PathBuf) -> Result<(aicp_core::IntentIr, aicp_planner::PlanningResult)> {
    let yaml = read(path)?;
    let ir = parse_and_normalize(&yaml)?;
    let result = plan(&ir, &CapabilityRegistry::milestone_0_1())?;
    Ok((ir, result))
}

/// Reads one UTF-8 intent file with contextual error reporting.
fn read(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

/// Runs the complete control-loop demonstration.
fn run_demo(path: &PathBuf) -> Result<()> {
    let (ir, result) = load_and_plan(path)?;
    println!("=== 1. PLAN ===\n{}", explain(&result));
    let (adapters, log) = aicp_adapter_mock::standard_mock_adapters();
    let receipt = aicp_executor::execute(&result.selected, &adapters)?;
    println!(
        "=== 2. APPLY ===\nApplied {} actions",
        receipt.applied_actions
    );
    for entry in log.entries() {
        println!("{entry}");
    }
    let violating = ir
        .goals
        .max_p99_latency_ms
        .map(|v| v as f64 + 5.0)
        .unwrap_or(15.0);
    let report = assure(
        &ir,
        &TelemetrySnapshot {
            p99_latency_ms: Some(violating),
            availability_percent: Some(99.999),
            strong_durability: Some(true),
            cost_units: Some(result.selected.expected.cost_units),
        },
    );
    println!(
        "\n=== 3. ASSURANCE ===\n{}",
        serde_json::to_string_pretty(&report)?
    );
    if report.recommend_replan {
        let replanned = plan(&ir, &CapabilityRegistry::milestone_0_1())?;
        println!("\n=== 4. REPLAN RECOMMENDED ===\n{}", explain(&replanned));
    }
    Ok(())
}
