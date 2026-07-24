//! Skill Gene — loads SKILL.md files as genes.
//!
//! SKILL.md is the industry standard format (used by Hermes, Superpowers, etc.)
//! In Pandora, skills are genes with GeneKind::Skill.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::path::Path;

/// A gene that wraps a SKILL.md file.
/// The SKILL.md content becomes the gene's system prompt injection.
#[derive(Debug, Clone)]
pub struct SkillGene {
    manifest: GeneManifest,
    skill_content: String,
}

impl SkillGene {
    /// Load a SKILL.md file from a directory.
    pub fn load(dir: &Path) -> Result<Self, String> {
        let skill_path = dir.join("SKILL.md");
        let content = std::fs::read_to_string(&skill_path)
            .map_err(|e| format!("Cannot read SKILL.md: {e}"))?;

        // Parse frontmatter (YAML between --- markers)
        let (frontmatter, body) = if content.starts_with("---") {
            let after_prefix = content.strip_prefix("---").unwrap_or("");
            let end = after_prefix.find("---").map(|i| i + 3);
            if let Some(end_idx) = end {
                let fm = &content[3..end_idx];
                let body = &content[end_idx + 3..].trim();
                (fm.to_string(), body.to_string())
            } else {
                (String::new(), content.clone())
            }
        } else {
            (String::new(), content.clone())
        };

        // Extract name from frontmatter or filename
        let name = frontmatter
            .lines()
            .find_map(|l| l.strip_prefix("name:").map(|s| s.trim().to_string()))
            .unwrap_or_else(|| {
                dir.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "skill".into())
            });

        // Extract description from frontmatter
        let description = frontmatter
            .lines()
            .find_map(|l| l.strip_prefix("description:").map(|s| s.trim().to_string()))
            .unwrap_or_default();

        // Extract triggers from frontmatter
        let triggers: Vec<String> = frontmatter
            .lines()
            .filter_map(|l| l.strip_prefix("- ").map(|s| s.trim().to_string()))
            .collect();

        let manifest = GeneManifestBuilder::default()
            .id(name.clone())
            .name(name)
            .kind(GeneKind::Skill)
            .version("0.1.0")
            .author("skill")
            .description(if description.is_empty() {
                "Skill loaded from SKILL.md".to_string()
            } else {
                description
            })
            .build()
            .map_err(|e| format!("Skill manifest build failed: {e}"))?;

        let mut manifest = manifest;
        manifest.capabilities = triggers;
        Ok(Self {
            manifest,
            skill_content: body,
        })
    }

    /// Get the SKILL.md content for system prompt injection.
    pub fn skill_content(&self) -> &str {
        &self.skill_content
    }
}

impl Gene for SkillGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        // Skill genes return their content for system prompt injection
        Ok(format!("{}\n\nTask: {}", self.skill_content, input))
    }
}

/// Discover all SKILL.md files in a directory tree.
pub fn discover_skills(root: &Path) -> Vec<SkillGene> {
    let mut skills = Vec::new();
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() && p.join("SKILL.md").exists() {
                if let Ok(skill) = SkillGene::load(&p) {
                    skills.push(skill);
                }
            }
        }
    }
    skills
}
