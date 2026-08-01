//! Client for K-O-Palace-compatible registry APIs and verified artifacts.

use flate2::read::GzDecoder;
use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;
use tar::Archive;

const MAX_ARTIFACT_BYTES: u64 = 100 * 1024 * 1024;
const MAX_ARCHIVE_ENTRIES: usize = 10_000;
const MAX_EXTRACTED_BYTES: u64 = 500 * 1024 * 1024;

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
        if !uses_allowed_transport(&base_url) {
            return Err("Registry URL must use HTTPS or loopback HTTP".into());
        }
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::none())
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
        if !uses_allowed_transport(artifact_url) {
            return Err("Artifact URL must use HTTPS or loopback HTTP".into());
        }
        let content_hash = package
            .trust
            .content_hash
            .as_deref()
            .ok_or_else(|| "Package has no content hash; refusing installation".to_string())?;
        let expected_hash = canonical_checksum(content_hash)?;
        let response = self.artifact_request(artifact_url)?;
        let bytes = read_limited(response)?;
        crate::checksum::verify_checksum_bytes(&bytes, &expected_hash)
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
                public_key: encoded_field(public_key),
                signature: encoded_field(signature),
                signed_at: String::new(),
                archive_sha256: content_hash.to_string(),
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
        extract_archive_transactionally(&bytes, destination)
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

    fn request(&self, request: reqwest::blocking::RequestBuilder) -> Result<Response, String> {
        self.send(request, true, "Registry")
    }

    fn artifact_request(&self, artifact_url: &str) -> Result<Response, String> {
        let url = reqwest::Url::parse(artifact_url)
            .map_err(|error| format!("Invalid artifact URL: {error}"))?;
        let include_token = is_registry_origin(&self.base_url, &url);
        self.send(self.client.get(url), include_token, "Artifact")
    }

    fn send(
        &self,
        mut request: reqwest::blocking::RequestBuilder,
        include_token: bool,
        target: &str,
    ) -> Result<Response, String> {
        if include_token {
            if let Some(token) = &self.token {
                request = request.bearer_auth(token);
            }
        }
        let response = request
            .send()
            .map_err(|error| format!("{target} request failed: {error}"))?;
        let status = response.status();
        if !status.is_success() {
            return Err(format!("{target} returned HTTP {status}"));
        }
        Ok(response)
    }
}

fn uses_allowed_transport(value: &str) -> bool {
    let Ok(url) = reqwest::Url::parse(value) else {
        return false;
    };
    if !url.username().is_empty() || url.password().is_some() {
        return false;
    }
    let Some(host) = url.host_str() else {
        return false;
    };
    match url.scheme() {
        "https" => true,
        "http" => {
            let loopback_host = host.trim_start_matches('[').trim_end_matches(']');
            host.eq_ignore_ascii_case("localhost")
                || loopback_host
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|address| address.is_loopback())
        }
        _ => false,
    }
}

fn is_registry_origin(base_url: &str, artifact_url: &reqwest::Url) -> bool {
    let Ok(base_url) = reqwest::Url::parse(base_url) else {
        return false;
    };
    base_url.scheme() == artifact_url.scheme()
        && base_url.host_str() == artifact_url.host_str()
        && base_url.port_or_known_default() == artifact_url.port_or_known_default()
}

fn canonical_checksum(value: &str) -> Result<String, String> {
    let digest = value.strip_prefix("sha256:").unwrap_or(value);
    if digest.len() != 64
        || !digest
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err("Package content hash must be a 64-character SHA-256 digest".into());
    }
    Ok(format!("sha256:{}", digest.to_ascii_lowercase()))
}

