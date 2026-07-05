//! Pandora Coding Engine — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodePatch {
    pub target_file: String,

    pub search: String,

    pub replace: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchResult {
    pub success: bool,

    pub modified_lines: usize,

    pub output: String,
}

pub struct AutonomousCodingEngine;

impl AutonomousCodingEngine {
    pub fn apply_patch(patch: &CodePatch) -> PatchResult {
        println!("[CODING] patching {}", patch.target_file);

        let content = match fs::read_to_string(&patch.target_file) {
            Ok(data) => data,

            Err(error) => {
                return PatchResult {
                    success: false,

                    modified_lines: 0,

                    output: error.to_string(),
                };
            }
        };

        let count = content.matches(&patch.search).count();

        let updated = content.replace(&patch.search, &patch.replace);

        match fs::write(&patch.target_file, updated) {
            Ok(_) => PatchResult {
                success: true,

                modified_lines: count,

                output: format!("patched {} matches", count),
            },

            Err(error) => PatchResult {
                success: false,

                modified_lines: 0,

                output: error.to_string(),
            },
        }
    }
}
