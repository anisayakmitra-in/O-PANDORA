//! Pandora Telemetry — subsystem crate.
//!
//! Phase 1C consolidation: absorbs pandora-telemetry, pandora-recorder,
//! pandora-replay, pandora-tracing micro-crates into a single subsystem.

pub mod checkpoint;
pub mod loop_detection;
pub mod trace;
pub mod windowed_telemetry;

pub use loop_detection::LoopDetector;

// Future modules (absorbed in subsequent commits):
// pub mod recorder;
// pub mod replay;
// pub mod tracing;
// pub mod metrics;
// pub mod exporter;
