use serde::{Deserialize, Serialize};

/// High-level assurance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceStatus {
    /// All observable required conditions are satisfied.
    Satisfied,
    /// At least one required condition is violated.
    Violated,
    /// Required data is unavailable, so the result cannot be determined safely.
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
