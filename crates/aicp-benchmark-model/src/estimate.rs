//! Raw and normalized benchmark estimates.

use crate::unit::TimeUnit;
use serde::{Deserialize, Serialize};

/// Raw confidence interval exactly as read from the benchmark source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawEstimate {
    /// Lower confidence bound.
    pub low: f64,
    /// Central estimate.
    pub mean: f64,
    /// Upper confidence bound.
    pub high: f64,
    /// Unit used by the source representation.
    pub unit: TimeUnit,
}

impl RawEstimate {
    /// Converts all values to the canonical nanosecond representation.
    pub fn normalize_to_ns(&self) -> NormalizedEstimate {
        let factor = self.unit.to_ns_factor();
        NormalizedEstimate {
            low_ns: self.low * factor,
            mean_ns: self.mean * factor,
            high_ns: self.high * factor,
        }
    }
}

/// Canonical benchmark estimate used for every AICP comparison.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalizedEstimate {
    /// Lower confidence bound in nanoseconds.
    pub low_ns: f64,
    /// Central estimate in nanoseconds.
    pub mean_ns: f64,
    /// Upper confidence bound in nanoseconds.
    pub high_ns: f64,
}
