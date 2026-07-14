use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactIdentity {
    pub artifact_id: String,

    pub artifact_type: String,

    pub creator: String,

    pub provenance: String,

    pub synthetic: bool,

    pub signed: bool,

    pub benchmark_certified: bool,

    pub constitutional_grade: String,

    pub mutation_policy: String,

    pub replay_lineage: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceDirective {
    pub artifact_id: String,

    pub creator_verified: bool,

    pub synthetic_separated: bool,

    pub replay_verified: bool,

    pub marketplace_allowed: bool,

    pub mutation_authorized: bool,

    pub provenance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvenanceState {
    pub constitutional_provenance_integrity: f64,

    pub replay_lineage_integrity: f64,

    pub ecosystem_trust_stability: f64,

    pub sovereign_provenance_stable: bool,

    pub directives: Vec<ProvenanceDirective>,
}

pub struct ConstitutionalArtifactProvenanceEngine;

impl ConstitutionalArtifactProvenanceEngine {
    pub fn verify(artifacts: &[ArtifactIdentity]) -> ProvenanceState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut replay = 0.0;

        let mut trust = 0.0;

        for artifact in artifacts {
            println!("[PROVENANCE] artifact={}", artifact.artifact_id);

            let creator_verified = artifact.signed;

            let synthetic_separated = if artifact.synthetic {
                artifact.provenance.starts_with("pandora.synthetic")
            } else {
                artifact.provenance.starts_with("pandora@")
            };

            let replay_verified = artifact.replay_lineage.len() > 0;

            let marketplace_allowed = artifact.benchmark_certified && creator_verified;

            let mutation_authorized = artifact.mutation_policy != "immutable";

            let provenance_score = (if creator_verified { 1.0 } else { 0.0 } * 0.25)
                + (if synthetic_separated { 1.0 } else { 0.0 } * 0.20)
                + (if replay_verified { 1.0 } else { 0.0 } * 0.20)
                + (if marketplace_allowed { 1.0 } else { 0.0 } * 0.20)
                + (if mutation_authorized { 1.0 } else { 0.0 } * 0.15);

            directives.push(ProvenanceDirective {
                artifact_id: artifact.artifact_id.clone(),

                creator_verified,

                synthetic_separated,

                replay_verified,

                marketplace_allowed,

                mutation_authorized,

                provenance_score,
            });

            integrity += provenance_score;

            replay += if replay_verified { 1.0 } else { 0.0 };

            trust += if creator_verified { 1.0 } else { 0.0 };
        }

        let count = artifacts.len() as f64;

        let constitutional_provenance_integrity = integrity / count;

        let replay_lineage_integrity = replay / count;

        let ecosystem_trust_stability = trust / count;

        let sovereign_provenance_stable = constitutional_provenance_integrity > 0.84
            && replay_lineage_integrity > 0.82
            && ecosystem_trust_stability > 0.84;

        ProvenanceState {
            constitutional_provenance_integrity,

            replay_lineage_integrity,

            ecosystem_trust_stability,

            sovereign_provenance_stable,

            directives,
        }
    }
}
