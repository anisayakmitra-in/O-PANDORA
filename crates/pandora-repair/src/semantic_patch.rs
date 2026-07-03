//! Semantic Patch — consolidated into pandora-repair.
//!
//!
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticIssue {
    pub file: String,

    pub issue: String,

    pub severity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticPatch {
    pub target_file: String,

    pub search: String,

    pub replace: String,

    pub confidence: f32,
}

pub struct SemanticPatchPlanner;

impl SemanticPatchPlanner {
    pub fn generate(issue: &SemanticIssue) -> Vec<SemanticPatch> {
        println!("[PATCH] analyzing {}", issue.file);

        let mut patches = Vec::new();

        if issue.issue.contains("unresolved import") {
            patches.push(SemanticPatch {
                target_file: issue.file.clone(),

                search: "use crate::memory;".into(),

                replace: "use crate::semantic_memory;".into(),

                confidence: 0.91,
            });
        }

        if issue.issue.contains("cannot find type") {
            patches.push(SemanticPatch {
                target_file: issue.file.clone(),

                search: "UnknownType".into(),

                replace: "KnownRuntimeType".into(),

                confidence: 0.82,
            });
        }

        if issue.severity > 0.90 {
            patches.push(SemanticPatch {
                target_file: issue.file.clone(),

                search: "unsafe".into(),

                replace: "safe_runtime_guard".into(),

                confidence: 0.76,
            });
        }

        patches
    }
}
