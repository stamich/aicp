use crate::model::DatasetState;
use aicp_core::StorageStrategy;
use serde::{Deserialize, Serialize};

/// Kinds of drift recognized by AICP 0.2.1.x.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Drift {
    IntentViolation { detail: String },
    ConfigurationDrift { detail: String },
    CapabilityDrift { detail: String },
    PerformanceDrift { detail: String },
}

/// Detects simple configuration drift for an AdaptiveDB dataset.
pub fn detect_storage_drift(dataset: &DatasetState, expected: StorageStrategy) -> Option<Drift> {
    match dataset.storage_strategy {
        Some(actual) if actual != expected => Some(Drift::ConfigurationDrift { detail: format!("dataset {} uses {:?}, expected {:?}", dataset.name, actual, expected) }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_layout_drift() {
        let ds = DatasetState { name: "orders".into(), storage_strategy: Some(StorageStrategy::Column), estimated_rows: 1, size_bytes: 1, p99_latency_ms: None };
        assert!(detect_storage_drift(&ds, StorageStrategy::Row).is_some());
    }
}
