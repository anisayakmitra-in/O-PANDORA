use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub lineage_id: String,

    pub parent: Option<String>,

    pub harness: String,

    pub continuity_score: f64,

    pub survivability: f64,

    pub divergence_risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageDirective {
    pub lineage_id: String,

    pub preserve: bool,

    pub archive: bool,

    pub quarantine: bool,

    pub sovereign_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereignLineageState {
    pub lineage_integrity: f64,

    pub recursive_continuity: bool,

    pub sovereign_stable: bool,

    pub directives: Vec<LineageDirective>,
}

pub struct RecursiveExecutionLineage;

impl RecursiveExecutionLineage {
    pub fn evaluate(nodes: &[LineageNode]) -> SovereignLineageState {
        let mut directives = Vec::new();

        let mut integrity = 0.0;

        let mut survivability = 0.0;

        for node in nodes {
            println!("[LINEAGE] evaluating {}", node.lineage_id);

            let preserve = node.continuity_score > 0.80 && node.survivability > 0.78;

            let archive = node.parent.is_some();

            let quarantine = node.divergence_risk > 0.82;

            let sovereign_valid = preserve && !quarantine;

            directives.push(LineageDirective {
                lineage_id: node.lineage_id.clone(),

                preserve,

                archive,

                quarantine,

                sovereign_valid,
            });

            integrity += node.continuity_score;

            survivability += node.survivability;
        }

        let count = nodes.len() as f64;

        let lineage_integrity = (integrity / count) * 0.55 + (survivability / count) * 0.45;

        let recursive_continuity = lineage_integrity > 0.80;

        let sovereign_stable = lineage_integrity > 0.90;

        SovereignLineageState {
            lineage_integrity,

            recursive_continuity,

            sovereign_stable,

            directives,
        }
    }
}
