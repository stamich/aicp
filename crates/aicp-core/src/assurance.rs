use serde::{Deserialize, Serialize};

/// High-level assurance state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssuranceStatus { Satisfied, Degraded, Violated, Unknown }

/// Result of comparing observed state with the requested intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssuranceReport {
    pub status: AssuranceStatus,
    pub reasons: Vec<String>,
    pub recommend_replan: bool,
}
