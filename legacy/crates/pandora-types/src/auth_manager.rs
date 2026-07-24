//! Auth Manager — authentication for pandora serve.
//!
//! Bootstrap tokens, API keys, session management, loopback detection.
//! No heavy frameworks — just what's needed to securely expose the API server.
//! Inspired by CodeNomad's AuthManager pattern.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Generate random bytes as hex string (no hex crate needed).
fn random_hex(len: usize) -> String {
    use ring::rand::SecureRandom;
    let rng = ring::rand::SystemRandom::new();
    let mut bytes = vec![0u8; len.div_ceil(2)];
    rng.fill(&mut bytes).expect("OS random generation failed");
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()[..len]
        .to_string()
}

fn hash(s: &str) -> String {
    use ring::digest;
    let digest = digest::digest(&digest::SHA256, s.as_bytes());
    hex::encode(digest.as_ref())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapToken {
    pub token: String,
    pub created_at: SystemTime,
    pub used: bool,
}
impl BootstrapToken {
    pub fn generate() -> Self {
        Self {
            token: random_hex(32),
            created_at: SystemTime::now(),
            used: false,
        }
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
        self.expires_at
            .map(|e| SystemTime::now() > e)
            .unwrap_or(false)
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
            let mut store: Self = match serde_json::from_str(&data) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Warning: auth.json corrupted, resetting: {e}");
                    Self::default()
                }
            };
            store.config_path = Some(path);
            store
        } else {
            Self {
                config_path: Some(path),
                ..Default::default()
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = self
            .config_path
            .as_ref()
            .cloned()
            .unwrap_or_else(Self::store_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("mkdir: {e}"))?;
        }
        let json = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        // Write with 0600 permissions on Unix to prevent other users from reading credentials
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(&path)
                .map_err(|e| format!("write: {e}"))?;
            std::io::Write::write_all(&mut file, json.as_bytes())
                .map_err(|e| format!("write: {e}"))?;
        }
        #[cfg(not(unix))]
        {
            std::fs::write(&path, json).map_err(|e| format!("write: {e}"))?;
        }
        Ok(())
    }

    fn store_path() -> PathBuf {
        // Resolution order: PANDORA_HOME > ~/.pandora (legacy) > XDG
        if let Ok(h) = std::env::var("PANDORA_HOME") {
            if !h.is_empty() {
                return PathBuf::from(h).join("auth.json");
            }
        }
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| {
                // Last resort: use PANDORA_HOME if available
                std::env::var("PANDORA_HOME").unwrap_or_else(|_| "/root/.pandora".into())
            });
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
            Some(bt) if constant_time_eq(&bt.token, token) && !bt.used => {
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
        self.api_keys.insert(
            cid.clone(),
            ApiKey {
                client_id: cid,
                key_hash,
                name: name.into(),
                created_at: SystemTime::now(),
                last_used: None,
                expires_at: None,
            },
        );
        let _ = self.save();
        raw_key
    }

    pub fn validate_api_key(&mut self, key: &str) -> Option<String> {
        let key_hash = hash(key);
        // Find by hash
        let mut found: Option<String> = None;
        for ak in self.api_keys.values_mut() {
            if constant_time_eq(&ak.key_hash, &key_hash) && !ak.is_expired() {
                ak.last_used = Some(SystemTime::now());
                found = Some(ak.client_id.clone());
                break;
            }
        }
        if found.is_some() {
            let _ = self.save();
        }
        found
    }

    pub fn create_session(&mut self, client_id: &str) -> String {
        let session_id = random_hex(32);
        let now = SystemTime::now();
        self.active_sessions.insert(
            session_id.clone(),
            Session {
                session_id: session_id.clone(),
                client_id: client_id.into(),
                created_at: now,
                last_seen: now,
                expires_at: now + Duration::from_secs(86400),
            },
        );
        let _ = self.save();
        session_id
    }

    /// Validate and refresh a session. Returns a clone.
    pub fn validate_session(&mut self, sid: &str) -> Option<Session> {
        // Two-phase: first check expiry without borrowing issues
        let expired = self
            .active_sessions
            .get(sid)
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
    addr.starts_with("127.")
        || addr == "::1"
        || addr == "[::1]"
        || addr == "localhost"
        || addr == "0.0.0.0"
}

/// Constant-time string comparison to prevent timing attacks.
fn constant_time_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let max_len = a_bytes.len().max(b_bytes.len());
    let mut diff: u8 = 0;
    for i in 0..max_len {
        let a_byte = a_bytes.get(i).copied().unwrap_or(0);
        let b_byte = b_bytes.get(i).copied().unwrap_or(0);
        diff |= a_byte ^ b_byte;
        diff |= (a_bytes.len() as u8) ^ (b_bytes.len() as u8);
    }
    diff == 0
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
        assert!(raw.len() >= 16);
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
