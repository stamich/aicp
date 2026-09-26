//! Typed decision reasons.

use serde::{Deserialize, Serialize};

/// Stable identifier of one decision reason node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReasonId(pub String);

/// Semantic category of a planner explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonKind {
    /// Candidate satisfies a hard requirement.
    ConstraintSatisfied,
    /// Candidate violates a hard requirement.
    ConstraintRejected,
    /// Candidate is preferred because of a lower normalized cost.
    CostPreference,
    /// Candidate is preferred because of lower expected latency.
    LatencyPreference,
    /// Candidate was removed to keep planning inside the configured budget.
    Pruned,
    /// Candidate was selected as the best feasible alternative.
    Selected,
}

/// One node in an explainable decision graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionReason {
    /// Stable reason identifier.
    pub id: ReasonId,
    /// Reason category.
    pub kind: ReasonKind,
    /// Human-readable explanation.
    pub message: String,
    /// Parent reasons that logically support this node.
    pub parents: Vec<ReasonId>,
}
