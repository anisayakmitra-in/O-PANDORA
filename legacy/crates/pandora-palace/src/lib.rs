//! Pandora Palace — hosted package registry server.
//!
//! Axum-based HTTP API. Run with: `cargo run -p pandora-palace`
//! Endpoints:
//!   GET  /health
//!   POST /api/login
//!   GET  /api/packages
//!   GET  /api/packages/{id}
//!   GET  /api/packages/{id}/versions
//!   POST /api/publish
//!   POST /api/search
//!
//! Storage: in-memory for MVP. Production would use Postgres + S3/R2.

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use pandora_types::package_format::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// In-memory Palace state.
#[derive(Clone)]
pub struct PalaceState {
    pub packages: Arc<RwLock<HashMap<String, RegistryPackage>>>,
    pub users: Arc<RwLock<HashMap<String, PalaceUser>>>,
    pub tokens: Arc<RwLock<HashMap<String, String>>>, // token -> user_id
}

impl PalaceState {
    pub fn new() -> Self { Self { packages: Arc::new(RwLock::new(HashMap::new())), users: Arc::new(RwLock::new(HashMap::new())), tokens: Arc::new(RwLock::new(HashMap::new())) } }
}

// ── Handlers ──

async fn health() -> impl IntoResponse { Json(serde_json::json!({"status": "ok", "service": "pandora-palace", "version": "0.1.0"})) }

async fn list_packages(State(state): State<PalaceState>) -> impl IntoResponse {
    let results: Vec<RegistryPackage> = { state.packages.read().await.values().cloned().collect() };
    Json(results)
}

async fn get_package(State(state): State<PalaceState>, Path(id): Path<String>) -> impl IntoResponse {
    let pkg = { state.packages.read().await.get(&id).cloned() };
    match pkg { Some(p) => Json(p).into_response(), None => (StatusCode::NOT_FOUND, Json(ApiError { code: 404, message: format!("Package not found: {id}") })).into_response() }
}

async fn get_versions(State(state): State<PalaceState>, Path(id): Path<String>) -> impl IntoResponse {
    let pid = id.clone();
    let versions: Vec<String> = state.packages.read().await.values().filter(|p| p.manifest.id == pid).map(|p| p.manifest.version.clone()).collect();
    drop(state);
    Json(versions)
}

async fn search_packages(State(state): State<PalaceState>, Json(query): Json<serde_json::Value>) -> impl IntoResponse {
    let q = query.get("q").and_then(|v| v.as_str()).unwrap_or("").to_lowercase();
    let kind_filter = query.get("kind").and_then(|v| v.as_str()).map(|s| s.to_string());
    let results: Vec<PalaceListing> = { state.packages.read().await.values().filter(|p| {
        let m = &p.manifest;
        (q.is_empty() || m.id.to_lowercase().contains(&q) || m.name.to_lowercase().contains(&q) || m.description.to_lowercase().contains(&q) || m.tags.iter().any(|t| t.to_lowercase().contains(&q)))
        && (kind_filter.as_deref().is_none_or(|k| m.kind.name() == k))
    }).map(|p| PalaceListing {
        full_id: package_id(&p.publisher, &p.manifest.id), name: p.manifest.name.clone(), version: p.manifest.version.clone(),
        kind: p.manifest.kind.clone(), description: p.manifest.description.clone(), publisher: p.publisher.clone(),
        downloads: p.downloads, verified: p.verified, tags: p.manifest.tags.clone(), success_rate: p.manifest.success_rate, ..Default::default()
    }).collect() };
    Json(results)
}

/// Extract user from Authorization: Bearer <token> header.
fn authenticate(state: &PalaceState, headers: &axum::http::HeaderMap) -> Option<PalaceUser> {
    let auth = headers.get("authorization")?.to_str().ok()?;
    let token = auth.strip_prefix("Bearer ")?;
    let tokens = state.tokens.blocking_read();
    let user_id = tokens.get(token)?;
    let users = state.users.blocking_read();
    users.get(user_id).cloned()
}

