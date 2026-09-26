//! Configuration drift detection.

use crate::{DatasetState, Drift};
use aicp_core::StorageStrategy;

/// Detects simple configuration drift for an AdaptiveDB dataset.
pub fn detect_storage_drift(dataset: &DatasetState, expected: StorageStrategy) -> Option<Drift> {
    match dataset.storage_strategy {
        Some(actual) if actual != expected => Some(Drift::ConfigurationDrift {
            detail: format!(
                "dataset {} uses {:?}, expected {:?}",
                dataset.name, actual, expected
            ),
        }),
        _ => None,
    }
}
