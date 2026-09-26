use aicp_core::{
    CandidatePlan, CompressionProfile, CoordinationStrategy, EngineKind, EngineOperation,
    IntentIr, PlanAction, PlanEstimate, StorageStrategy,
};

/// Builds the transparent baseline strategies considered by the deterministic planner.
pub(crate) fn generate_candidates(intent: &IntentIr) -> Vec<CandidatePlan> {
    let dataset = &intent.target.dataset;
    vec![
        build_candidate(
            "latency-first",
            dataset,
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
            },
        ),
        build_candidate(
            "balanced-adaptive",
            dataset,
            StorageStrategy::Hybrid,
            CompressionProfile::Balanced,
            CoordinationStrategy::GraphScoped,
            PlanEstimate {
                p99_latency_ms: 8.0,
                cost_units: 95.0,
                storage_units: 82.0,
                cpu_units: 82.0,
                network_units: 62.0,
                availability_percent: 99.995,
                strong_durability: true,
            },
        ),
        build_candidate(
            "cost-storage-first",
            dataset,
            StorageStrategy::Column,
            CompressionProfile::Dense,
            CoordinationStrategy::Partition,
            PlanEstimate {
                p99_latency_ms: 24.0,
                cost_units: 62.0,
                storage_units: 48.0,
                cpu_units: 115.0,
                network_units: 55.0,
                availability_percent: 99.95,
                strong_durability: false,
            },
        ),
    ]
}

/// Constructs one candidate from typed engine strategy choices and a fixed estimate.
fn build_candidate(
    name: &str,
    dataset: &str,
    storage: StorageStrategy,
    profile: CompressionProfile,
    coordination: CoordinationStrategy,
    estimate: PlanEstimate,
) -> CandidatePlan {
    CandidatePlan {
        name: name.to_owned(),
        actions: vec![
            PlanAction {
                engine: EngineKind::AdaptiveDb,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetStorage { strategy: storage },
            },
            PlanAction {
                engine: EngineKind::Ace,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetCompression { profile },
            },
            PlanAction {
                engine: EngineKind::GraphNet,
                dataset: dataset.to_owned(),
                operation: EngineOperation::SetCoordination {
                    strategy: coordination,
                },
            },
        ],
        estimate,
        feasible: true,
        rejection_reasons: Vec::new(),
        score: None,
    }
}
