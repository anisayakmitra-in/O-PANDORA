use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::task::{Task, TaskStatus};

#[derive(thiserror::Error, Debug)]
pub enum PersistenceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("State corruption: {0}")]
    Corruption(String),
}

/// The persistence layer acting as an in-memory cache backed by an append-only JSONL log.
pub struct TaskStore {
    file_path: PathBuf,
    cache: HashMap<Uuid, Task>,
    append_handle: Option<File>,
}

impl TaskStore {
    /// Initializes the store but does NOT load or compact. Call `load_and_compact` immediately after.
    pub fn new<P: AsRef<Path>>(directory: P) -> Self {
        let file_path = directory.as_ref().join("tasks.jsonl");
        Self {
            file_path,
            cache: HashMap::new(),
            append_handle: None,
        }
    }

    /// Replays the append-only log, updates the cache, and writes out a clean, compacted state.
    #[instrument(skip(self), fields(path = %self.file_path.display()))]
    pub async fn load_and_compact(&mut self) -> Result<(), PersistenceError> {
        // Ensure the parent directory exists
        if let Some(parent) = self.file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).await?;
                info!(
                    "Created scheduler persistence directory at {}",
                    parent.display()
                );
            }
        }

        // 1. Replay existing log (if it exists)
        if self.file_path.exists() {
            let file = File::open(&self.file_path).await?;
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            let mut line_count = 0;

            while reader.read_line(&mut line).await? > 0 {
                line_count += 1;
                match serde_json::from_str::<Task>(&line) {
                    Ok(task) => {
                        // Upsert: Later lines overwrite earlier lines for the same UUID
                        self.cache.insert(task.id, task);
                    }
                    Err(e) => {
                        warn!(line_number = line_count, error = %e, "Corrupt task entry skipped during replay");
                    }
                }
                line.clear();
            }
            debug!(
                replayed_lines = line_count,
                active_tasks = self.cache.len(),
                "Replayed persistence log"
            );
        }

        // 2. Filter out completed/cancelled tasks that no longer need tracking
        self.cache.retain(|_, task| {
            task.status != TaskStatus::Completed && task.status != TaskStatus::Cancelled
        });

        // 3. Compact: Write the current exact state to a temporary file
        let tmp_path = self.file_path.with_extension("jsonl.tmp");
        let mut tmp_file = File::create(&tmp_path).await?;

        let mut serialized_buffer = Vec::with_capacity(self.cache.len() * 256);
        for task in self.cache.values() {
            let mut json = serde_json::to_vec(task)?;
            json.push(b'\n');
            serialized_buffer.extend_from_slice(&json);
        }

        tmp_file.write_all(&serialized_buffer).await?;
        tmp_file.sync_data().await?; // Guarantee it hits disk before rename

        // 4. Atomic Replace: Prevents corruption if the runtime crashes right here
        fs::rename(&tmp_path, &self.file_path).await?;
        info!(
            "Scheduler storage compacted atomically. Tracking {} active tasks.",
            self.cache.len()
        );

        // 5. Open file in append mode for future runtime mutations
        let append_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await?;

        self.append_handle = Some(append_file);

        Ok(())
    }

    /// Persists a task mutation by appending to the log, then updates the cache.
    #[instrument(skip(self, task), fields(task_id = %task.id, status = ?task.status))]
    pub async fn update_task(&mut self, task: &Task) -> Result<(), PersistenceError> {
        let handle = self.append_handle.as_mut().ok_or_else(|| {
            PersistenceError::Corruption(
                "Attempted to update task before storage was booted/compacted".into(),
            )
        })?;

        // Fast append to JSONL
        let mut line = serde_json::to_vec(task)?;
        line.push(b'\n');

        handle.write_all(&line).await?;
        handle.sync_data().await?; // Fsync ensures survival against sudden power loss

        // Only update in-memory cache after disk write succeeds
        self.cache.insert(task.id, task.clone());

        Ok(())
    }

    /// Retrieves a copy of a specific task.
    pub fn get_task(&self, id: &Uuid) -> Option<&Task> {
        self.cache.get(id)
    }

    /// Retrieves a snapshot of all currently tracked tasks.
    pub fn get_all_tasks(&self) -> &HashMap<Uuid, Task> {
        &self.cache
    }
}
