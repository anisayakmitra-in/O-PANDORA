//! Pandora Swarm Immunity — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSignal {
    pub subsystem: String,

    pub severity: f32,

    pub anomaly: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneResponse {
    pub action: String,

    pub target: String,
}

pub struct SwarmImmunity;

impl SwarmImmunity {
    pub fn detect(signals: &[ThreatSignal]) -> Vec<ImmuneResponse> {
        let mut responses = Vec::new();

        for signal in signals {
            if signal.severity > 0.80 {
                println!("[IMMUNITY] threat detected in {}", signal.subsystem);

                responses.push(ImmuneResponse {
                    action: "quarantine_subsystem".into(),

                    target: signal.subsystem.clone(),
                });
            }

            if signal.anomaly == "entropy_spike" {
                responses.push(ImmuneResponse {
                    action: "trigger_rollback".into(),

                    target: signal.subsystem.clone(),
                });
            }
        }

        responses
    }
}
