//! ACE dataset state.

use aicp_core::CompressionProfile;

/// Compression metrics visible to the control plane through ACE.
#[derive(Debug, Clone, PartialEq)]
pub struct AceDatasetState {
    /// Dataset name.
    pub dataset: String,
    /// Active compression profile.
    pub profile: CompressionProfile,
    /// Ratio `original_bytes / compressed_bytes`.
    pub compression_ratio: f64,
    /// Approximate encode CPU cost in normalized units.
    pub cpu_cost_units: f64,
    /// Approximate decode latency in microseconds.
    pub decode_latency_us: f64,
    /// Original uncompressed bytes.
    pub original_bytes: u64,
    /// Current compressed bytes.
    pub compressed_bytes: u64,
}
