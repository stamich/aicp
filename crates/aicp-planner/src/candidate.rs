//! Cross-engine candidate generation and bounded cheap pruning.

use aicp_core::{
    ActionId, CandidatePlan, CompressionProfile, CoordinationStrategy, EngineKind, EngineOperation,
    IntentIr, PlanAction, PlanEstimate, StorageStrategy,
};
use aicp_state::ObservedState;

/// Generates the bounded AdaptiveDB × ACE strategy product used in 0.3.
pub(crate) fn generate_cross_engine_candidates(
    intent: &IntentIr,
    observed: Option<&ObservedState>,
) -> Vec<CandidatePlan> {
    let dataset = intent.target.dataset.clone();
    let current_latency = observed
        .and_then(|s| s.datasets.iter().find(|x| x.name == dataset))
        .and_then(|x| x.p99_latency_ms)
        .unwrap_or(18.0);
    let storages = [
        StorageStrategy::Row,
        StorageStrategy::Hybrid,
        StorageStrategy::Column,
    ];
    let profiles = [
        CompressionProfile::Fast,
        CompressionProfile::Balanced,
        CompressionProfile::Dense,
    ];
    let mut out = Vec::new();
    for storage in storages {
        for profile in profiles {
            let storage_latency = match storage {
                StorageStrategy::Row => 6.0,
                StorageStrategy::Hybrid => 8.5,
                StorageStrategy::Column => 16.0,
            };
            let compression_latency = match profile {
                CompressionProfile::Fast => 0.10,
                CompressionProfile::Balanced => 0.25,
                CompressionProfile::Dense => 0.60,
            };
            let storage_units = match storage {
                StorageStrategy::Row => 120.0,
                StorageStrategy::Hybrid => 85.0,
                StorageStrategy::Column => 60.0,
            };
            let compression_factor = match profile {
                CompressionProfile::Fast => 0.90,
                CompressionProfile::Balanced => 0.68,
                CompressionProfile::Dense => 0.52,
            };
            let cpu = match profile {
                CompressionProfile::Fast => 48.0,
                CompressionProfile::Balanced => 70.0,
                CompressionProfile::Dense => 105.0,
            };
            let migration = match storage {
                StorageStrategy::Row => {
                    if current_latency <= 7.0 {
                        1.0
                    } else {
                        18.0
                    }
                }
                StorageStrategy::Hybrid => 10.0,
                StorageStrategy::Column => 8.0,
            } + match profile {
                CompressionProfile::Fast => 3.0,
                CompressionProfile::Balanced => 5.0,
                CompressionProfile::Dense => 9.0,
            };
            let cost = match storage {
                StorageStrategy::Row => 120.0,
                StorageStrategy::Hybrid => 92.0,
                StorageStrategy::Column => 70.0,
            } + cpu * 0.10;
            let name = format!("{:?}-{:?}", storage, profile).to_lowercase();
            out.push(candidate(
                &name,
                &dataset,
                storage,
                profile,
                PlanEstimate {
                    p99_latency_ms: storage_latency + compression_latency,
                    cost_units: cost,
                    storage_units: storage_units * compression_factor,
                    cpu_units: cpu,
                    network_units: 60.0 * compression_factor,
                    availability_percent: 99.999,
                    strong_durability: storage != StorageStrategy::Column,
                    migration_cost_units: migration,
                },
            ));
        }
    }
    out
}

/// Keeps planning work bounded by a deterministic cheap pre-score.
pub(crate) fn cheap_prune(candidates: &mut Vec<CandidatePlan>, max_candidates: usize) {
    candidates.sort_by(|a, b| {
        let ac = a.estimate.p99_latency_ms
            + a.estimate.cost_units / 20.0
            + a.estimate.migration_cost_units / 10.0;
        let bc = b.estimate.p99_latency_ms
            + b.estimate.cost_units / 20.0
            + b.estimate.migration_cost_units / 10.0;
        ac.total_cmp(&bc)
    });
    candidates.truncate(max_candidates.max(1));
}

/// Constructs one cross-engine candidate; GraphNet remains a mock coordination target in 0.3.
fn candidate(
    name: &str,
    dataset: &str,
    storage: StorageStrategy,
    profile: CompressionProfile,
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
                    strategy: CoordinationStrategy::Raft,
                },
            },
        ],
        estimate,
        feasible: true,
        rejection_reasons: vec![],
        score: None,
    }
}
