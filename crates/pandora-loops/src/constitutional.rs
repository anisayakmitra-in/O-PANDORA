use crate::outcome::{LoopOutcome, LoopStatus};
use pandora_types::universal::{EvolutionConfig, Health, Lifecycle};

impl LoopOutcome {
    pub fn health_universal(&self) -> Health {
        match self.status {
            LoopStatus::Completed => Health::Healthy,
            LoopStatus::Escalated => Health::Degraded,
            LoopStatus::Failed => Health::Offline,
            LoopStatus::Skipped => Health::Ready,
        }
    }

    pub fn lifecycle_universal(&self) -> Lifecycle {
        match self.status {
            LoopStatus::Completed => Lifecycle::Ready,
            LoopStatus::Escalated => Lifecycle::Recovering,
            LoopStatus::Failed => Lifecycle::Stopped,
            LoopStatus::Skipped => Lifecycle::Paused,
        }
    }

    pub fn evolution_config(&self) -> EvolutionConfig {
        EvolutionConfig::enabled()
    }
}
