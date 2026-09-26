use sha2::{Digest, Sha256};

/// Computes a deterministic SHA-256 fingerprint for ordered text parts.
pub fn fingerprint(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0u8]);
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fingerprint_is_deterministic() {
        assert_eq!(fingerprint(&["a", "b"]), fingerprint(&["a", "b"]));
    }
}
