//! Pre-execution action validation model.

/// Outcome of validating one action before execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionValidation {
    /// Whether execution is permitted.
    pub allowed: bool,
    /// Explanation suitable for CLI output.
    pub reason: String,
}

impl ActionValidation {
    /// Constructs an allowed result.
    pub fn allow(reason: impl Into<String>) -> Self {
        Self { allowed: true, reason: reason.into() }
    }

    /// Constructs a rejected result.
    pub fn reject(reason: impl Into<String>) -> Self {
        Self { allowed: false, reason: reason.into() }
    }
}
