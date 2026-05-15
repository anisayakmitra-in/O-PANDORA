use std::sync::Arc;

use tokio::sync::RwLock;

use crate::telemetry::CognitionTelemetry;

#[derive(Clone)]
pub struct TelemetryRecorder {

    records:
        Arc<
            RwLock<
                Vec<CognitionTelemetry>
            >
        >,
}

impl TelemetryRecorder {

    pub fn new() -> Self {

        Self {

            records:
                Arc::new(
                    RwLock::new(
                        Vec::new()
                    )
                ),
        }
    }

    pub async fn record(

        &self,

        telemetry:
            CognitionTelemetry,

    ) {

        let mut records =
            self.records
                .write()
                .await;

        records.push(
            telemetry
        );
    }

    pub async fn all(
        &self,
    ) -> Vec<CognitionTelemetry> {

        self.records
            .read()
            .await
            .clone()
    }

    pub async fn failures(
        &self,
    ) -> Vec<CognitionTelemetry> {

        self.records
            .read()
            .await
            .iter()
            .filter(
                |r| !r.success
            )
            .cloned()
            .collect()
    }
}

