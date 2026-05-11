use std::path::PathBuf;

use tokio::fs::OpenOptions;

use tokio::io::AsyncWriteExt;

use tokio::sync::Mutex;

use async_trait::async_trait;

use std::sync::Arc;

use crate::audit::AuditEvent;

use crate::traits::AuditLogger;

use crate::error::GovernanceError;

pub struct JsonlAuditLogger {

    file:
        Arc<Mutex<tokio::fs::File>>,
}

impl JsonlAuditLogger {

    pub async fn new(
        path: PathBuf,
    ) -> Result<Self, GovernanceError> {

        let file =
            OpenOptions::new()

                .create(true)

                .append(true)

                .open(path)

                .await

                .map_err(|e| {
                    GovernanceError::Violation(
                        e.to_string()
                    )
                })?;

        Ok(

            Self {

                file:
                    Arc::new(
                        Mutex::new(file)
                    ),
            }
        )
    }
}

#[async_trait]
impl AuditLogger
for JsonlAuditLogger {

    async fn log_event(

        &self,

        event:
            AuditEvent,

    ) -> Result<(), GovernanceError> {

        let json =
            serde_json::to_string(
                &event
            )
            .map_err(|e| {
                GovernanceError::Violation(
                    e.to_string()
                )
            })?;

        let mut file =
            self.file.lock().await;

        file.write_all(
            json.as_bytes()
        )
        .await
        .map_err(|e| {
            GovernanceError::Violation(
                e.to_string()
            )
        })?;

        file.write_all(b"\n")
            .await
            .map_err(|e| {
                GovernanceError::Violation(
                    e.to_string()
                )
            })?;

        Ok(())
    }
}

