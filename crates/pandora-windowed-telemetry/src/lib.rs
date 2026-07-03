//! Pandora Windowed Telemetry — extracted from pandora-runtime (Phase 1A).
//!
pub struct WindowedTelemetry;

impl WindowedTelemetry {
    pub fn moving_average(values: &[f32]) -> f32 {
        if values.is_empty() {
            return 0.0;
        }

        values.iter().sum::<f32>() / values.len() as f32
    }
}
