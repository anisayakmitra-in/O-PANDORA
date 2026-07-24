//! SQLite session storage — persistent session database.
//!
//! Replaces JSON file-based session storage with a proper database.
//! Sessions are stored with full metadata, timeline, and replay data.

use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct SqliteSessionStore {
    conn: Mutex<Connection>,
}

impl SqliteSessionStore {
    pub fn new(db_path: PathBuf) -> Result<Self, String> {
        let conn =
            Connection::open(&db_path).map_err(|e| format!("Cannot open session DB: {e}"))?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS sessions (
                id TEXT PRIMARY KEY,
                task TEXT NOT NULL,
                status TEXT NOT NULL,
                domain TEXT,
                provider TEXT,
                model TEXT,
                workflow TEXT,
                replay_id TEXT,
                created_at TEXT NOT NULL,
                completed_at TEXT,
                metadata_json TEXT
            );

            CREATE TABLE IF NOT EXISTS execution_frames (
                frame_id TEXT PRIMARY KEY,
                session_id TEXT NOT NULL,
                step_kind TEXT,
                step_label TEXT,
                provider TEXT,
                model TEXT,
                duration_ms INTEGER,
                tokens_used INTEGER,
                success INTEGER,
                timestamp TEXT NOT NULL,
                FOREIGN KEY (session_id) REFERENCES sessions(id)
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_domain ON sessions(domain);
            CREATE INDEX IF NOT EXISTS idx_frames_session ON execution_frames(session_id);
            ",
        )
        .map_err(|e| format!("Cannot create tables: {e}"))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn create_session(&self, id: &str, task: &str, domain: &str) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        conn.execute(
            "INSERT OR REPLACE INTO sessions (id, task, status, domain, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, task, "running", domain, chrono::Utc::now().to_rfc3339()],
        ).map_err(|e| format!("Insert session: {e}"))?;
        Ok(())
    }

    pub fn complete_session(
        &self,
        id: &str,
        status: &str,
        provider: &str,
        model: &str,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        conn.execute(
            "UPDATE sessions SET status = ?1, provider = ?2, model = ?3, completed_at = ?4 WHERE id = ?5",
            params![status, provider, model, chrono::Utc::now().to_rfc3339(), id],
        ).map_err(|e| format!("Update session: {e}"))?;
        Ok(())
    }

    pub fn list_sessions(&self, limit: usize) -> Result<Vec<SessionRecord>, String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, task, status, domain, provider, model, created_at, completed_at FROM sessions ORDER BY created_at DESC LIMIT ?1"
        ).map_err(|e| format!("Prepare: {e}"))?;
        let rows = stmt
            .query_map(params![limit as i64], |row| {
                Ok(SessionRecord {
                    id: row.get(0)?,
                    task: row.get(1)?,
                    status: row.get(2)?,
                    domain: row.get(3)?,
                    provider: row.get(4).unwrap_or_default(),
                    model: row.get(5).unwrap_or_default(),
                    created_at: row.get(6)?,
                    completed_at: row.get(7).unwrap_or_default(),
                })
            })
            .map_err(|e| format!("Query: {e}"))?;
        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(row.map_err(|e| format!("Row: {e}"))?);
        }
        Ok(sessions)
    }

    pub fn get_session(&self, id: &str) -> Result<Option<SessionRecord>, String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, task, status, domain, provider, model, created_at, completed_at FROM sessions WHERE id = ?1"
        ).map_err(|e| format!("Prepare: {e}"))?;
        let mut rows = stmt.query(params![id]).map_err(|e| format!("Query: {e}"))?;
        if let Ok(Some(row)) = rows.next() {
            return Ok(Some(SessionRecord {
                id: row.get(0).unwrap_or_default(),
                task: row.get(1).unwrap_or_default(),
                status: row.get(2).unwrap_or_default(),
                domain: row.get(3).unwrap_or_default(),
                provider: row.get(4).unwrap_or_default(),
                model: row.get(5).unwrap_or_default(),
                created_at: row.get(6).unwrap_or_default(),
                completed_at: row.get(7).unwrap_or_default(),
            }));
        }
        Ok(None)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn record_frame(
        &self,
        frame_id: &str,
        session_id: &str,
        step_kind: &str,
        step_label: &str,
        provider: &str,
        model: &str,
        duration_ms: u64,
        tokens: usize,
        success: bool,
    ) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        conn.execute(
            "INSERT OR REPLACE INTO execution_frames (frame_id, session_id, step_kind, step_label, provider, model, duration_ms, tokens_used, success, timestamp) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![frame_id, session_id, step_kind, step_label, provider, model, duration_ms as i64, tokens as i64, success as i32, chrono::Utc::now().to_rfc3339()],
        ).map_err(|e| format!("Insert frame: {e}"))?;
        Ok(())
    }

    pub fn session_count(&self) -> Result<usize, String> {
        let conn = self.conn.lock().map_err(|e| format!("DB lock: {e}"))?;
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))
            .map_err(|e| format!("Count: {e}"))?;
        Ok(count as usize)
    }
}

#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub id: String,
    pub task: String,
    pub status: String,
    pub domain: String,
    pub provider: String,
    pub model: String,
    pub created_at: String,
    pub completed_at: String,
}
