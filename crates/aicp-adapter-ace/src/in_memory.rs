//! Deterministic in-memory ACE client.

use crate::{client::AceClient, model::AceDatasetState};
use aicp_core::CompressionProfile;
use aicp_plan::ActionEstimate;
use std::collections::HashMap;

/// Deterministic ACE client used by the 0.3 demo and test suite.
pub struct InMemoryAceClient {
    version: String,
    datasets: HashMap<String, AceDatasetState>,
}

impl InMemoryAceClient {
    /// Creates a client with one dataset using the supplied profile.
    pub fn with_dataset(dataset: &str, profile: CompressionProfile, original_bytes: u64) -> Self {
        let state = model_state(dataset, profile, original_bytes);
        Self {
            version: "ace-contract-0.x".into(),
            datasets: HashMap::from([(dataset.into(), state)]),
        }
    }
}

impl AceClient for InMemoryAceClient {
    /// Returns the demo contract version.
    fn version(&self) -> Result<String, String> {
        Ok(self.version.clone())
    }

    /// Advertises all milestone-0.3 profiles.
    fn profiles(&self) -> Result<Vec<CompressionProfile>, String> {
        Ok(vec![
            CompressionProfile::Fast,
            CompressionProfile::Balanced,
            CompressionProfile::Dense,
        ])
    }

    /// Returns a copy of all compression states.
    fn observe_datasets(&self) -> Result<Vec<AceDatasetState>, String> {
        Ok(self.datasets.values().cloned().collect())
    }

    /// Estimates profile change using a deterministic model.
    fn estimate_profile_change(
        &self,
        dataset: &str,
        profile: CompressionProfile,
    ) -> Result<ActionEstimate, String> {
        let current = self
            .datasets
            .get(dataset)
            .ok_or_else(|| format!("unknown dataset {dataset}"))?;
        let next = model_state(dataset, profile, current.original_bytes);
        Ok(ActionEstimate {
            migration_cost_units: match profile {
                CompressionProfile::Fast => 3.0,
                CompressionProfile::Balanced => 5.0,
                CompressionProfile::Dense => 9.0,
            },
            latency_delta_ms: (next.decode_latency_us - current.decode_latency_us) / 1_000.0,
            storage_delta_percent: ((next.compressed_bytes as f64
                / current.compressed_bytes as f64)
                - 1.0)
                * 100.0,
            confidence: 0.92,
        })
    }

    /// Applies a profile and refreshes deterministic compression metrics.
    fn set_profile(&mut self, dataset: &str, profile: CompressionProfile) -> Result<(), String> {
        let original = self
            .datasets
            .get(dataset)
            .ok_or_else(|| format!("unknown dataset {dataset}"))?
            .original_bytes;
        self.datasets
            .insert(dataset.into(), model_state(dataset, profile, original));
        Ok(())
    }
}

/// Creates deterministic compression metrics for one profile.
fn model_state(dataset: &str, profile: CompressionProfile, original_bytes: u64) -> AceDatasetState {
    let (ratio, cpu, latency) = match profile {
        CompressionProfile::Fast => (1.8, 35.0, 90.0),
        CompressionProfile::Balanced => (2.7, 60.0, 160.0),
        CompressionProfile::Dense => (3.8, 95.0, 340.0),
    };
    AceDatasetState {
        dataset: dataset.into(),
        profile,
        compression_ratio: ratio,
        cpu_cost_units: cpu,
        decode_latency_us: latency,
        original_bytes,
        compressed_bytes: (original_bytes as f64 / ratio) as u64,
    }
}
