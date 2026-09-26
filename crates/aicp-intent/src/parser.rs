//! YAML parsing and normalization.

use crate::{error::IntentError, model::IntentDocument, validate::validate_ir};
use aicp_core::{Constraints, Goals, IntentIr, Preferences, Target};

/// Parses and normalizes a v1alpha1 YAML document to canonical IntentIr.
pub fn parse_and_normalize(yaml: &str) -> Result<IntentIr, IntentError> {
    let doc: IntentDocument = serde_yaml::from_str(yaml)?;
    if doc.api_version != "aicp/v1alpha1" { return Err(IntentError::Semantic(format!("unsupported apiVersion {}", doc.api_version))); }
    if doc.kind != "DataIntent" { return Err(IntentError::Semantic(format!("unsupported kind {}", doc.kind))); }
    let ir = IntentIr {
        name: doc.metadata.name,
        revision: doc.metadata.revision,
        target: Target { dataset: doc.spec.target.dataset },
        goals: Goals {
            max_p99_latency_ms: doc.spec.goals.p99_latency_ms.map(|x| x.max),
            min_availability_percent: doc.spec.goals.availability_percent.map(|x| x.min),
        },
        constraints: Constraints { durability: doc.spec.constraints.durability, residency: doc.spec.constraints.residency },
        preferences: Preferences { minimize: doc.spec.preferences.minimize },
    };
    validate_ir(&ir)?;
    Ok(ir)
}
