use serde::{
    Serialize,
    Deserialize,
};

use crate::reflection::ReflectionResult;

use crate::telemetry::CognitionTelemetry;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CognitionDatasetEntry {

    pub objective:
        String,

    pub output:
        String,

    pub telemetry:
        CognitionTelemetry,

    pub reflection:
        Option<ReflectionResult>,
}

pub mod store;
