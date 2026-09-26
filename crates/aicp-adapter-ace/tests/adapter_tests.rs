use aicp_adapter_ace::{AceAdapter, AceClient, InMemoryAceClient};
use aicp_adapter_api::IntentTarget;
use aicp_core::{ActionId, CompressionProfile, EngineKind, EngineOperation, PlanAction};
use aicp_plan::ExecutionStatus;

#[test]
fn dense_profile_reduces_storage() {
    let mut client = InMemoryAceClient::with_dataset("orders", CompressionProfile::Fast, 1_000_000);
    let fast = client.observe_datasets().unwrap()[0].compressed_bytes;
    client
        .set_profile("orders", CompressionProfile::Dense)
        .unwrap();
    let dense = client.observe_datasets().unwrap()[0].compressed_bytes;
    assert!(dense < fast);
}

#[test]
fn execute_is_idempotent() {
    let mut adapter = AceAdapter::new(InMemoryAceClient::with_dataset(
        "orders",
        CompressionProfile::Fast,
        1_000_000,
    ));
    let action = PlanAction {
        id: ActionId("ace-1".into()),
        engine: EngineKind::Ace,
        dataset: "orders".into(),
        operation: EngineOperation::SetCompression {
            profile: CompressionProfile::Balanced,
        },
    };
    assert_eq!(
        adapter.execute("p1", &action).unwrap().status,
        ExecutionStatus::Applied
    );
    assert_eq!(
        adapter.execute("p1", &action).unwrap().status,
        ExecutionStatus::AlreadyApplied
    );
}
