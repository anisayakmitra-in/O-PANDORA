use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamFragment {
    pub source: String,

    pub scenario: String,

    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DreamOutcome {
    pub synthesized_pattern: String,

    pub projected_gain: f32,
}

pub struct SwarmDreamEngine;

impl SwarmDreamEngine {
    pub fn simulate(fragments: &[DreamFragment]) -> Vec<DreamOutcome> {
        let mut outcomes = Vec::new();

        for fragment in fragments {
            println!(
                "[DREAM] simulating {} -> {}",
                fragment.source, fragment.scenario
            );

            let pattern = if fragment.intensity > 0.80 {
                "high-expansion"
            } else {
                "stability-preserving"
            };

            outcomes.push(DreamOutcome {
                synthesized_pattern: pattern.into(),

                projected_gain: fragment.intensity * 1.2,
            });
        }

        outcomes
    }
}
