//! Checksum Verification — SHA-256 checksums for package integrity.
//!
//! Every package archive must have a valid SHA-256 checksum before install.
//! This prevents tampered or corrupted packages from being installed.

use sha2::{Digest, Sha256};
use std::path::Path;

/// Compute SHA-256 checksum of a file.
pub fn compute_checksum(path: &Path) -> Result<String, pandora_types::PandoraError> {
    let data = std::fs::read(path)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot read file: {e}")))?;
    Ok(compute_checksum_bytes(&data))
}

/// Compute SHA-256 checksum of raw bytes.
pub fn compute_checksum_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("sha256:{}", hex_encode(&result))
}

/// Verify a checksum matches expected value.
pub fn verify_checksum(path: &Path, expected: &str) -> Result<(), pandora_types::PandoraError> {
    let actual = compute_checksum(path)?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("Checksum mismatch: expected {expected}, got {actual}").into())
    }
}

/// Verify checksum of raw bytes.
pub fn verify_checksum_bytes(
    data: &[u8],
    expected: &str,
) -> Result<(), pandora_types::PandoraError> {
    let actual = compute_checksum_bytes(data);
    if actual == expected {
        Ok(())
    } else {
        Err(format!("Checksum mismatch: expected {expected}, got {actual}").into())
    }
}

fn hex_encode(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn compute_and_verify() {
        let dir = std::env::temp_dir().join(format!(
            "cksum-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"hello pandora").unwrap();
        drop(f);

        let checksum = compute_checksum(&path).unwrap();
        assert!(checksum.starts_with("sha256:"));
        assert_eq!(checksum.len(), 71); // "sha256:" + 64 hex chars

        assert!(verify_checksum(&path, &checksum).is_ok());
        assert!(verify_checksum(
            &path,
            "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        )
        .is_err());

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn bytes_checksum() {
        let data = b"test data";
        let c1 = compute_checksum_bytes(data);
        let c2 = compute_checksum_bytes(data);
        assert_eq!(c1, c2);
        assert!(c1.starts_with("sha256:"));
    }

    #[test]
    fn verify_bytes_ok() {
        let data = b"hello";
        let checksum = compute_checksum_bytes(data);
        assert!(verify_checksum_bytes(data, &checksum).is_ok());
    }

    #[test]
    fn verify_bytes_fail() {
        let data = b"hello";
        assert!(verify_checksum_bytes(data, "sha256:wrong").is_err());
    }
}
