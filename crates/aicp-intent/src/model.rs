use aicp_core::{Durability, Objective};
use serde::Deserialize;

/// Top-level v1alpha1 YAML document used only at the input boundary.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IntentDocument {
    pub(crate) api_version: String,
    pub(crate) kind: String,
    pub(crate) metadata: Metadata,
    pub(crate) spec: Spec,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Metadata {
    pub(crate) name: String,
    #[serde(default = "default_revision")]
    pub(crate) revision: u64,
}

fn default_revision() -> u64 { 1 }

#[derive(Debug, Deserialize)]
pub(crate) struct Spec {
    pub(crate) target: TargetDoc,
    #[serde(default)] pub(crate) goals: GoalsDoc,
    #[serde(default)] pub(crate) constraints: ConstraintsDoc,
    #[serde(default)] pub(crate) preferences: PreferencesDoc,
}

#[derive(Debug, Deserialize)]
pub(crate) struct TargetDoc { pub(crate) dataset: String }

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GoalsDoc {
    pub(crate) p99_latency_ms: Option<MaxDoc<u64>>,
    pub(crate) availability_percent: Option<MinDoc<f64>>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct MaxDoc<T> { pub(crate) max: T }
#[derive(Debug, Deserialize)]
pub(crate) struct MinDoc<T> { pub(crate) min: T }

#[derive(Debug, Default, Deserialize)]
pub(crate) struct ConstraintsDoc {
    pub(crate) durability: Option<Durability>,
    #[serde(default)] pub(crate) residency: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct PreferencesDoc {
    #[serde(default)] pub(crate) minimize: Vec<Objective>,
}
