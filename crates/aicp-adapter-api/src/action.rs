/// Result token returned after a successful engine operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedAction {
    /// Human-readable engine-specific token useful for auditing and rollback.
    pub token: String,
}
