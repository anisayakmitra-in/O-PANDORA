use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};

#[derive(Debug)]
pub struct HelloGene {
    manifest: GeneManifest,
}

impl Default for HelloGene {
    fn default() -> Self {
        Self::new()
    }
}

impl HelloGene {
    pub fn new() -> Self {
        let manifest = GeneManifestBuilder::default()
            .id("hello-gene")
            .name("Hello Gene")
            .kind(GeneKind::Tool)
            .version("0.2.0")
            .author("you")
            .description("A simple greeting gene")
            .build()
            .expect("manifest must build");
        Self { manifest }
    }
}

impl Gene for HelloGene {
    fn manifest(&self) -> &GeneManifest {
        &self.manifest
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("Hello from gene! Input was: {}", input))
    }
}

fn main() {
    let gene = HelloGene::new();
    let m = gene.manifest();
    println!("Gene: {} (v{})", m.name, m.version);
    println!("Kind: {:?}", m.kind);
    println!("Author: {}", m.author);
    match gene.execute("world") {
        Ok(r) => println!("Execute: {}", r),
        Err(e) => eprintln!("Error: {}", e),
    }
}