fn encoded_field(value: &str) -> String {
    value.strip_prefix("base64:").unwrap_or(value).to_string()
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

fn extract_archive_transactionally(bytes: &[u8], destination: &Path) -> Result<PathBuf, String> {
    match std::fs::symlink_metadata(destination) {
        Ok(_) => {
            return Err(format!(
                "Staging destination already exists: {}",
                destination.display()
            ));
        }
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
            return Err(format!(
                "Could not inspect staging destination {}: {error}",
                destination.display()
            ));
        }
        Err(_) => {}
    }

    let parent = destination
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("Could not create staging parent: {error}"))?;
    let name = destination
        .file_name()
        .ok_or_else(|| "Staging destination must have a file name".to_string())?
        .to_string_lossy();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| format!("System clock error: {error}"))?
        .as_nanos();
    let mut staging = None;
    for attempt in 0..10 {
        let candidate = parent.join(format!(
            ".{name}-download-{}-{timestamp}-{attempt}",
            std::process::id()
        ));
        match std::fs::create_dir(&candidate) {
            Ok(()) => {
                staging = Some(candidate);
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => {
                return Err(format!(
                    "Could not create extraction staging directory: {error}"
                ))
            }
        }
    }
    let staging =
        staging.ok_or_else(|| "Could not allocate extraction staging directory".to_string())?;

    let result = (|| {
        extract_archive(bytes, &staging)?;
        let root = find_package_root(&staging)?;
        let relative_root = root
            .strip_prefix(&staging)
            .map_err(|_| "Extraction root escaped staging directory".to_string())?;
        std::fs::rename(&staging, destination)
            .map_err(|error| format!("Could not commit extracted package: {error}"))?;
        Ok(destination.join(relative_root))
    })();
    if result.is_err() {
        let _ = std::fs::remove_dir_all(&staging);
    }
    result
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
    let mut entry_count = 0;
    let mut extracted_bytes: u64 = 0;
    for entry in entries {
        entry_count += 1;
        if entry_count > MAX_ARCHIVE_ENTRIES {
            return Err(format!(
                "Package contains more than {MAX_ARCHIVE_ENTRIES} entries"
            ));
        }
        let mut entry = entry.map_err(|e| format!("Invalid package entry: {e}"))?;
        let path = entry
            .path()
            .map_err(|e| format!("Invalid package path: {e}"))?;
        if !is_safe_path(&path) {
            return Err(format!("Unsafe package path: {}", path.display()));
        }
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err(format!("Package links are not allowed: {}", path.display()));
        }
        let entry_size = entry
            .header()
            .size()
            .map_err(|e| format!("Invalid package size: {e}"))?;
        extracted_bytes = extracted_bytes
            .checked_add(entry_size)
            .ok_or_else(|| "Package extracted size overflow".to_string())?;
        if extracted_bytes > MAX_EXTRACTED_BYTES {
            return Err(format!(
                "Package expands beyond {MAX_EXTRACTED_BYTES} bytes"
            ));
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
    fn validates_registry_transport() {
        assert!(RegistryClient::new("localhost:3001", None).is_err());
        assert!(RegistryClient::new("http://registry.example", None).is_err());
        assert!(RegistryClient::new("http://localhost:3001", None).is_ok());
        assert!(RegistryClient::new("https://registry.example", None).is_ok());
    }

    #[test]
    fn accepts_only_secure_or_loopback_transport() {
        assert!(uses_allowed_transport(
            "https://registry.example/package.tar.gz"
        ));
        assert!(uses_allowed_transport(
            "http://127.0.0.1:3001/package.tar.gz"
        ));
        assert!(uses_allowed_transport("http://[::1]:3001/package.tar.gz"));
        assert!(!uses_allowed_transport(
            "http://registry.example/package.tar.gz"
        ));
        assert!(!uses_allowed_transport(
            "http://localhost.example/package.tar.gz"
        ));
        assert!(!uses_allowed_transport(
            "http://localhost:3001@registry.example/package.tar.gz"
        ));
    }

    #[test]
    fn scopes_artifact_token_to_registry_origin() {
        let registry = "https://registry.example";
        assert!(is_registry_origin(
            registry,
            &reqwest::Url::parse("https://registry.example/artifact").expect("same origin")
        ));
        assert!(!is_registry_origin(
            registry,
            &reqwest::Url::parse("https://registry.example:8443/artifact").expect("different port")
        ));
        assert!(!is_registry_origin(
            registry,
            &reqwest::Url::parse("https://artifact.example/artifact").expect("different host")
        ));
    }

    #[test]
    fn normalizes_checksum_and_encoded_fields() {
        let digest = "A".repeat(64);
        assert_eq!(
            canonical_checksum(&digest).expect("bare digest"),
            format!("sha256:{}", "a".repeat(64))
        );
        assert_eq!(
            canonical_checksum(&format!("sha256:{digest}")).expect("prefixed digest"),
            format!("sha256:{}", "a".repeat(64))
        );
        assert_eq!(encoded_field("base64:YWJj"), "YWJj");
        assert_eq!(encoded_field("YWJj"), "YWJj");
    }

    #[test]
    fn rejects_unsafe_archive_paths() {
        assert!(!is_safe_path(Path::new("../escape.txt")));
        assert!(!is_safe_path(Path::new("/absolute.txt")));
        assert!(is_safe_path(Path::new("package/gene.toml")));
    }

    #[test]
    fn rejects_archive_links() {
        let mut bytes = Vec::new();
        {
            let mut builder = tar::Builder::new(&mut bytes);
            let mut header = tar::Header::new_gnu();
            builder
                .append_link(&mut header, "outside", "package/link")
                .expect("create link archive");
            builder.finish().expect("finish link archive");
        }
        let destination =
            std::env::temp_dir().join(format!("pandora-registry-link-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&destination);
        let mut archive = Archive::new(std::io::Cursor::new(bytes));
        let result = unpack_entries(&mut archive, &destination);
        let _ = std::fs::remove_dir_all(&destination);
        assert!(result.is_err());
    }

    #[test]
    fn failed_extraction_leaves_destination_untouched() {
        let mut bytes = Vec::new();
        {
            let mut archive = tar::Builder::new(&mut bytes);
            let mut header = tar::Header::new_gnu();
            header.set_size(4);
            header.set_mode(0o644);
            header.set_cksum();
            archive
                .append_data(&mut header, "package/readme.txt", &b"safe"[..])
                .expect("create archive entry");
            archive.finish().expect("finish archive");
        }

        let destination = std::env::temp_dir().join(format!(
            "pandora-registry-transaction-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&destination);
        let result = extract_archive_transactionally(&bytes, &destination);
        let destination_exists = destination.exists();
        let _ = std::fs::remove_dir_all(&destination);

        assert!(result.is_err());
        assert!(!destination_exists);
    }
    #[test]
    fn successful_extraction_commits_staging_directory() {
        let mut bytes = Vec::new();
        {
            let mut archive = tar::Builder::new(&mut bytes);
            let mut header = tar::Header::new_gnu();
            header.set_size(4);
            header.set_mode(0o644);
            header.set_cksum();
            archive
                .append_data(&mut header, "package/gene.toml", &b"safe"[..])
                .expect("create package archive");
            archive.finish().expect("finish archive");
        }

        let destination = std::env::temp_dir().join(format!(
            "pandora-registry-commit-test-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock")
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&destination);
        let root = extract_archive_transactionally(&bytes, &destination).expect("extract archive");
        let installed = root.join("gene.toml").is_file();
        let _ = std::fs::remove_dir_all(&destination);

        assert_eq!(root, destination.join("package"));
        assert!(installed);
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
