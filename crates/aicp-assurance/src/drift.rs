//! Conversion of assurance violations into drift records.

use aicp_core::{AssuranceReport, AssuranceStatus};
use aicp_state::Drift;

/// Converts assurance violations into explicit drift records.
pub fn drift_from_assurance(report: &AssuranceReport) -> Vec<Drift> {
    if report.status == AssuranceStatus::Violated {
        report
            .reasons
            .iter()
            .cloned()
            .map(|detail| Drift::IntentViolation { detail })
            .collect()
    } else {
        vec![]
    }
}
