//! Performance change classification.

use serde::{Deserialize, Serialize};

/// Result classification produced by the comparison engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PerformanceClassification {
    /// Meaningful improvement.
    Improved,
    /// No relevant change.
    Stable,
    /// Small regression worth observing.
    Warning,
    /// Material performance regression.
    Regression,
    /// Measurement likely contains a scale/unit/environment problem.
    Suspicious,
    /// Measurement cannot be compared safely.
    Invalid,
}
