//! Command-line interface for inspecting milestone-0.2.1 intents and plans.

use aicp_adapter_adaptive_db::{AdaptiveDbAdapter, InMemoryAdaptiveDbClient};
use aicp_adapter_api::IntentTarget;
use aicp_capability::CapabilityRegistry;
use aicp_core::StorageStrategy;
use aicp_intent::parse_and_normalize;
use aicp_planner::{explain, plan, why_not};
use anyhow::Context;
use clap::{Parser, Subcommand};
use std::fs;

/// AICP command-line options.
#[derive(Parser)]
#[command(name = "aicp", version, about = "Adaptive Intent Control Plane 0.2.1")]
struct Cli {
    /// Requested subcommand.
    #[command(subcommand)]
    command: Command,
}

/// Supported milestone-0.2.1 CLI commands.
#[derive(Subcommand)]
enum Command {
    /// Builds and explains a plan for an intent file.
    Plan { file: String },
    /// Explains why a candidate was not selected.
    WhyNot { file: String, candidate: String },
    /// Observes the demo AdaptiveDB adapter.
    Observe { dataset: String },
}

/// Loads one intent file, discovers current state and executes the requested read-only command.
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Observe { dataset } => {
            let adapter = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
                &dataset,
                StorageStrategy::Column,
                18.0,
            ));
            println!("{:#?}", adapter.observe()?);
        }
        Command::Plan { file } => {
            let yaml = fs::read_to_string(&file).with_context(|| format!("cannot read {file}"))?;
            let intent = parse_and_normalize(&yaml)?;
            let adapter = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
                &intent.target.dataset,
                StorageStrategy::Column,
                18.0,
            ));
            let state = adapter.observe()?;
            let result = plan(&intent, &CapabilityRegistry::baseline(), Some(&state))?;
            println!("{}", explain(&result));
        }
        Command::WhyNot { file, candidate } => {
            let yaml = fs::read_to_string(&file).with_context(|| format!("cannot read {file}"))?;
            let intent = parse_and_normalize(&yaml)?;
            let adapter = AdaptiveDbAdapter::new(InMemoryAdaptiveDbClient::with_dataset(
                &intent.target.dataset,
                StorageStrategy::Column,
                18.0,
            ));
            let state = adapter.observe()?;
            let result = plan(&intent, &CapabilityRegistry::baseline(), Some(&state))?;
            println!("{}", why_not(&result, &candidate));
        }
    }
    Ok(())
}
