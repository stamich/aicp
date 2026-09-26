//! Deterministic in-memory AdaptiveDB client.

use crate::client::AdaptiveDbClient;
use aicp_core::StorageStrategy;
use aicp_plan::ActionEstimate;
use aicp_state::DatasetState;
use std::collections::HashMap;

/// Executable local AdaptiveDB client used by demo and tests.
///
/// It models the same contract a future FFI/RPC implementation must satisfy and therefore
/// exercises the actual AICP adapter instead of bypassing it with a mock adapter.
pub struct InMemoryAdaptiveDbClient {
    version: String,
    datasets: HashMap<String, DatasetState>,
}

impl InMemoryAdaptiveDbClient {
    /// Creates a client seeded with one dataset and its observed latency.
    pub fn with_dataset(name: &str, storage: StorageStrategy, p99_latency_ms: f64) -> Self {
        let mut datasets = HashMap::new();
        datasets.insert(
            name.into(),
            DatasetState {
                name: name.into(),
                storage_strategy: Some(storage),
                estimated_rows: 1_000_000,
                size_bytes: 512 * 1024 * 1024,
                p99_latency_ms: Some(p99_latency_ms),
            },
        );
        Self {
            version: "adaptive-db-contract-2.x".into(),
            datasets,
        }
    }
}

impl AdaptiveDbClient for InMemoryAdaptiveDbClient {
    /// Returns the contract version.
    fn version(&self) -> Result<String, String> {
        Ok(self.version.clone())
    }

    /// Reports layouts available in the local demo client.
    fn storage_capabilities(&self) -> Result<Vec<StorageStrategy>, String> {
        Ok(vec![
            StorageStrategy::Row,
            StorageStrategy::Column,
            StorageStrategy::Hybrid,
        ])
    }

    /// Returns a copy of current dataset state.
    fn observe_datasets(&self) -> Result<Vec<DatasetState>, String> {
        Ok(self.datasets.values().cloned().collect())
    }

    /// Applies a physical layout and updates the demo latency model.
    fn set_storage(&mut self, dataset: &str, strategy: StorageStrategy) -> Result<(), String> {
        let state = self
            .datasets
            .get_mut(dataset)
            .ok_or_else(|| format!("unknown dataset {dataset}"))?;
        state.storage_strategy = Some(strategy);
        state.p99_latency_ms = Some(match strategy {
            StorageStrategy::Row => 6.3,
            StorageStrategy::Hybrid => 8.7,
            StorageStrategy::Column => 18.0,
        });
        Ok(())
    }

    /// Estimates migration cost and latency impact from the current layout.
    fn estimate_storage_change(
        &self,
        dataset: &str,
        strategy: StorageStrategy,
    ) -> Result<ActionEstimate, String> {
        let state = self
            .datasets
            .get(dataset)
            .ok_or_else(|| format!("unknown dataset {dataset}"))?;
        let current = state.p99_latency_ms.unwrap_or(20.0);
        let target = match strategy {
            StorageStrategy::Row => 6.0,
            StorageStrategy::Hybrid => 9.0,
            StorageStrategy::Column => 18.0,
        };
        Ok(ActionEstimate {
            migration_cost_units: match strategy {
                StorageStrategy::Row => 18.0,
                StorageStrategy::Hybrid => 10.0,
                StorageStrategy::Column => 8.0,
            },
            latency_delta_ms: target - current,
            storage_delta_percent: match strategy {
                StorageStrategy::Row => 20.0,
                StorageStrategy::Hybrid => 8.0,
                StorageStrategy::Column => -25.0,
            },
            confidence: 0.9,
        })
    }
}
