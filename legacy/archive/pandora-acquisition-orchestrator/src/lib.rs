//! Pandora Acquisition Orchestrator — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionCandidate {
    pub candidate_id: String,

    pub provider: String,

    pub capability_domains: Vec<String>,

    pub governance_score: f64,

    pub compatibility_score: f64,

    pub deployment_stability: f64,

    pub quantization_profiles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentTarget {
    pub substrate: String,

    pub compute_pressure: f64,

    pub memory_pressure: f64,

    pub telemetry_health: f64,

    pub sandbox_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcquisitionDeploymentPlan {
    pub candidate: String,

    pub provider: String,

    pub substrate: String,

    pub quantization: String,

    pub deployment_mode: String,

    pub governance_required: bool,

    pub approved: bool,
}

pub struct AcquisitionOrchestrator;

impl AcquisitionOrchestrator {
    pub fn orchestrate(
        domain: &str,

        candidates: &[AcquisitionCandidate],

        targets: &[DeploymentTarget],
    ) -> Option<AcquisitionDeploymentPlan> {
        println!("[ACQUISITION] domain={}", domain);

        let candidate = candidates
            .iter()
            .filter(|candidate| candidate.capability_domains.contains(&domain.to_string()))
            .max_by(|a, b| {
                let score_a = (a.governance_score * 0.40)
                    + (a.compatibility_score * 0.35)
                    + (a.deployment_stability * 0.25);

                let score_b = (b.governance_score * 0.40)
                    + (b.compatibility_score * 0.35)
                    + (b.deployment_stability * 0.25);

                score_a.partial_cmp(&score_b).unwrap()
            })?;

        let target = targets
            .iter()
            .filter(|target| target.sandbox_ready)
            .max_by(|a, b| {
                let score_a = (a.telemetry_health * 0.45)
                    + ((1.0 - a.compute_pressure) * 0.30)
                    + ((1.0 - a.memory_pressure) * 0.25);

                let score_b = (b.telemetry_health * 0.45)
                    + ((1.0 - b.compute_pressure) * 0.30)
                    + ((1.0 - b.memory_pressure) * 0.25);

                score_a.partial_cmp(&score_b).unwrap()
            })?;

        let default_quantization = String::from("fp16");

        let quantization = if target.memory_pressure > 0.75 {
            "q4_k_m"
        } else if target.memory_pressure > 0.50 {
            "q5_k_m"
        } else {
            candidate
                .quantization_profiles
                .first()
                .unwrap_or(&default_quantization)
        };

        let deployment_mode = if target.compute_pressure > 0.80 {
            "distributed-offload"
        } else if target.telemetry_health > 0.90 {
            "stable-governed"
        } else {
            "sandbox-constrained"
        };

        let governance_required = candidate.governance_score < 0.85;

        let approved = candidate.compatibility_score > 0.80 && target.sandbox_ready;

        Some(AcquisitionDeploymentPlan {
            candidate: candidate.candidate_id.clone(),

            provider: candidate.provider.clone(),

            substrate: target.substrate.clone(),

            quantization: quantization.into(),

            deployment_mode: deployment_mode.into(),

            governance_required,

            approved,
        })
    }
}
