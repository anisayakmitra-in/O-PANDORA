//! Package Signing — key generation, signing, and verification.
//! Uses timestamp-based keys. Real Ed25519 requires `ed25519-dalek` crate.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherKeyPair {
    pub public_key: String,
    pub secret_key: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    pub package_id: String,
    pub version: String,
    pub publisher: String,
    pub public_key: String,
    pub signature: String,
    pub signed_at: String,
    pub archive_sha256: String,
}

fn rand_key() -> String {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("pk-{:x}", t)
}

pub fn generate_keypair() -> PublisherKeyPair {
    PublisherKeyPair {
        public_key: rand_key(),
        secret_key: rand_key(),
        created_at: chrono::Utc::now().to_rfc3339(),
    }
}
pub fn sign_package(
    _a: &str,
    _b: &str,
    _c: &str,
    _d: &[u8],
    _e: &str,
) -> Result<PackageSignature, String> {
    Err("ed25519 feature not enabled".into())
}
pub fn verify_signature(_s: &PackageSignature) -> Result<bool, String> {
    Err("ed25519 feature not enabled".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keypair_generates() {
        let kp = generate_keypair();
        assert!(!kp.public_key.is_empty());
    }
}
