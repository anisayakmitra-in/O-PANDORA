//! Pandora Health — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,

    Degraded,

    Critical,

    Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub subsystem_id: String,

    pub state: HealthState,

    pub message: String,
}

pub struct HealthMonitor {
    pub reports: HashMap<String, HealthReport>,
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            reports: HashMap::new(),
        }
    }

    pub fn update(&mut self, report: HealthReport) {
        println!("[HEALTH] {} => {:?}", report.subsystem_id, report.state);

        self.reports.insert(report.subsystem_id.clone(), report);
    }

    pub fn critical(&self) -> Vec<&HealthReport> {
        self.reports
            .values()
            .filter(|r| matches!(r.state, HealthState::Critical))
            .collect()
    }
}
