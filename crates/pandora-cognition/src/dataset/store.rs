use std::sync::Arc;

use tokio::sync::RwLock;

use crate::dataset::CognitionDatasetEntry;

#[derive(Clone)]
pub struct DatasetStore {

    entries:
        Arc<
            RwLock<
                Vec<CognitionDatasetEntry>
            >
        >,
}

impl DatasetStore {

    pub fn new() -> Self {

        Self {

            entries:
                Arc::new(
                    RwLock::new(
                        Vec::new()
                    )
                ),
        }
    }

    pub async fn insert(

        &self,

        entry:
            CognitionDatasetEntry,

    ) {

        let mut entries =
            self.entries
                .write()
                .await;

        entries.push(
            entry
        );
    }

    pub async fn all(
        &self,
    ) -> Vec<CognitionDatasetEntry> {

        self.entries
            .read()
            .await
            .clone()
    }

    pub async fn successful(
        &self,
    ) -> Vec<CognitionDatasetEntry> {

        self.entries
            .read()
            .await
            .iter()
            .filter(
                |e| {
                    e.telemetry.success
                }
            )
            .cloned()
            .collect()
    }

    pub async fn failed(
        &self,
    ) -> Vec<CognitionDatasetEntry> {

        self.entries
            .read()
            .await
            .iter()
            .filter(
                |e| {
                    !e.telemetry.success
                }
            )
            .cloned()
            .collect()
    }
}
