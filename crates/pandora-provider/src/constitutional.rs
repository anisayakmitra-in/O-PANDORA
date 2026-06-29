use pandora_types::universal::{EvolutionConfig, Health, Lifecycle, Telemetry};

/// Constitutional metadata for the provider system.
pub struct ProviderConstitutional;

impl ProviderConstitutional {
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
