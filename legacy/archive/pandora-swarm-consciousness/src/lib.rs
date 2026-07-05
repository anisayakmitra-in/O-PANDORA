//! Pandora Swarm Consciousness — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessSignal {
    pub subsystem: String,

    pub state: String,

    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    pub awareness: f32,

    pub coherence: f32,

    pub stability: f32,

    pub dominant_state: String,
}

pub struct SwarmConsciousness;

impl SwarmConsciousness {
    pub fn synthesize(signals: &[ConsciousnessSignal]) -> ConsciousnessState {
        let awareness = signals.len() as f32 / 10.0;

        let mut coherence = 1.0;

        let mut dominant = "stable".to_string();

        for signal in signals {
            println!(
                "[CONSCIOUSNESS] {} => {} ({})",
                signal.subsystem, signal.state, signal.confidence
            );

            if signal.confidence < 0.60 {
                coherence -= 0.15;
            }

            if signal.state == "critical" {
                dominant = "critical".into();
            }
        }

        ConsciousnessState {
            awareness,

            coherence,

            stability: coherence * awareness,

            dominant_state: dominant,
        }
    }
}
