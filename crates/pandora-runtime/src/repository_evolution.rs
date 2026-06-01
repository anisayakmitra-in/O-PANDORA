use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryTrait {
    pub trait_name: String,

    pub adaptability: f64,

    pub stability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionMutation {
    pub mutation: String,

    pub projected_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionPlan {
    pub dominant_trait: String,

    pub mutations: Vec<EvolutionMutation>,
}

pub struct RepositoryEvolutionEngine;

impl RepositoryEvolutionEngine {
    pub fn evolve(traits: &[RepositoryTrait]) -> EvolutionPlan {
        let mut dominant = "stability".to_string();

        let mut highest = 0.0;

        let mut mutations = Vec::new();

        for trait_data in traits {
            println!(
                "[EVOLUTION] trait={} adaptability={} stability={}",
                trait_data.trait_name, trait_data.adaptability, trait_data.stability
            );

            let score = (trait_data.adaptability + trait_data.stability) / 2.0;

            if score > highest {
                highest = score;

                dominant = trait_data.trait_name.clone();
            }

            if trait_data.adaptability > 0.80 {
                mutations.push(EvolutionMutation {
                    mutation: format!("expand {} topology", trait_data.trait_name),

                    projected_gain: trait_data.adaptability,
                });
            }

            if trait_data.stability < 0.70 {
                mutations.push(EvolutionMutation {
                    mutation: format!("stabilize {} execution", trait_data.trait_name),

                    projected_gain: 0.78,
                });
            }
        }

        EvolutionPlan {
            dominant_trait: dominant,

            mutations,
        }
    }
}
