use crate::gene::GeneManifest;

#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub active_gene: Option<GeneManifest>,

    pub loaded_genes: Vec<GeneManifest>,

    pub active_provider: Option<String>,

    pub active_harness: Option<String>,

    pub runtime_status: String,
}

impl RuntimeState {
    pub fn new(genes: Vec<GeneManifest>) -> Self {
        Self {
            active_gene: genes.first().cloned(),

            loaded_genes: genes,

            active_provider: Some(String::from("ollama")),

            active_harness: Some(String::from("PANOPTES")),

            runtime_status: String::from("operational"),
        }
    }
}
