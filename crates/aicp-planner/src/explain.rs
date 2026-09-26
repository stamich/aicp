//! Human-readable plan explanations.

use crate::result::PlanningResult;

/// Produces a ranked human-readable explanation including rejected alternatives.
pub fn explain(result: &PlanningResult) -> String {
    let mut candidates = result.candidates.clone();
    candidates.sort_by(|a, b| match (a.score, b.score) {
        (Some(x), Some(y)) => x.total_cmp(&y),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.name.cmp(&b.name),
    });
    let mut out = format!("Selected {} ({})\nscore: {:.4}\nfingerprint: {}\nexpected p99: {:.2} ms\nexpected migration cost: {:.1}\n\nCandidate ranking:\n", result.selected.strategy_name, result.selected.id, result.selected.score, result.selected.fingerprint, result.selected.expected.p99_latency_ms, result.selected.expected.migration_cost_units);
    for (i, c) in candidates.iter().enumerate() {
        if c.feasible {
            out.push_str(&format!(
                "#{} {}: feasible score={:.4}\n",
                i + 1,
                c.name,
                c.score.unwrap()
            ));
        } else {
            out.push_str(&format!(
                "#{} {}: rejected: {}\n",
                i + 1,
                c.name,
                c.rejection_reasons.join("; ")
            ));
        }
    }
    out.push_str("\nDecision graph:\n");
    for reason in &result.decision_graph.reasons {
        out.push_str(&format!(
            "{} {:?}: {}\n",
            reason.id.0, reason.kind, reason.message
        ));
    }
    out
}

/// Explains why a named candidate was not selected.
pub fn why_not(result: &PlanningResult, candidate_name: &str) -> String {
    match result.candidates.iter().find(|c| c.name == candidate_name) {
        None => format!("candidate {candidate_name} does not exist"),
        Some(c) if !c.feasible => format!(
            "{candidate_name} rejected because {}",
            c.rejection_reasons.join("; ")
        ),
        Some(c) if c.name == result.selected.strategy_name => {
            format!("{candidate_name} was selected")
        }
        Some(c) => format!(
            "{candidate_name} was feasible but score {:.4} was worse than selected {:.4}",
            c.score.unwrap_or(f64::INFINITY),
            result.selected.score
        ),
    }
}
