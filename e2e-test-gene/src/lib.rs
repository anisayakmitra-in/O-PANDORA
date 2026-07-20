//! e2e-test-gene gene
use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
#[derive(Debug)]
pub struct e2e_test_geneGene { m: GeneManifest }
impl e2e_test_geneGene { pub fn new() -> Self { Self { m: GeneManifestBuilder::default().id("e2e-test-gene").name("e2e-test-gene").kind(GeneKind::Tool).version("0.1.0").author("").description("e2e-test-gene gene").build() } } }
impl Gene for e2e_test_geneGene { fn manifest(&self) -> &GeneManifest { &self.m } fn execute(&self, i: &str) -> Result<String, String> { Ok(format!("executed: {i}")) } }
