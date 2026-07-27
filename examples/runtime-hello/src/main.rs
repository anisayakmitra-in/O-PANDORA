//! runtime-hello — the simplest way to use PandoraRuntime programmatically.
//!
//! This example shows how to load a gene, wire it into the runtime,
//! and execute a task — all from your own Rust code.
//!
//! Run with: cargo run --example hello

use pandora_orchestrator::PandoraRuntime;
use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use pandora_types::PandoraError;

/// A tiny gene that echoes its input.
#[derive(Debug)]
struct EchoGene {
    m: GeneManifest,
}

impl EchoGene {
    fn new() -> Self {
        Self {
            m: GeneManifestBuilder::default()
                .id("echo")
                .name("Echo Gene")
                .kind(GeneKind::Tool)
                .version("0.1.0")
                .author("you")
                .description("Echoes whatever you say back to you")
                .build(),
        }
    }
}

impl Gene for EchoGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, PandoraError> {
        Ok(format!("echo: {input}"))
    }
}

/// A tiny harness that holds the echo gene.
#[derive(Debug)]
struct EchoHarness {
    m: HarnessManifest,
}

impl EchoHarness {
    fn new() -> Self {
        Self {
            m: HarnessManifestBuilder::default()
                .id("echo-harness")
                .name("Echo Harness")
                .kind(HarnessKind::Domain)
                .version("0.1.0")
                .author("you")
                .description("A tiny harness for the echo gene")
                .build()
                .expect("valid manifest"),
        }
    }
}

impl Harness for EchoHarness {
    fn manifest(&self) -> &HarnessManifest { &self.m }
}

#[tokio::main]
async fn main() {
    // 1. Create the runtime
    let mut runtime = PandoraRuntime::new();

    // 2. Register a gene
    let gene = EchoGene::new();
    runtime.council.install_gene(Box::new(gene));

    // 3. Register a harness
    let harness = EchoHarness::new();
    runtime.council.install(Box::new(harness)).ok();

    // 4. Run a task
    match runtime.run("say hello", "echo").await {
        Ok(report) => {
            println!("✓ Execution complete");
            println!("  Session:  {}", report.execution_id);
            println!("  Provider: {}", report.provider);
            println!("  Duration: {}ms", report.duration_ms);
            println!("  Output:   {}", report.output);
        }
        Err(e) => {
            eprintln!("✗ Execution failed: {e}");
        }
    }
}
