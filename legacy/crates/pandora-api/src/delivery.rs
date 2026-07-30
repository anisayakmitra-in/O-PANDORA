use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryStatus {
    Pending,
    Delivered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRecord {
    pub id: String,
    pub channel: String,
    pub session_id: String,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub attempts: u32,
    pub status: DeliveryStatus,
}

#[derive(Debug)]
pub struct DeliveryLedger {
    path: PathBuf,
    records: Mutex<Vec<DeliveryRecord>>,
}

impl DeliveryLedger {
    pub fn new(sessions_dir: &Path) -> Self {
        let path = sessions_dir.join("delivery-ledger.json");
        let records = fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();
        Self {
            path,
            records: Mutex::new(records),
        }
    }

    pub fn enqueue(
        &self,
        channel: &str,
        session_id: &str,
        payload: String,
    ) -> anyhow::Result<String> {
        let id = format!("delivery-{}", rand::random::<u128>());
        let record = DeliveryRecord {
            id: id.clone(),
            channel: channel.to_string(),
            session_id: session_id.to_string(),
            payload,
            created_at: Utc::now(),
            delivered_at: None,
            attempts: 0,
            status: DeliveryStatus::Pending,
        };
        let mut records = self
            .records
            .lock()
            .map_err(|_| anyhow::anyhow!("delivery ledger lock poisoned"))?;
        records.push(record);
        self.persist(&records)?;
        Ok(id)
    }

    pub fn mark_delivered(&self, id: &str) -> anyhow::Result<()> {
        let mut records = self
            .records
            .lock()
            .map_err(|_| anyhow::anyhow!("delivery ledger lock poisoned"))?;
        if let Some(record) = records.iter_mut().find(|record| record.id == id) {
            record.attempts = record.attempts.saturating_add(1);
            record.delivered_at = Some(Utc::now());
            record.status = DeliveryStatus::Delivered;
            self.persist(&records)?;
        }
        Ok(())
    }

    pub fn list(&self) -> anyhow::Result<Vec<DeliveryRecord>> {
        let records = self
            .records
            .lock()
            .map_err(|_| anyhow::anyhow!("delivery ledger lock poisoned"))?;
        Ok(records.clone())
    }

    fn persist(&self, records: &[DeliveryRecord]) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temporary = self.path.with_extension("json.tmp");
        fs::write(&temporary, serde_json::to_vec_pretty(records)?)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&temporary)?.permissions();
            permissions.set_mode(0o600);
            fs::set_permissions(&temporary, permissions)?;
        }
        #[cfg(windows)]
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        fs::rename(temporary, &self.path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persists_and_marks_delivery() {
        let dir = std::env::temp_dir().join(format!("pandora-delivery-{}", rand::random::<u64>()));
        let ledger = DeliveryLedger::new(&dir);
        let id = ledger.enqueue("http", "session", "{}".into()).unwrap();
        let second_id = ledger
            .enqueue("http", "session", "{\"second\":true}".into())
            .unwrap();
        assert_eq!(ledger.list().unwrap().len(), 2);
        assert_eq!(ledger.list().unwrap()[0].status, DeliveryStatus::Pending);
        ledger.mark_delivered(&id).unwrap();
        ledger.mark_delivered(&second_id).unwrap();
        assert_eq!(ledger.list().unwrap()[0].status, DeliveryStatus::Delivered);
        let reloaded = DeliveryLedger::new(&dir);
        assert_eq!(reloaded.list().unwrap()[0].attempts, 1);
        let _ = fs::remove_dir_all(dir);
    }
}
