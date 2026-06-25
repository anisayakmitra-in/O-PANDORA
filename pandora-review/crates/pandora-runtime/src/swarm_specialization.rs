use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializedAgent {
    pub agent_id: String,

    pub specialization: String,

    pub performance: f32,
}

pub struct SwarmSpecializationEngine;

impl SwarmSpecializationEngine {
    pub fn evolve(agents: &mut Vec<SpecializedAgent>) {
        for agent in agents.iter_mut() {
            if agent.performance > 0.90 {
                agent.specialization = format!("elite-{}", agent.specialization);

                println!(
                    "[SPECIALIZATION] {} evolved into {}",
                    agent.agent_id, agent.specialization
                );
            }
        }
    }
}