async fn publish_package(State(state): State<PalaceState>, headers: axum::http::HeaderMap, Json(req): Json<PublishRequest>) -> impl IntoResponse {
    let user = match authenticate(&state, &headers) {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, Json(ApiError { code: 401, message: "Authentication required. Use: pandora login".into() })).into_response(),
    };
    if !req.manifest.publisher.is_empty() && req.manifest.publisher != user.username {
        return (StatusCode::FORBIDDEN, Json(ApiError { code: 403, message: format!("Publisher {} does not match authenticated user {}", req.manifest.publisher, user.username) })).into_response();
    }
    let publisher_name = if req.manifest.publisher.is_empty() { user.username.clone() } else { req.manifest.publisher.clone() };
    let id = req.manifest.id.clone(); let id2 = id.clone();
    let version = req.manifest.version.clone();
    // Compute checksum of the archive
    let hash = Sha256::digest(&req.archive_base64);
    let checksum = hex::encode(hash);

    let registry_pkg = RegistryPackage {
        manifest: req.manifest,
        publisher: publisher_name,
        published_at: chrono::Utc::now().to_rfc3339(),
        downloads: 0,
        verified: false,
        signature: req.signature,
        checksum_sha256: checksum,
        archive_url: format!("/blob/{id}-{version}.tar.gz"), ..Default::default()
    };
    let key = format!("{id2}@{version}");
    state.packages.write().await.insert(key, registry_pkg);

    (StatusCode::CREATED, Json(PublishResponse { id: id.clone(), version, url: format!("/packages/{id}") })).into_response()
}

async fn login(State(state): State<PalaceState>, Json(creds): Json<serde_json::Value>) -> impl IntoResponse {
    let username = creds.get("username").and_then(|v| v.as_str()).unwrap_or("");
    // MVP: create user on first login, return token
    let mut users = state.users.write().await;
    let user = users.entry(username.to_string()).or_insert(PalaceUser {
        id: format!("user-{:016x}", rand::random::<u64>()),
        username: username.to_string(), email: format!("{username}@palace.local"),
        joined_at: chrono::Utc::now().to_rfc3339(), tier: AccountTier::Free,
    });
    let token = format!("tok-{:016x}", rand::random::<u64>());
    state.tokens.write().await.insert(token.clone(), user.id.clone());
    Json(AuthToken { token, user_id: user.id.clone(), expires_at: (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339() })
}

// ── Server ──

pub async fn serve(addr: &str) -> Result<(), anyhow::Error> {
    let state = PalaceState::new();

    // Seed some demo packages
    {
        let mut pkgs = state.packages.write().await;
        for (id, name, kind, desc, author) in &[
            ("rust-tool", "Rust Tool", "gene", "Cargo subcommands", "pandora"),
            ("shell", "Shell", "gene", "Execute shell commands", "pandora"),
            ("coding-domain", "Coding", "domain-harness", "Developer workflow", "pandora"),
            ("security-domain", "Security", "domain-harness", "Security analysis", "pandora"),
            ("cargo-test-evaluator", "Cargo Test Evaluator", "evaluator", "Rust test evaluator", "pandora"),
            ("rust-backend-skill", "Rust Backend Skill", "skill", "Complete Rust backend development skill", "pandora"),
        ] {
            pkgs.insert(format!("{id}@0.1.0"), RegistryPackage { 
                manifest: PackageManifest { publisher: "pandora".into(),
                    id: id.to_string(), name: name.to_string(), version: "0.1.0".into(),
                    kind: match *kind { "gene" => PackageKind::Gene, "domain-harness" => PackageKind::DomainHarness, "evaluator" => PackageKind::Evaluator, "skill" => PackageKind::Skill, _ => PackageKind::Bundle },
                    description: desc.to_string(), author: author.to_string(),
                    pandora_version: ">=1.0".into(), tags: vec![kind.to_string()], categories: vec![],
                    dependencies: vec![], genes: vec![], harnesses: vec![], evaluators: vec![],
                    skills: vec![], profiles: vec![], plans: vec![], license: "MIT".into(),
                    repository: String::new(), documentation: String::new(), homepage: String::new(),
                    success_rate: 0.95, ..Default::default()
                },
                publisher: "pandora".into(), published_at: chrono::Utc::now().to_rfc3339(),
                downloads: 100, verified: true, signature: None, checksum_sha256: String::new(), archive_url: String::new(), ..Default::default()
            });
        }
    }

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/packages", get(list_packages))
        .route("/api/packages/{id}", get(get_package))
        .route("/api/packages/{id}/versions", get(get_versions))
        .route("/api/publish", post(publish_package))
        .route("/api/search", post(search_packages))
        .route("/api/login", post(login))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("[PALACE] Listening on {addr}\n  Packages: /api/packages\n  Publish:  POST /api/publish\n  Search:   POST /api/search\n  Login:    POST /api/login");
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // Palace tests require a running server.
    // Run: cargo run -p pandora-palace and test with curl.
}
