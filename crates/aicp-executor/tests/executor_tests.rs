use aicp_adapter_api::IntentTarget;
use aicp_adapter_mock::MockAdapter;
use aicp_core::{
    ActionId, CompressionProfile, EngineKind, EngineOperation, ExecutionPlan, PlanAction,
    PlanEstimate,
};
use aicp_executor::execute_plan;
use aicp_plan::ExecutionStatus;
use std::collections::HashMap;
use uuid::Uuid;

#[test]
fn executes_registered_actions_in_order() {
    let plan = ExecutionPlan {
        id: Uuid::new_v4(),
        intent_name: "orders".into(),
        intent_revision: 1,
        strategy_name: "test".into(),
        actions: vec![PlanAction {
            id: ActionId("a1".into()),
            engine: EngineKind::Ace,
            dataset: "orders".into(),
            operation: EngineOperation::SetCompression {
                profile: CompressionProfile::Balanced,
            },
        }],
        expected: PlanEstimate {
            p99_latency_ms: 10.0,
            cost_units: 1.0,
            storage_units: 1.0,
            cpu_units: 1.0,
            network_units: 1.0,
            availability_percent: 99.9,
            strong_durability: true,
            migration_cost_units: 1.0,
        },
        score: 1.0,
        fingerprint: "fp".into(),
    };
    let mut adapters: HashMap<EngineKind, Box<dyn IntentTarget>> = HashMap::new();
    adapters.insert(EngineKind::Ace, Box::new(MockAdapter::new(EngineKind::Ace)));
    let receipts = execute_plan(&plan, &mut adapters).unwrap();
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0].status, ExecutionStatus::Applied);
}
