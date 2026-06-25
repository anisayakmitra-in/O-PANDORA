use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HormoneSignal {
    pub hormone: String,

    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndocrineState {
    pub aggression: f32,

    pub stability: f32,

    pub expansion: f32,
}

pub struct SwarmEndocrineSystem;

impl SwarmEndocrineSystem {
    pub fn regulate(signals: &[HormoneSignal]) -> EndocrineState {
        let mut state = EndocrineState {
            aggression: 0.5,

            stability: 0.5,

            expansion: 0.5,
        };

        for signal in signals {
            println!(
                "[ENDOCRINE] hormone={} intensity={}",
                signal.hormone, signal.intensity
            );

            match signal.hormone.as_str() {
                "stress" => {
                    state.aggression += signal.intensity;

                    state.stability -= signal.intensity * 0.4;
                }

                "growth" => {
                    state.expansion += signal.intensity;
                }

                "recovery" => {
                    state.stability += signal.intensity;
                }

                _ => {}
            }
        }

        state
    }
}
