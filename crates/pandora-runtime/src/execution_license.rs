use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionArtifact {
    pub artifact_id: String,

    pub creator: String,

    pub constitutional_grade: String,

    pub execution_license: String,

    pub synthetic: bool,

    pub benchmark_certified: bool,

    pub replay_verified: bool,

    pub autonomy_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionDirective {
    pub artifact_id: String,

    pub execution_allowed: bool,

    pub sovereign_runtime_allowed: bool,

    pub mutation_allowed: bool,

    pub autonomy_expansion_allowed: bool,

    pub quarantine_required: bool,

    pub execution_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionState {
    pub constitutional_execution_integrity: f64,

    pub sovereign_runtime_stability: f64,

    pub autonomy_governance_stability: f64,

    pub sovereign_execution_safe: bool,

    pub directives: Vec<ExecutionDirective>,
}

pub struct ConstitutionalExecutionLicenseEngine;

impl ConstitutionalExecutionLicenseEngine {
    pub fn authorize(artifacts: &[ExecutionArtifact]) -> ExecutionState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut runtime = 0.0;

        let mut autonomy = 0.0;

        for artifact in artifacts {
            println!("[EXECUTION] artifact={}", artifact.artifact_id);

            let execution_allowed = artifact.benchmark_certified && artifact.replay_verified;

            let sovereign_runtime_allowed = artifact.constitutional_grade == "sovereign"
                || artifact.constitutional_grade == "constitutional";

            let mutation_allowed = artifact.execution_license != "immutable";

            let autonomy_expansion_allowed =
                artifact.autonomy_level > 0.84 && sovereign_runtime_allowed;

            let quarantine_required =
                artifact.synthetic && artifact.execution_license == "synthetic-experimental";

            let execution_score = (if execution_allowed { 1.0 } else { 0.0 } * 0.25)
                + (if sovereign_runtime_allowed { 1.0 } else { 0.0 } * 0.25)
                + (if mutation_allowed { 1.0 } else { 0.0 } * 0.15)
                + (if autonomy_expansion_allowed { 1.0 } else { 0.0 } * 0.20)
                + (if !quarantine_required { 1.0 } else { 0.0 } * 0.15);

            directives.push(ExecutionDirective {
                artifact_id: artifact.artifact_id.clone(),

                execution_allowed,

                sovereign_runtime_allowed,

                mutation_allowed,

                autonomy_expansion_allowed,

                quarantine_required,

                execution_score,
            });

            integrity += execution_score;

            runtime += if sovereign_runtime_allowed { 1.0 } else { 0.0 };

            autonomy += artifact.autonomy_level;
        }

        let count = artifacts.len() as f64;

        let constitutional_execution_integrity = integrity / count;

        let sovereign_runtime_stability = runtime / count;

        let autonomy_governance_stability = autonomy / count;

        let sovereign_execution_safe = constitutional_execution_integrity > 0.82
            && sovereign_runtime_stability > 0.81
            && autonomy_governance_stability > 0.78;

        ExecutionState {
            constitutional_execution_integrity,

            sovereign_runtime_stability,

            autonomy_governance_stability,

            sovereign_execution_safe,

            directives,
        }
    }
}
