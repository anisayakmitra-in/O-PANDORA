//! custom-harness-domain — a custom domain harness for O-PANDORA.
//!
//! Domain harnesses package policies, workflows, capabilities, and genes
//! for a specific domain (e.g., coding, security, research, design).
//!
//! This example shows the minimum contract required to implement a Domain harness.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use pandora_types::PandoraError;

#[derive(Debug)]
pub struct CustomDomainHarness {
    m: HarnessManifest,
}

impl CustomDomainHarness {
    pub fn new() -> Self {
        let manifest = HarnessManifestBuilder::default()
            .id("custom-harness-domain")
            .name("Custom Domain Harness")
            .kind(HarnessKind::Domain)
            .version("0.1.0")
            .author("")
            .description("A custom domain harness")
            .capability("custom-domain")
            .build()
            .expect("valid manifest");

        Self { m: manifest }
    }
}

impl Harness for CustomDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.m
    }

    fn initialize(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-domain] initialized");
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-domain] shutdown");
        Ok(())
    }

    fn health(&self) -> Result<(), PandoraError> {
        Ok(())
    }
}
