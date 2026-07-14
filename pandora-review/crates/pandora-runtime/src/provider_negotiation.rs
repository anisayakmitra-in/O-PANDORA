use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareSubstrate {
    pub substrate: String,

    pub compute_capacity: f64,

    pub memory_capacity: f64,

    pub telemetry_health: f64,

    pub heterogeneous: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderBackend {
    pub provider: String,

    pub supported_domains: Vec<String>,

    pub governance_score: f64,

    pub deployment_stability: f64,

    pub quantization_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedExecution {
    pub provider: String,

    pub substrate: String,

    pub quantization: String,

    pub topology: String,

    pub governance_required: bool,
}

pub struct ProviderHardwareNegotiator;

impl ProviderHardwareNegotiator {
    pub fn negotiate(
        domain: &str,

        hardware: &[HardwareSubstrate],

        providers: &[ProviderBackend],
    ) -> Option<NegotiatedExecution> {
        println!("[NEGOTIATION] domain={}", domain);

        let provider = providers
            .iter()
            .filter(|provider| provider.supported_domains.contains(&domain.to_string()))
            .max_by(|a, b| {
                let score_a = (a.governance_score * 0.55) + (a.deployment_stability * 0.45);

                let score_b = (b.governance_score * 0.55) + (b.deployment_stability * 0.45);

                score_a.partial_cmp(&score_b).unwrap()
            })?;

        let substrate = hardware.iter().max_by(|a, b| {
            let score_a = (a.compute_capacity * 0.50)
                + (a.memory_capacity * 0.30)
                + (a.telemetry_health * 0.20);

            let score_b = (b.compute_capacity * 0.50)
                + (b.memory_capacity * 0.30)
                + (b.telemetry_health * 0.20);

            score_a.partial_cmp(&score_b).unwrap()
        })?;

        let quantization = if provider.quantization_support {
            if substrate.memory_capacity < 0.50 {
                "q4_k_m"
            } else if substrate.memory_capacity < 0.75 {
                "q5_k_m"
            } else {
                "fp16"
            }
        } else {
            "native"
        };

        let topology = if substrate.heterogeneous {
            "heterogeneous-distributed"
        } else {
            "stable-local"
        };

        Some(NegotiatedExecution {
            provider: provider.provider.clone(),

            substrate: substrate.substrate.clone(),

            quantization: quantization.into(),

            topology: topology.into(),

            governance_required: provider.governance_score < 0.80,
        })
    }
}
