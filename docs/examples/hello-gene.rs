//! Hello Gene — minimal first-party gene example.
//! Use as template for new gene implementations.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};

#[derive(Debug)]
pub struct HelloGene { m: GeneManifest }

impl HelloGene {
    pub fn new() -> Self {
        Self {
            m: GeneManifestBuilder::default()
                .id("hello")
                .name("Hello")
                .kind(GeneKind::Tool)
                .version("0.1.0")
                .author("example")
                .description("Example hello world gene")
                .build().unwrap(),
        }
    }
}

impl Gene for HelloGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn id(&self) -> &str { "hello" }
    fn kind(&self) -> GeneKind { GeneKind::Tool }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("Hello from gene: {}!", input))
    }
}
