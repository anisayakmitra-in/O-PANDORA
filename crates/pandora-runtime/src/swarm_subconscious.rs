use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousImprint {
    pub origin: String,

    pub pattern: String,

    pub influence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubconsciousState {
    pub dominant_pattern: String,

    pub behavioral_pressure: f32,
}

pub struct SwarmSubconscious;

impl SwarmSubconscious {
    pub fn integrate(imprints: &[SubconsciousImprint]) -> SubconsciousState {
        let mut strongest = "neutral".to_string();

        let mut pressure = 0.0;

        for imprint in imprints {
            println!(
                "[SUBCONSCIOUS] {} -> {} ({})",
                imprint.origin, imprint.pattern, imprint.influence
            );

            if imprint.influence > pressure {
                pressure = imprint.influence;

                strongest = imprint.pattern.clone();
            }
        }

        SubconsciousState {
            dominant_pattern: strongest,

            behavioral_pressure: pressure,
        }
    }
}
