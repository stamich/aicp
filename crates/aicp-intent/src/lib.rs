//! Intent parsing, normalization and semantic validation.

use aicp_core::{Constraints, Durability, Goals, IntentIr, Objective, Preferences, Target};
use serde::Deserialize;
use thiserror::Error;

/// Errors produced while parsing or validating an intent document.
#[derive(Debug, Error)]
pub enum IntentError {
    /// YAML syntax or deserialization failure.
    #[error("invalid YAML intent: {0}")]
    Parse(#[from] serde_yaml::Error),
    /// Semantic validation failure.
    #[error("invalid intent: {0}")]
    Validation(String),
}

/// External v1alpha1 intent document accepted by AICP 0.1.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentDocument {
    /// API version. Milestone 0.1 accepts only `aicp/v1alpha1`.
    pub api_version: String,
    /// Resource kind. Milestone 0.1 accepts only `DataIntent`.
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
    pub max: u64,
}

/// Inclusive lower bound for a floating-point value.
#[derive(Debug, Deserialize)]
pub struct LowerF64 {
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

/// Parses YAML and returns a normalized, semantically validated intent IR.
pub fn parse_and_normalize(yaml: &str) -> Result<IntentIr, IntentError> {
    let doc: IntentDocument = serde_yaml::from_str(yaml)?;
    validate_document(&doc)?;
    Ok(IntentIr {
        name: doc.metadata.name.trim().to_owned(),
        target: Target {
            dataset: doc.spec.target.dataset.trim().to_owned(),
        },
        goals: Goals {
            max_p99_latency_ms: doc.spec.goals.p99_latency_ms.map(|v| v.max),
            min_availability_percent: doc.spec.goals.availability_percent.map(|v| v.min),
        },
        constraints: Constraints {
            durability: doc.spec.constraints.durability,
            residency: doc
                .spec
                .constraints
                .residency
                .into_iter()
                .map(|s| s.trim().to_uppercase())
                .collect(),
        },
        preferences: Preferences {
            minimize: doc.spec.preferences.minimize,
        },
    })
}

/// Validates semantic invariants in a decoded v1alpha1 intent document.
pub fn validate_document(doc: &IntentDocument) -> Result<(), IntentError> {
    if doc.api_version != "aicp/v1alpha1" {
        return Err(IntentError::Validation(format!(
            "unsupported apiVersion '{}'",
            doc.api_version
        )));
    }
    if doc.kind != "DataIntent" {
        return Err(IntentError::Validation(format!(
            "unsupported kind '{}'",
            doc.kind
        )));
    }
    if doc.metadata.name.trim().is_empty() {
        return Err(IntentError::Validation(
            "metadata.name must not be empty".into(),
        ));
    }
    if doc.spec.target.dataset.trim().is_empty() {
        return Err(IntentError::Validation(
            "spec.target.dataset must not be empty".into(),
        ));
    }
    if let Some(v) = &doc.spec.goals.p99_latency_ms {
        if v.max == 0 {
            return Err(IntentError::Validation(
                "p99 latency must be greater than zero".into(),
            ));
        }
    }
    if let Some(v) = &doc.spec.goals.availability_percent {
        if !(0.0..=100.0).contains(&v.min) {
            return Err(IntentError::Validation(
                "availability must be between 0 and 100".into(),
            ));
        }
    }
    Ok(())
}

/// Re-validates invariants on an already normalized intent IR.
pub fn validate_ir(ir: &IntentIr) -> Result<(), IntentError> {
    if ir.name.trim().is_empty() {
        return Err(IntentError::Validation(
            "intent name must not be empty".into(),
        ));
    }
    if ir.target.dataset.trim().is_empty() {
        return Err(IntentError::Validation(
            "target dataset must not be empty".into(),
        ));
    }
    if ir.goals.max_p99_latency_ms == Some(0) {
        return Err(IntentError::Validation(
            "p99 latency must be greater than zero".into(),
        ));
    }
    if let Some(a) = ir.goals.min_availability_percent {
        if !(0.0..=100.0).contains(&a) {
            return Err(IntentError::Validation(
                "availability must be between 0 and 100".into(),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensures a representative intent becomes normalized IR.
    #[test]
    fn parses_valid_intent() {
        let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
        let ir = parse_and_normalize(yaml).unwrap();
        assert_eq!(ir.target.dataset, "orders");
        assert_eq!(ir.goals.max_p99_latency_ms, Some(10));
        assert_eq!(ir.constraints.durability, Some(Durability::Strong));
    }

    /// Ensures impossible latency bounds are rejected before planning.
    #[test]
    fn rejects_zero_latency() {
        let yaml = "apiVersion: aicp/v1alpha1\nkind: DataIntent\nmetadata: {name: x}\nspec:\n  target: {dataset: d}\n  goals:\n    p99LatencyMs: {max: 0}\n";
        assert!(parse_and_normalize(yaml).is_err());
    }
}
