use std::sync::Mutex;

/// Shared in-memory log used by mock adapters and tests.
#[derive(Debug, Default)]
pub struct MockLog {
    entries: Mutex<Vec<String>>,
}

impl MockLog {
    /// Appends one audit line.
    pub fn push(&self, value: String) {
        self.entries
            .lock()
            .expect("mock log mutex poisoned")
            .push(value);
    }

    /// Returns a snapshot of all audit lines.
    pub fn entries(&self) -> Vec<String> {
        self.entries
            .lock()
            .expect("mock log mutex poisoned")
            .clone()
    }
}
