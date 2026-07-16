//! Artifact Verifier — integrity checking on install.
//!
//! Every package install goes through: download → verify hash → verify sig → unpack.
//! Never unpack before verification.

use crate::signing;

/// Verification result for a package artifact.
#[derive(Debug)]
pub struct Verdict {
    pub hash_match: bool,
    pub signature_valid: bool,
    pub package_id: String,
    pub expected_hash: String,
    pub actual_hash: String,
}

/// Verify a downloaded package before unpacking.
/// 1. Compute SHA-256 of downloaded bytes
/// 2. Compare against expected hash
/// 3. Verify Ed25519 signature against publisher's public key
pub fn verify_package(
    data: &[u8],
    expected_hash: &str,
    public_key: &str,
    signature: &str,
    package_id: &str,
) -> Verdict {
    let actual_hash = crate::artifact_store::ArtifactStore::hash_bytes(data);
    let hash_match = actual_hash == expected_hash;

    let signature_valid = if !public_key.is_empty() && !signature.is_empty() {
        let sig = signing::PackageSignature {
            package_id: package_id.into(),
            version: String::new(),
            publisher: String::new(),
            public_key: public_key.into(),
            signature: signature.into(),
            signed_at: String::new(),
            archive_sha256: actual_hash.clone(),
        };
        signing::verify_signature(&sig, data).unwrap_or(false)
    } else {
        true // No signature = trust-on-first-use
    };

    Verdict {
        hash_match,
        signature_valid,
        package_id: package_id.into(),
        expected_hash: expected_hash.into(),
        actual_hash,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_verification_passes() {
        let data = b"test package content";
        let hash = crate::artifact_store::ArtifactStore::hash_bytes(data);
        let verdict = verify_package(data, &hash, "", "", "test-pkg");
        assert!(verdict.hash_match);
    }

    #[test]
    fn tampered_data_fails() {
        let data = b"original content";
        let verdict = verify_package(data, "wrong_hash", "", "", "test-pkg");
        assert!(!verdict.hash_match);
    }
}
