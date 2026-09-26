use aicp_core::StorageStrategy;
use aicp_state::{detect_storage_drift, fingerprint, DatasetState};

#[test]
fn fingerprint_is_deterministic() {
    assert_eq!(fingerprint(&["a", "b"]), fingerprint(&["a", "b"]));
}

#[test]
fn detects_layout_drift() {
    let ds = DatasetState {
        name: "orders".into(),
        storage_strategy: Some(StorageStrategy::Column),
        estimated_rows: 1,
        size_bytes: 1,
        p99_latency_ms: None,
    };
    assert!(detect_storage_drift(&ds, StorageStrategy::Row).is_some());
}
