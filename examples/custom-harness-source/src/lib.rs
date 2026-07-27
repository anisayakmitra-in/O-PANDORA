//! custom-harness-source — a custom source harness for O-PANDORA.
//!
//! Source harnesses augment one or more constitutional services.
//! They affect foundational runtime behavior and require explicit
//! approval before being enabled.
//!
//! WARNING: Source harness activation requires:
//!   pandora harness enable-source custom-harness-source <approver> <reason>

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use pandora_types::PandoraError;

#[derive(Debug)]
pub struct CustomSourceHarness { m: HarnessManifest }

impl CustomSourceHarness {
    pub fn new() -> Self {
        let manifest = HarnessManifestBuilder::default()
            .id("custom-harness-source")
            .name("Custom Source Harness")
            .kind(HarnessKind::Source)
            .version("0.1.0")
            .author("")
            .description("A custom source harness for constitutional augmentation")
            .capability("governance")
            .capability("audit")
            .build().expect("valid manifest");
        Self { m: manifest }
    }
}

impl Harness for CustomSourceHarness {
    fn manifest(&self) -> &HarnessManifest { &self.m }
    fn initialize(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-source] initialized — constitutional augmentation active");
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-source] shutdown");
        Ok(())
    }
    fn health(&self) -> Result<(), PandoraError> { Ok(()) }
}
