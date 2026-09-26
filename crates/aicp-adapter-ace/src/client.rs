//! ACE client abstraction.

use crate::model::AceDatasetState;
use aicp_core::CompressionProfile;
use aicp_plan::ActionEstimate;

/// Minimal client contract required from a real ACE implementation.
pub trait AceClient {
    /// Returns engine version.
    fn version(&self) -> Result<String, String>;
    /// Returns supported compression profiles.
    fn profiles(&self) -> Result<Vec<CompressionProfile>, String>;
    /// Returns compression state for managed datasets.
    fn observe_datasets(&self) -> Result<Vec<AceDatasetState>, String>;
    /// Estimates the impact of changing a dataset profile.
    fn estimate_profile_change(
        &self,
        dataset: &str,
        profile: CompressionProfile,
    ) -> Result<ActionEstimate, String>;
    /// Applies a new compression profile.
    fn set_profile(&mut self, dataset: &str, profile: CompressionProfile) -> Result<(), String>;
}
