use aicp_state::{fingerprint, ObservedState};

/// Produces a stable compact state fingerprint without leaking engine internals.
pub(crate) fn state_fingerprint(state: &ObservedState) -> String { fingerprint(&[&format!("{:?}", state)]) }
