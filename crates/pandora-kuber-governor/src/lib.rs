//! Pandora Kuber Governor — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemCreator {
    pub creator_id: String,

    pub published_packs: usize,

    pub survivability_reputation: f64,

    pub governance_reputation: f64,

    pub replay_authenticity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemArtifact {
    pub artifact_id: String,

    pub artifact_type: String,

    pub creator_id: String,

    pub benchmark_integrity: f64,

    pub mutation_risk: f64,

    pub topology_stability: f64,

    pub replay_verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceVerdict {
    pub artifact_id: String,

    pub certified: bool,

    pub quarantine: bool,

    pub promotion_priority: String,

    pub governance_review: bool,

    pub trust_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGovernanceState {
    pub ecosystem_stability: f64,

    pub replay_integrity: f64,

    pub constitutional_trust: f64,

    pub sovereign_market_ready: bool,

    pub verdicts: Vec<GovernanceVerdict>,
}

pub struct KuberPalaceGovernor;

impl KuberPalaceGovernor {
    pub fn govern(
        creators: &[EcosystemCreator],

        artifacts: &[EcosystemArtifact],
    ) -> EcosystemGovernanceState {
        let mut verdicts = Vec::new();

        let mut stability = 0.0;

        let mut replay = 0.0;

        let mut trust = 0.0;

        for artifact in artifacts {
            println!("[KUBER] artifact={}", artifact.artifact_id);

            let creator = creators
                .iter()
                .find(|creator| creator.creator_id == artifact.creator_id);

            let creator_score = creator
                .map(|creator| {
                    (creator.survivability_reputation * 0.40)
                        + (creator.governance_reputation * 0.35)
                        + (creator.replay_authenticity * 0.25)
                })
                .unwrap_or(0.0);

            let trust_score = (artifact.benchmark_integrity * 0.30)
                + (artifact.topology_stability * 0.25)
                + (creator_score * 0.30)
                + (if artifact.replay_verified { 1.0 } else { 0.0 } * 0.15);

            let certified = trust_score > 0.86;

            let quarantine = artifact.mutation_risk > 0.88;

            let governance_review = trust_score < 0.72;

            let promotion_priority = if certified {
                "constitutional-tier"
            } else if trust_score > 0.74 {
                "stable-tier"
            } else {
                "restricted-tier"
            };

            verdicts.push(GovernanceVerdict {
                artifact_id: artifact.artifact_id.clone(),

                certified,

                quarantine,

                promotion_priority: promotion_priority.into(),

                governance_review,

                trust_score,
            });

            stability += artifact.topology_stability;

            replay += if artifact.replay_verified { 1.0 } else { 0.0 };

            trust += trust_score;
        }

        let count = artifacts.len() as f64;

        let ecosystem_stability = stability / count;

        let replay_integrity = replay / count;

        let constitutional_trust = trust / count;

        let sovereign_market_ready =
            ecosystem_stability > 0.82 && replay_integrity > 0.80 && constitutional_trust > 0.84;

        EcosystemGovernanceState {
            ecosystem_stability,

            replay_integrity,

            constitutional_trust,

            sovereign_market_ready,

            verdicts,
        }
    }
}
