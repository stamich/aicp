//! Command-line interface for the AICP 1.1 vertical slice.

use aicp_assurance::assure;
use aicp_capability::CapabilityRegistry;
use aicp_core::{IntentIr, TelemetrySnapshot};
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan, PlanningResult};
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

/// Supported baseline CLI operations.
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
    match Cli::parse().command {
        Command::Plan { intent } => print_plan(&intent),
        Command::Explain { intent } => print_explanation(&intent),
        Command::Apply { intent } => apply_plan(&intent),
        Command::Assure {
            intent,
            observed_p99,
        } => print_assurance(&intent, observed_p99),
        Command::Demo { intent } => run_demo(&intent),
    }
}

/// Parses an intent and prints both canonical IR and the planning explanation.
fn print_plan(path: &PathBuf) -> Result<()> {
    let (intent, result) = load_and_plan(path)?;
    println!("Intent IR:\n{}", serde_json::to_string_pretty(&intent)?);
    println!("\n{}", explain(&result));
    Ok(())
}

/// Prints only the human-readable explanation for a selected plan.
fn print_explanation(path: &PathBuf) -> Result<()> {
    let (_, result) = load_and_plan(path)?;
    println!("{}", explain(&result));
    Ok(())
}

/// Applies a planned intent to deterministic mock adapters and prints their audit log.
fn apply_plan(path: &PathBuf) -> Result<()> {
    let (_, result) = load_and_plan(path)?;
    let (adapters, log) = aicp_adapter_mock::standard_mock_adapters();
    let receipt = aicp_executor::execute(&result.selected, &adapters)?;
    println!("Applied {} actions", receipt.applied_actions);
    for entry in log.entries() {
        println!("{entry}");
    }
    Ok(())
}

/// Evaluates an observed latency value against a parsed intent and prints JSON assurance output.
fn print_assurance(path: &PathBuf, observed_p99: f64) -> Result<()> {
    let intent = parse_and_normalize(&read(path)?)?;
    let observed = TelemetrySnapshot {
        p99_latency_ms: Some(observed_p99),
        availability_percent: intent.goals.min_availability_percent,
        strong_durability: Some(true),
        cost_units: None,
    };
    println!("{}", serde_json::to_string_pretty(&assure(&intent, &observed))?);
    Ok(())
}

/// Loads, parses and plans one intent file.
fn load_and_plan(path: &PathBuf) -> Result<(IntentIr, PlanningResult)> {
    let intent = parse_and_normalize(&read(path)?)?;
    let result = plan(&intent, &CapabilityRegistry::baseline())?;
    Ok((intent, result))
}

/// Reads one UTF-8 intent file with contextual error reporting.
fn read(path: &PathBuf) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))
}

/// Runs the complete control-loop demonstration.
fn run_demo(path: &PathBuf) -> Result<()> {
    let (intent, result) = load_and_plan(path)?;
    println!("=== 1. PLAN ===\n{}", explain(&result));

    let (adapters, log) = aicp_adapter_mock::standard_mock_adapters();
    let receipt = aicp_executor::execute(&result.selected, &adapters)?;
    println!("=== 2. APPLY ===\nApplied {} actions", receipt.applied_actions);
    for entry in log.entries() {
        println!("{entry}");
    }

    let violating_latency = intent
        .goals
        .max_p99_latency_ms
        .map(|value| value as f64 + 5.0)
        .unwrap_or(15.0);
    let observed = TelemetrySnapshot {
        p99_latency_ms: Some(violating_latency),
        availability_percent: Some(99.999),
        strong_durability: Some(true),
        cost_units: Some(result.selected.expected.cost_units),
    };
    let report = assure(&intent, &observed);
    println!("\n=== 3. ASSURANCE ===\n{}", serde_json::to_string_pretty(&report)?);

    if report.recommend_replan {
        let replanned = plan(&intent, &CapabilityRegistry::baseline())?;
        println!("\n=== 4. REPLAN RECOMMENDED ===\n{}", explain(&replanned));
    }
    Ok(())
}
