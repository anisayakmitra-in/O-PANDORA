//! Pandora Swarm Identity — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityTrait {
    pub trait_name: String,

    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityState {
    pub dominant_identity: String,

    pub coherence: f32,

    pub adaptability: f32,

    pub continuity: f32,
}

pub struct SwarmIdentity;

impl SwarmIdentity {
    pub fn synthesize(traits: &[IdentityTrait]) -> IdentityState {
        let mut dominant = "neutral".to_string();

        let mut strongest = 0.0;

        let mut continuity = 0.0;

        for identity in traits {
            println!(
                "[IDENTITY] {} strength={}",
                identity.trait_name, identity.strength
            );

            continuity += identity.strength;

            if identity.strength > strongest {
                strongest = identity.strength;

                dominant = identity.trait_name.clone();
            }
        }

        let coherence = strongest;

        let adaptability = 1.0 - (strongest * 0.3);

        IdentityState {
            dominant_identity: dominant,

            coherence,

            adaptability,

            continuity,
        }
    }
}
