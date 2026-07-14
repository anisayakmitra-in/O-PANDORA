use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    pub provider_id: String,

    pub capabilities: Vec<String>,

    pub latency: f32,

    pub reliability: f32,
}

pub struct ProviderArbitrator;

impl ProviderArbitrator {
    pub fn select(required: &str, providers: &[ProviderCapability]) -> Option<ProviderCapability> {
        let mut candidates = providers
            .iter()
            .filter(|p| p.capabilities.contains(&required.to_string()))
            .cloned()
            .collect::<Vec<_>>();

        candidates.sort_by(|a, b| b.reliability.partial_cmp(&a.reliability).unwrap());

        let selected = candidates.first().cloned();

        if let Some(provider) = &selected {
            println!("[ARBITRATOR] selected provider: {}", provider.provider_id);
        }

        selected
    }
}
