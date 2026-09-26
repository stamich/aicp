//! Benchmark time units and conversion factors.

use serde::{Deserialize, Serialize};

/// Supported raw time units emitted by benchmark tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeUnit {
    /// Nanoseconds.
    Nanoseconds,
    /// Microseconds.
    Microseconds,
    /// Milliseconds.
    Milliseconds,
    /// Seconds.
    Seconds,
}

impl TimeUnit {
    /// Returns the multiplicative factor required to convert this unit to nanoseconds.
    pub fn to_ns_factor(self) -> f64 {
        match self {
            Self::Nanoseconds => 1.0,
            Self::Microseconds => 1_000.0,
            Self::Milliseconds => 1_000_000.0,
            Self::Seconds => 1_000_000_000.0,
        }
    }
}
