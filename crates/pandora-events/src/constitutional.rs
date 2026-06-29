use pandora_types::universal::{EvolutionConfig, Health, Lifecycle, Telemetry};

/// Constitutional metadata for the event system.
pub struct EventConstitutional;

impl EventConstitutional {
    pub fn health() -> Health {
        Health::Healthy
    }
    pub fn lifecycle() -> Lifecycle {
        Lifecycle::Ready
    }
    pub fn evolution_config() -> EvolutionConfig {
        EvolutionConfig::enabled()
    }
    pub fn telemetry() -> Telemetry {
        Telemetry::default()
    }
}
