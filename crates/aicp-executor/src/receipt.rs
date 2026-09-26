/// Successful execution receipt.
#[derive(Debug, Clone)]
pub struct ExecutionReceipt {
    /// Number of actions successfully applied.
    pub applied_actions: usize,
}
