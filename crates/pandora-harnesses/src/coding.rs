//! Coding Domain Harness — packages developer workflow genes together.
//!
//! Part of Stage 2: First-Party Ecosystem.
//! A Domain Harness packages experiences — it bundles genes,
//! adds slash commands, and configures the domain environment.

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use std::sync::Arc;

#[derive(Debug)]
pub struct CodingDomainHarness {
    manifest: HarnessManifest,
    genes: Vec<Arc<dyn Gene>>,
}

impl CodingDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("coding-domain")
                .name("Coding")
                .version("0.1.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("Developer workflow domain — coding, review, build, test")
                .capability("coding")
                .capability("development")
                .capability("build-automation")
                .slash_command("/build", "Build the current project")
                .slash_command("/test", "Run tests")
                .slash_command("/lint", "Run linter")
                .slash_command("/review", "Review code changes")
                .build()
                .unwrap(),
            genes: Vec::new(),
        }
    }

    /// Register a gene that belongs to this domain.
    pub fn add_gene(&mut self, gene: Arc<dyn Gene>) {
        self.genes.push(gene);
    }

    pub fn gene_count(&self) -> usize {
        self.genes.len()
    }
}

impl Harness for CodingDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }

    fn initialize(&mut self) -> Result<(), String> {
        // ponytail: domain initialization — just log for now

        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
}
