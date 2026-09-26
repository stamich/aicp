//! Decision explanation graph.

use crate::reason::{DecisionReason, ReasonId, ReasonKind};
use serde::{Deserialize, Serialize};

/// Directed acyclic explanation graph attached to a planning decision.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionGraph {
    /// All explanation nodes.
    pub reasons: Vec<DecisionReason>,
}

impl DecisionGraph {
    /// Creates an empty explanation graph.
    pub fn new() -> Self { Self::default() }

    /// Appends a reason node and returns its identifier.
    pub fn push(&mut self, kind: ReasonKind, message: impl Into<String>, parents: Vec<ReasonId>) -> ReasonId {
        let id = ReasonId(format!("r{}", self.reasons.len() + 1));
        self.reasons.push(DecisionReason { id: id.clone(), kind, message: message.into(), parents });
        id
    }
}
