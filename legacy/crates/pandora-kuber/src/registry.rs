//! Client for KUBER-compatible registry APIs and verified artifacts.

use flate2::read::GzDecoder;
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tar::Archive;

const MAX_ARTIFACT_BYTES: u64 = 100 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct RegistryClient {
    base_url: String,
    client: Client,
    token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPackage {
    pub id: String,
    pub name: String,
    pub version: String,
    pub kind: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub trust: RegistryTrust,
    pub compatibility: RegistryCompatibility,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default)]
    pub artifact_url: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryTrust {
    pub level: String,
    #[serde(default)]
    pub signature: Option<String>,
    #[serde(default)]
    pub public_key: Option<String>,
    #[serde(default)]
    pub content_hash: Option<String>,
    pub publisher: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RegistryCompatibility {
    #[serde(default)]
    pub runtimes: Vec<String>,
    #[serde(default)]
    pub platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PackageListResponse {
    packages: Vec<RegistryPackage>,
}

impl RegistryClient {
    pub fn new(base_url: &str, token: Option<String>) -> Result<Self, String> {
        let base_url = base_url.trim_end_matches('/').to_string();
        if !(base_url.starts_with("http://") || base_url.starts_with("https://")) {
            return Err("Registry URL must use http:// or https://".into());
        }
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .user_agent(format!("pandora/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .map_err(|e| format!("Could not create registry client: {e}"))?;
        Ok(Self {
            base_url,
            client,
            token,
        })
    }

    pub fn get_package(&self, id: &str) -> Result<RegistryPackage, String> {
        let url = format!("{}/api/v1/packages/{}", self.base_url, encode_component(id));
        self.request(self.client.get(url)).and_then(|response| {
            response
                .json()
                .map_err(|e| format!("Invalid registry package response: {e}"))
        })
    }

    pub fn artifact_url<'a>(&self, package: &'a RegistryPackage) -> Result<&'a str, String> {
        package
            .artifact_url
            .as_deref()
            .ok_or_else(|| format!("Package '{}' has no published artifact", package.id))
    }

    pub fn download_and_extract(
        &self,
        package: &RegistryPackage,
        destination: &Path,
    ) -> Result<PathBuf, String> {
        let artifact_url = self.artifact_url(package)?;
        if !(artifact_url.starts_with("http://") || artifact_url.starts_with("https://")) {
            return Err("Artifact URL must use http:// or https://".into());
        }
        let expected_hash = package
            .trust
            .content_hash
            .as_deref()
            .ok_or_else(|| "Package has no content hash; refusing installation".to_string())?;
        let response = self.request(self.client.get(artifact_url.to_string()))?;
        let bytes = read_limited(response)?;
        crate::checksum::verify_checksum_bytes(&bytes, expected_hash)
            .map_err(|e| format!("Artifact verification failed: {e}"))?;
        if let Some(signature) = &package.trust.signature {
            let public_key = package
                .trust
                .public_key
                .as_deref()
                .ok_or_else(|| "Signed package has no publisher public key".to_string())?;
            let signature = pandora_types::signing::PackageSignature {
                package_id: package.id.clone(),
                version: package.version.clone(),
                publisher: package.trust.publisher.clone(),
                public_key: public_key.to_string(),
                signature: signature.clone(),
                signed_at: String::new(),
                archive_sha256: expected_hash.to_string(),
            };
            let message = format!(
                "{}:{}:{}:{}",
                signature.package_id,
                signature.version,
                signature.publisher,
                signature.archive_sha256
            );
            let valid = pandora_types::signing::verify_signature(&signature, message.as_bytes())
                .map_err(|e| format!("Signature verification failed: {e}"))?;
            if !valid {
                return Err("Package signature verification failed".into());
            }
        }
        extract_archive(&bytes, destination)?;
        find_package_root(destination)
    }

    pub fn search(&self, query: &str) -> Result<Vec<RegistryPackage>, String> {
        let url = format!(
            "{}/api/v1/search?q={}",
            self.base_url,
            encode_component(query)
        );
        self.request(self.client.get(url)).and_then(|response| {
            response
                .json::<PackageListResponse>()
                .map(|result| result.packages)
                .map_err(|e| format!("Invalid registry search response: {e}"))
        })
    }

    fn request(&self, mut request: reqwest::blocking::RequestBuilder) -> Result<Response, String> {
        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }
        let response = request
            .send()
            .map_err(|e| format!("Registry request failed: {e}"))?;
        let status = response.status();
        if !status.is_success() {
            return Err(format!("Registry returned HTTP {status}"));
        }
        Ok(response)
    }
}

fn read_limited(response: Response) -> Result<Vec<u8>, String> {
    if response
        .content_length()
        .is_some_and(|size| size > MAX_ARTIFACT_BYTES)
    {
        return Err(format!("Artifact exceeds {MAX_ARTIFACT_BYTES} byte limit"));
    }
    let bytes = response
        .bytes()
        .map_err(|e| format!("Could not read artifact: {e}"))?;
    if bytes.len() as u64 > MAX_ARTIFACT_BYTES {
        return Err(format!("Artifact exceeds {MAX_ARTIFACT_BYTES} byte limit"));
    }
    Ok(bytes.to_vec())
}

fn extract_archive(bytes: &[u8], destination: &Path) -> Result<(), String> {
    std::fs::create_dir_all(destination)
        .map_err(|e| format!("Could not create staging directory: {e}"))?;
    if bytes.starts_with(&[0x1f, 0x8b]) {
        let decoder = GzDecoder::new(bytes);
        let mut archive = Archive::new(decoder);
        unpack_entries(&mut archive, destination)
    } else {
        let mut archive = Archive::new(bytes);
        unpack_entries(&mut archive, destination)
    }
}

fn unpack_entries<R: std::io::Read>(
    archive: &mut Archive<R>,
    destination: &Path,
) -> Result<(), String> {
    let entries = archive
        .entries()
        .map_err(|e| format!("Invalid package archive: {e}"))?;
    for entry in entries {
        let mut entry = entry.map_err(|e| format!("Invalid package entry: {e}"))?;
        let path = entry
            .path()
            .map_err(|e| format!("Invalid package path: {e}"))?;
        if !is_safe_path(&path) {
            return Err(format!("Unsafe package path: {}", path.display()));
        }
        entry
            .unpack_in(destination)
            .map_err(|e| format!("Could not extract package: {e}"))?;
    }
    Ok(())
}

fn find_package_root(destination: &Path) -> Result<PathBuf, String> {
    if has_manifest(destination) {
        return Ok(destination.to_path_buf());
    }
    let mut roots = std::fs::read_dir(destination)
        .map_err(|e| format!("Could not inspect extracted package: {e}"))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir() && has_manifest(path));
    roots
        .next()
        .ok_or_else(|| "Extracted archive contains no supported package manifest".into())
}

fn has_manifest(path: &Path) -> bool {
    path.join("gene.toml").is_file()
        || path.join("harness.toml").is_file()
        || path.join("SKILL.md").is_file()
        || path.join("provider.toml").is_file()
}

fn is_safe_path(path: &Path) -> bool {
    !path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    })
}
fn encode_component(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_registry_scheme() {
        assert!(RegistryClient::new("localhost:3001", None).is_err());
        assert!(RegistryClient::new("https://registry.example", None).is_ok());
    }

    #[test]
    fn rejects_unsafe_archive_paths() {
        assert!(!is_safe_path(Path::new("../escape.txt")));
        assert!(!is_safe_path(Path::new("/absolute.txt")));
        assert!(is_safe_path(Path::new("package/gene.toml")));
    }

    #[test]
    fn encodes_ids_and_queries() {
        assert_eq!(
            encode_component("owner/package name"),
            "owner%2Fpackage%20name"
        );
        assert_eq!(encode_component("browser.chrome"), "browser.chrome");
    }
}
