//! custom-harness-meta — a custom meta harness for O-PANDORA.
//!
//! Meta harnesses handle communication and orchestration between
//! other harnesses. They provide the mesh that connects Source and
//! Domain harnesses.

use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use pandora_types::PandoraError;

#[derive(Debug)]
pub struct CustomMetaHarness { m: HarnessManifest }

impl CustomMetaHarness {
    pub fn new() -> Self {
        let manifest = HarnessManifestBuilder::default()
            .id("custom-harness-meta")
            .name("Custom Meta Harness")
            .kind(HarnessKind::Meta)
            .version("0.1.0")
            .author("")
            .description("A custom meta harness for orchestration")
            .capability("routing")
            .capability("mesh")
            .build().expect("valid manifest");
        Self { m: manifest }
    }
}

impl Harness for CustomMetaHarness {
    fn manifest(&self) -> &HarnessManifest { &self.m }
    fn initialize(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-meta] initialized");
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), PandoraError> {
        println!("[custom-harness-meta] shutdown");
        Ok(())
    }
    fn health(&self) -> Result<(), PandoraError> { Ok(()) }
}
