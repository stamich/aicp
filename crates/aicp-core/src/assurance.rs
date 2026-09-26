//! Assurance result model.

use serde::{Deserialize, Serialize};

/// High-level assurance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceStatus {
    /// All required observable conditions are satisfied.
    Satisfied,
    /// Conditions are currently satisfied but close to a configured objective boundary.
    Degraded,
    /// At least one hard requirement is violated.
    Violated,
    /// Required data is unavailable.
    Unknown,
}

/// Result of comparing observed state with the requested intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssuranceReport {
    /// Overall assurance state.
    pub status: AssuranceStatus,
    /// Human-readable evidence or violations.
    pub reasons: Vec<String>,
    /// Whether the control plane should generate a fresh plan.
    pub recommend_replan: bool,
}
