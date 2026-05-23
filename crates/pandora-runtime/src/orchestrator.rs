use crate::harness::MetaHarness;
use crate::harness_loader::HarnessLoader;

pub struct PandoraRuntime {
    pub harnesses: Vec<Box<dyn MetaHarness>>,
}

impl PandoraRuntime {
    pub fn new() -> Self {
        Self {
            harnesses: Vec::new(),
        }
    }

    pub fn register_harness(&mut self, harness: Box<dyn MetaHarness>) {
        self.harnesses.push(harness);
    }

    pub fn run(&self) {
        println!("Pandora Runtime Started\n");

        let manifests = HarnessLoader::discover();

        println!("[DISCOVERED META-HARNESSES] {}\n", manifests.len());

        for manifest in manifests {
            println!("[HARNESS] {}", manifest.name);

            println!("VERSION: {}", manifest.version);

            println!("AUTHOR: {}", manifest.author);

            println!("DESCRIPTION: {}\n", manifest.description);
        }

        println!("[ACTIVE META-HARNESSES] {}", self.harnesses.len());
    }
}
