//! Pandora Telemetry — subsystem crate.
//!
//! Phase 1C consolidation: absorbs pandora-telemetry, pandora-recorder,
//! pandora-replay, pandora-tracing micro-crates into a single subsystem.

pub mod entropy;

pub use entropy::{EntropyEngine, ToolCall};

// Future modules (absorbed in subsequent commits):
// pub mod recorder;
// pub mod replay;
// pub mod tracing;
// pub mod metrics;
// pub mod exporter;
