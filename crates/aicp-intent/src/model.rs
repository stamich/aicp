use aicp_core::{Durability, Objective};
use serde::Deserialize;

/// External v1alpha1 intent document accepted by the baseline AICP parser.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentDocument {
    /// API version. The baseline schema accepts only `aicp/v1alpha1`.
    pub api_version: String,
    /// Resource kind. The baseline schema accepts only `DataIntent`.
    pub kind: String,
    /// User-facing metadata.
    pub metadata: Metadata,
    /// Intent specification.
    pub spec: Spec,
}

/// User-facing intent metadata.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// Stable human-readable name.
    pub name: String,
}

/// Intent specification in the wire format.
#[derive(Debug, Deserialize)]
pub struct Spec {
    /// Target dataset.
    pub target: TargetWire,
    /// Optional measurable goals.
    #[serde(default)]
    pub goals: GoalsWire,
    /// Optional hard constraints.
    #[serde(default)]
    pub constraints: ConstraintsWire,
    /// Optional soft preferences.
    #[serde(default)]
    pub preferences: PreferencesWire,
}

/// Wire representation of the target.
#[derive(Debug, Deserialize)]
pub struct TargetWire {
    /// Dataset name supplied by the user.
    pub dataset: String,
}

/// Wire representation of goals.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalsWire {
    /// Maximum p99 latency bound.
    pub p99_latency_ms: Option<UpperU64>,
    /// Minimum availability bound.
    pub availability_percent: Option<LowerF64>,
}

/// Inclusive upper bound for an unsigned value.
#[derive(Debug, Deserialize)]
pub struct UpperU64 {
    /// Inclusive maximum value.
    pub max: u64,
}

/// Inclusive lower bound for a floating-point value.
#[derive(Debug, Deserialize)]
pub struct LowerF64 {
    /// Inclusive minimum value.
    pub min: f64,
}

/// Wire representation of hard constraints.
#[derive(Debug, Default, Deserialize)]
pub struct ConstraintsWire {
    /// Durability requirement.
    pub durability: Option<Durability>,
    /// Residency jurisdictions.
    #[serde(default)]
    pub residency: Vec<String>,
}

/// Wire representation of soft optimization preferences.
#[derive(Debug, Default, Deserialize)]
pub struct PreferencesWire {
    /// Ordered list of objectives to minimize.
    #[serde(default)]
    pub minimize: Vec<Objective>,
}
