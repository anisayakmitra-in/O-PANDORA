//! Package Signing — real Ed25519 keys and signatures via `ring` crate.
//!
//! Key generation uses OS random (getrandom). Signing/verification uses
//! Ed25519 via the `ring` crate. Verifiable, cryptographic package trust.

use rand::Rng;
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublisherKeyPair {
    pub public_key: String,  // base64 encoded
    pub secret_key: String,  // base64 encoded (keep secure!)
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    pub package_id: String,
    pub version: String,
    pub publisher: String,
    pub public_key: String,
    pub signature: String,   // base64 encoded Ed25519 signature
    pub signed_at: String,
    pub archive_sha256: String,
}

/// Generate a real Ed25519 keypair using OS randomness (ring::rand::SystemRandom).
pub fn generate_keypair() -> PublisherKeyPair {
    let rng = SystemRandom::new();
    let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Failed to generate Ed25519 keypair");
    let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
        .expect("Failed to parse generated keypair");

    let public_key = key_pair.public_key().as_ref();
    let secret_key = pkcs8_bytes.as_ref();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    PublisherKeyPair {
        public_key: base64_encode(public_key),
        secret_key: base64_encode(secret_key),
        created_at: now.to_string(),
    }
}

/// Sign package metadata with an Ed25519 secret key.
pub fn sign_package(
    package_id: &str,
    version: &str,
    publisher: &str,
    secret_key_b64: &str,
    archive_hash: &str,
) -> Result<PackageSignature, String> {
    let pkcs8_bytes = base64_decode(secret_key_b64)
        .map_err(|e| format!("Invalid secret key: {e}"))?;
    let key_pair = Ed25519KeyPair::from_pkcs8(&pkcs8_bytes)
        .map_err(|e| format!("Failed to load key: {e}"))?;

    // Build message to sign: package_id:version:publisher:archive_hash
    let message = format!("{}:{}:{}:{}", package_id, version, publisher, archive_hash);
    let signature = key_pair.sign(message.as_bytes());

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    Ok(PackageSignature {
        package_id: package_id.into(),
        version: version.into(),
        publisher: publisher.into(),
        public_key: base64_encode(key_pair.public_key().as_ref()),
        signature: base64_encode(signature.as_ref()),
        signed_at: now.to_string(),
        archive_sha256: archive_hash.into(),
    })
}

/// Verify a package signature against the public key and original data.
pub fn verify_signature(sig: &PackageSignature, data: &[u8]) -> Result<bool, String> {
    let public_key_bytes = base64_decode(&sig.public_key)
        .map_err(|e| format!("Invalid public key: {e}"))?;
    let signature_bytes = base64_decode(&sig.signature)
        .map_err(|e| format!("Invalid signature: {e}"))?;

    let public_key = UnparsedPublicKey::new(&ED25519, &public_key_bytes);
    match public_key.verify(data, &signature_bytes) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Fallback: generate a key using rand entropy (non-Ed25519, for environments without ring).
pub fn generate_keypair_fallback() -> PublisherKeyPair {
    let mut rng = rand::thread_rng();
    let mut pk = [0u8; 32];
    let mut sk = [0u8; 32];
    rng.fill(&mut pk);
    rng.fill(&mut sk);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    PublisherKeyPair {
        public_key: format!("pk-{}", hex_encode(&pk[..8])),
        secret_key: format!("sk-{}", hex_encode(&sk[..8])),
        created_at: now.to_string(),
    }
}

fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}

fn base64_decode(data: &str) -> Result<Vec<u8>, String> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|e| format!("base64 decode: {e}"))
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_ed25519_keypair() {
        let kp = generate_keypair();
        assert!(!kp.public_key.is_empty());
        assert!(!kp.secret_key.is_empty());
        assert_ne!(kp.public_key, kp.secret_key);
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let kp = generate_keypair();
        let sig = sign_package(
            "test-pkg", "1.0.0", "test-author",
            &kp.secret_key, "abc123"
        ).expect("signing failed");
        
        let message = format!("{}:{}:{}:{}", sig.package_id, sig.version, sig.publisher, sig.archive_sha256);
        let valid = verify_signature(&sig, message.as_bytes()).expect("verification failed");
        assert!(valid, "Signature verification failed for roundtrip");
    }

    #[test]
    fn tampered_signature_fails() {
        let kp = generate_keypair();
        let mut sig = sign_package(
            "test-pkg", "1.0.0", "test-author",
            &kp.secret_key, "abc123"
        ).expect("signing failed");
        
        // Tamper with the version
        sig.version = "2.0.0".into();
        let message = format!("{}:{}:{}:{}", sig.package_id, sig.version, sig.publisher, sig.archive_sha256);
        let valid = verify_signature(&sig, message.as_bytes()).expect("verification failed");
        assert!(!valid, "Tampered signature should fail verification");
    }

    #[test]
    fn fallback_keygen_works() {
        let kp = generate_keypair_fallback();
        assert!(kp.public_key.starts_with("pk-"));
        assert!(kp.secret_key.starts_with("sk-"));
    }
}
