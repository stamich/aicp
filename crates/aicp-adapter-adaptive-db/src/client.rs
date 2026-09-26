use aicp_core::StorageStrategy;
use aicp_plan::ActionEstimate;
use aicp_state::DatasetState;

/// Minimal transport-neutral AdaptiveDB client required by the AICP adapter.
pub trait AdaptiveDbClient {
    fn version(&self) -> Result<String, String>;
    fn storage_capabilities(&self) -> Result<Vec<StorageStrategy>, String>;
    fn observe_datasets(&self) -> Result<Vec<DatasetState>, String>;
    fn set_storage(&mut self, dataset: &str, strategy: StorageStrategy) -> Result<(), String>;
    fn estimate_storage_change(
        &self,
        dataset: &str,
        strategy: StorageStrategy,
    ) -> Result<ActionEstimate, String>;
}
