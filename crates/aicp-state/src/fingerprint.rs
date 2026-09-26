//! Stable state fingerprinting.

use sha2::{Digest, Sha256};

/// Computes a deterministic SHA-256 fingerprint for arbitrary serializable state text.
pub fn fingerprint(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0u8]);
    }
    format!("{:x}", hasher.finalize())
}
