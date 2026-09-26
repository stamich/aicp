//! AdaptiveDB client abstraction.

use aicp_core::StorageStrategy;
use aicp_plan::ActionEstimate;
use aicp_state::DatasetState;

/// Minimal transport-neutral AdaptiveDB client required by the AICP adapter.
pub trait AdaptiveDbClient {
    /// Returns a human-readable engine version.
    fn version(&self) -> Result<String, String>;
    /// Returns supported physical layouts.
    fn storage_capabilities(&self) -> Result<Vec<StorageStrategy>, String>;
    /// Reads normalized state for all visible datasets.
    fn observe_datasets(&self) -> Result<Vec<DatasetState>, String>;
    /// Applies a storage strategy to one dataset.
    fn set_storage(&mut self, dataset: &str, strategy: StorageStrategy) -> Result<(), String>;
    /// Returns a cheap deterministic estimate for a proposed storage change.
    fn estimate_storage_change(
        &self,
        dataset: &str,
        strategy: StorageStrategy,
    ) -> Result<ActionEstimate, String>;
}
