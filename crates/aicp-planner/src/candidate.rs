use aicp_core::{
    ActionId, CandidatePlan, CompressionProfile, CoordinationStrategy, EngineKind, EngineOperation,
    IntentIr, PlanAction, PlanEstimate, StorageStrategy,
};
use aicp_state::ObservedState;

/// Creates transparent candidate strategies from intent plus current state.
pub(crate) fn generate_candidates(
    intent: &IntentIr,
    observed: Option<&ObservedState>,
) -> Vec<CandidatePlan> {
    let dataset = intent.target.dataset.clone();
    let current_latency = observed
        .and_then(|s| s.datasets.iter().find(|x| x.name == dataset))
        .and_then(|x| x.p99_latency_ms)
        .unwrap_or(18.0);
    vec![
        candidate(
            "latency-first",
            &dataset,
            StorageStrategy::Row,
            CompressionProfile::Fast,
            CoordinationStrategy::Raft,
            PlanEstimate {
                p99_latency_ms: 6.0,
                cost_units: 130.0,
                storage_units: 125.0,
                cpu_units: 70.0,
                network_units: 80.0,
                availability_percent: 99.999,
                strong_durability: true,
                migration_cost_units: if current_latency <= 7.0 { 1.0 } else { 18.0 },
            },
        ),
        candidate(
            "balanced-adaptive",
            &dataset,
            StorageStrategy::Hybrid,
            CompressionProfile::Balanced,
            CoordinationStrategy::GraphScoped,
            PlanEstimate {
                p99_latency_ms: 8.7,
                cost_units: 95.0,
                storage_units: 82.0,
                cpu_units: 82.0,
                network_units: 62.0,
                availability_percent: 99.995,
                strong_durability: true,
                migration_cost_units: 10.0,
            },
        ),
        candidate(
            "cost-storage-first",
            &dataset,
            StorageStrategy::Column,
            CompressionProfile::Dense,
            CoordinationStrategy::Partition,
            PlanEstimate {
                p99_latency_ms: 18.0,
                cost_units: 62.0,
                storage_units: 48.0,
                cpu_units: 115.0,
                network_units: 55.0,
                availability_percent: 99.95,
                strong_durability: false,
                migration_cost_units: 8.0,
            },
        ),
    ]
}

fn candidate(
    name: &str,
    dataset: &str,
    storage: StorageStrategy,
    profile: CompressionProfile,
    coordination: CoordinationStrategy,
    estimate: PlanEstimate,
) -> CandidatePlan {
    CandidatePlan {
        name: name.into(),
        actions: vec![
            PlanAction {
                id: ActionId(format!("{name}-adb")),
                engine: EngineKind::AdaptiveDb,
                dataset: dataset.into(),
                operation: EngineOperation::SetStorage { strategy: storage },
            },
            PlanAction {
                id: ActionId(format!("{name}-ace")),
                engine: EngineKind::Ace,
                dataset: dataset.into(),
                operation: EngineOperation::SetCompression { profile },
            },
            PlanAction {
                id: ActionId(format!("{name}-graphnet")),
                engine: EngineKind::GraphNet,
                dataset: dataset.into(),
                operation: EngineOperation::SetCoordination {
                    strategy: coordination,
                },
            },
        ],
        estimate,
        feasible: true,
        rejection_reasons: vec![],
        score: None,
    }
}
