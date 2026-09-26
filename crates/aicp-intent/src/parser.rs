use crate::{error::IntentError, model::IntentDocument, normalize::normalize, validate::validate_document};
use aicp_core::IntentIr;

/// Decodes YAML into the external intent document without semantic normalization.
pub fn parse_document(yaml: &str) -> Result<IntentDocument, IntentError> {
    serde_yaml::from_str(yaml).map_err(IntentError::from)
}

/// Parses YAML, validates the external document, and returns canonical intent IR.
pub fn parse_and_normalize(yaml: &str) -> Result<IntentIr, IntentError> {
    let document = parse_document(yaml)?;
    validate_document(&document)?;
    Ok(normalize(document))
}

#[cfg(test)]
mod tests {
    use super::parse_and_normalize;
    use aicp_core::Durability;

    #[test]
    /// Verifies a representative YAML document normalizes into canonical IR.
    fn parses_valid_intent() {
        let yaml = include_str!("../../../examples/intents/low-latency-orders.yaml");
        let intent = parse_and_normalize(yaml).unwrap();
        assert_eq!(intent.target.dataset, "orders");
        assert_eq!(intent.goals.max_p99_latency_ms, Some(10));
        assert_eq!(intent.constraints.durability, Some(Durability::Strong));
    }

    #[test]
    /// Verifies an invalid zero latency bound is rejected before planning.
    fn rejects_zero_latency() {
        let yaml = concat!(
            "apiVersion: aicp/v1alpha1\n",
            "kind: DataIntent\n",
            "metadata: {name: x}\n",
            "spec:\n",
            "  target: {dataset: d}\n",
            "  goals:\n",
            "    p99LatencyMs: {max: 0}\n",
        );
        assert!(parse_and_normalize(yaml).is_err());
    }
}
