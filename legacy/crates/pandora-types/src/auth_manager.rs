//! Auth Manager — authentication for pandora serve.
//!
//! Bootstrap tokens, API keys, session management, loopback detection.
//! No heavy frameworks — just what's needed to securely expose the API server.
//! Inspired by CodeNomad's AuthManager pattern.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};

/// Generate random bytes as hex string (no hex crate needed).
fn random_hex(len: usize) -> String {
    use std::hash::{Hash, Hasher};
    let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_nanos();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    nanos.hash(&mut hasher);
    std::thread::current().id().hash(&mut hasher);
    format!("{:016x}", hasher.finish())[..len.min(32)].to_string()
}

fn hash(s: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapToken {
    pub token: String,
    pub created_at: SystemTime,
    pub used: bool,
}
impl BootstrapToken {
    pub fn generate() -> Self {
        Self { token: random_hex(32), created_at: SystemTime::now(), used: false }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub client_id: String,
    pub key_hash: String,
    pub name: String,
    pub created_at: SystemTime,
    pub last_used: Option<SystemTime>,
    pub expires_at: Option<SystemTime>,
}
impl ApiKey {
    pub fn is_expired(&self) -> bool {
        self.expires_at.map(|e| SystemTime::now() > e).unwrap_or(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub client_id: String,
    pub created_at: SystemTime,
    pub last_seen: SystemTime,
    pub expires_at: SystemTime,
}
impl Session {
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }
    pub fn touch(&mut self) {
        self.last_seen = SystemTime::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthStore {
    pub bootstrap: Option<BootstrapToken>,
    pub api_keys: HashMap<String, ApiKey>,
    pub active_sessions: HashMap<String, Session>,
    #[serde(skip)]
    pub config_path: Option<PathBuf>,
}

impl AuthStore {
    pub fn load() -> Self {
        let path = Self::store_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            let mut store: Self = serde_json::from_str(&data).unwrap_or_default();
            store.config_path = Some(path);
            store
        } else {
            Self { config_path: Some(path), ..Default::default() }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = self.config_path.as_ref()
            .cloned()
            .unwrap_or_else(Self::store_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        std::fs::write(&path, json).map_err(|e| format!("write: {e}"))
    }

    fn store_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
        PathBuf::from(home).join(".pandora/auth.json")
    }

    pub fn create_bootstrap(&mut self) -> String {
        let bt = BootstrapToken::generate();
        let token = bt.token.clone();
        self.bootstrap = Some(bt);
        let _ = self.save();
        token
    }

    pub fn validate_bootstrap(&mut self, token: &str) -> Option<String> {
        match &self.bootstrap {
            Some(bt) if bt.token == token && !bt.used => {
                self.bootstrap.as_mut().unwrap().used = true;
                let _ = self.save();
                Some("bootstrap-client".into())
            }
            _ => None,
        }
    }

    pub fn create_api_key(&mut self, name: &str) -> String {
        let raw_key = random_hex(48);
        let key_hash = hash(&raw_key);
        let cid = format!("ak-{}", &raw_key[..12]);
        self.api_keys.insert(cid.clone(), ApiKey {
            client_id: cid,
            key_hash,
            name: name.into(),
            created_at: SystemTime::now(),
            last_used: None,
            expires_at: None,
        });
        let _ = self.save();
        raw_key
    }

    pub fn validate_api_key(&mut self, key: &str) -> Option<String> {
        let key_hash = hash(key);
        // Find by hash
        let mut found: Option<String> = None;
        for ak in self.api_keys.values_mut() {
            if ak.key_hash == key_hash && !ak.is_expired() {
                ak.last_used = Some(SystemTime::now());
                found = Some(ak.client_id.clone());
                break;
            }
        }
        if found.is_some() { let _ = self.save(); }
        found
    }

    pub fn create_session(&mut self, client_id: &str) -> String {
        let session_id = random_hex(32);
        let now = SystemTime::now();
        self.active_sessions.insert(session_id.clone(), Session {
            session_id: session_id.clone(),
            client_id: client_id.into(),
            created_at: now,
            last_seen: now,
            expires_at: now + Duration::from_secs(86400),
        });
        let _ = self.save();
        session_id
    }

    /// Validate and refresh a session. Returns a clone.
    pub fn validate_session(&mut self, sid: &str) -> Option<Session> {
        // Two-phase: first check expiry without borrowing issues
        let expired = self.active_sessions.get(sid)
            .map(|s| s.is_expired())
            .unwrap_or(true);
        if expired {
            self.active_sessions.remove(sid);
            let _ = self.save();
            return None;
        }
        // Touch and clone
        if let Some(session) = self.active_sessions.get_mut(sid) {
            session.touch();
            let result = session.clone();
            let _ = self.save();
            return Some(result);
        }
        None
    }

    pub fn purge_expired(&mut self) -> usize {
        let before = self.active_sessions.len();
        self.active_sessions.retain(|_, s| !s.is_expired());
        let _ = self.save();
        before - self.active_sessions.len()
    }
}

pub fn is_loopback(addr: &str) -> bool {
    addr.starts_with("127.") || addr.starts_with("::1") || addr == "localhost"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_lifecycle() {
        let mut store = AuthStore::default();
        let token = store.create_bootstrap();
        assert!(store.validate_bootstrap(&token).is_some());
        assert!(store.validate_bootstrap(&token).is_none()); // already used
    }

    #[test]
    fn api_key_lifecycle() {
        let mut store = AuthStore::default();
        let raw = store.create_api_key("test");
        assert!(raw.len() > 20);
        assert!(store.validate_api_key(&raw).is_some());
        assert!(store.validate_api_key("bad-key").is_none());
    }

    #[test]
    fn session_lifecycle() {
        let mut store = AuthStore::default();
        let sid = store.create_session("c1");
        assert!(store.validate_session(&sid).is_some());
        assert!(store.validate_session("nope").is_none());
    }

    #[test]
    fn loopback_detection() {
        assert!(is_loopback("127.0.0.1"));
        assert!(is_loopback("::1"));
        assert!(!is_loopback("192.168.1.1"));
    }
}
