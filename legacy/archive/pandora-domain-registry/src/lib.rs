//! Pandora Domain Registry — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainGenePack {
    pub pack_id: String,

    pub domain: String,

    pub meta_harness: String,

    pub genes: Vec<String>,

    pub governance_score: f64,

    pub survivability_score: f64,

    pub replay_compatible: bool,

    pub heterogeneous_ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentCompatibility {
    pub substrate: String,

    pub supported_domains: Vec<String>,

    pub replay_support: bool,

    pub sandbox_support: bool,

    pub telemetry_support: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryDirective {
    pub pack_id: String,

    pub installable: bool,

    pub governance_review: bool,

    pub benchmark_required: bool,

    pub sovereign_approved: bool,

    pub deployment_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryState {
    pub ecosystem_integrity: f64,

    pub sovereign_registry_ready: bool,

    pub directives: Vec<RegistryDirective>,
}

pub struct DomainGenePackRegistry;

impl DomainGenePackRegistry {
    pub fn validate(
        packs: &[DomainGenePack],

        substrates: &[DeploymentCompatibility],
    ) -> RegistryState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        for pack in packs {
            println!("[REGISTRY] pack={}", pack.pack_id);

            let compatible = substrates.iter().any(|substrate| {
                substrate.supported_domains.contains(&pack.domain)
                    && substrate.replay_support
                    && substrate.sandbox_support
                    && substrate.telemetry_support
            });

            let governance_review = pack.governance_score < 0.80;

            let benchmark_required = pack.survivability_score < 0.85;

            let sovereign_approved = pack.governance_score > 0.88
                && pack.survivability_score > 0.87
                && pack.replay_compatible
                && compatible;

            let installable = compatible && pack.replay_compatible;

            let deployment_class = if pack.heterogeneous_ready {
                "heterogeneous-governed"
            } else {
                "stable-specialized"
            };

            directives.push(RegistryDirective {
                pack_id: pack.pack_id.clone(),

                installable,

                governance_review,

                benchmark_required,

                sovereign_approved,

                deployment_class: deployment_class.into(),
            });

            integrity += (pack.governance_score * 0.50) + (pack.survivability_score * 0.50);
        }

        let ecosystem_integrity = integrity / packs.len() as f64;

        let sovereign_registry_ready = ecosystem_integrity > 0.86;

        RegistryState {
            ecosystem_integrity,

            sovereign_registry_ready,

            directives,
        }
    }
}
