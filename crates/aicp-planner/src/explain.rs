use crate::result::PlanningResult;
use std::fmt::Write;

/// Produces a human-readable explanation of why a plan was selected.
pub fn explain(result: &PlanningResult) -> String {
    let mut output = String::new();
    writeln!(
        output,
        "Selected {} ({})",
        result.selected.strategy_name, result.selected.id
    )
    .expect("writing to String must succeed");
    writeln!(output, "score: {:.3}", result.selected.score)
        .expect("writing to String must succeed");
    writeln!(
        output,
        "expected p99: {:.1} ms",
        result.selected.expected.p99_latency_ms
    )
    .expect("writing to String must succeed");
    writeln!(
        output,
        "expected cost: {:.1} units\n",
        result.selected.expected.cost_units
    )
    .expect("writing to String must succeed");
    writeln!(output, "Candidates:").expect("writing to String must succeed");

    for candidate in &result.candidates {
        if candidate.feasible {
            writeln!(
                output,
                "- {}: feasible, score {:.3}",
                candidate.name,
                candidate.score.expect("feasible candidate must have a score")
            )
            .expect("writing to String must succeed");
        } else {
            writeln!(
                output,
                "- {}: rejected: {}",
                candidate.name,
                candidate.rejection_reasons.join("; ")
            )
            .expect("writing to String must succeed");
        }
    }
    output
}
