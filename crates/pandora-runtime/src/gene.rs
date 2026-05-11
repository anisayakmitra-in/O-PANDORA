use std::fs;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct GeneSignature {

    pub algorithm: String,

    pub signed_by: String,

    pub public_key_id: String,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct GeneLineage {

    pub generation: u32,

    pub parent: String,

    pub mutation: String,

    pub fitness: f32,
}

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
)]
pub struct GeneManifest {

    pub name: String,

    pub namespace: String,

    pub gene_type: String,

    pub version: String,

    pub schema_version: String,

    pub description: String,
    
    pub instructions: String,
    
    pub capabilities: Vec<String>,

    pub required_harnesses: Vec<String>,

    pub dependencies: Vec<String>,

    pub permissions: Vec<String>,

    pub entrypoint: String,

    pub compatible_runtimes: Vec<String>,

    pub signature: GeneSignature,

    pub lineage: GeneLineage,

}

pub fn validate_gene(
    gene: &GeneManifest
)
    -> bool
{

    let valid_types =
        vec![
            "meta_harness",
            "workflow",
            "tool",
            "provider",
            "memory",
            "governance",
            "sandbox",
            "security",
            "gateway",
            "scheduler",
            "runtime",
            "kernel",
            "agent",
            "subagent",
            "interface",
            "training",
            "dataset",
            "evaluation",
            "inference",
            "hardware",
            "distributed",
            "routing",
            "telemetry",
            "execution",
            "automation",
            "integration",
            "connector",
            "bridge",
            "registry",
            "marketplace",
            "identity",
            "persona",
            "adapter",
            "optimization",
            "evolution",
            "mutation",
            "reflection",
            "planning",
            "reasoning",
            "retrieval",
            "embedding",
            "compression",
            "simulation",
            "research",
            "compliance",
            "audit",
            "containment",
            "policy",
            "verification",
            "monitoring",
            "deployment",
            "infrastructure",
            "observability",
            "storage",
            "database",
            "network",
            "communication",
            "mcp",
            "skill",
            "plugin",
            "service",
            "orchestrator",
            "compiler",
            "translator",
            "multimodal",
            "voice",
            "vision",
            "robotics",
            "sensor",
            "mobile",
            "desktop",
            "web",
            "api",
            "filesystem",
            "browser",
            "shell",
            "search",
            "scraper",
            "crawler",
            "validator",
            "parser",
            "transformer",
            "executor",
            "indexer",
            "cache",
            "archive",
            "consensus",
            "swarm",
            "cluster",
            "federation",
            "replication",
            "synchronization",
            "quantization",
            "alignment",
            "benchmark",
            "judge",
            "critic",
            "redteam",
            "simulation_environment",
            "world_model",
            "cognitive_model",
            "user_model",
            "behavior_model",
            "preference_model",
            "localization",
            "virtualization",
            "container",
            "backup",
            "migration",
            "recovery",
        ];

    if gene.name.is_empty() {

        return false;
    }

    if gene.namespace.is_empty() {

        return false;
    }

    if gene.description.is_empty() {

        return false;
    }

    if !valid_types.contains(
        &gene.gene_type.as_str()
    ) {

        return false;
    }

    true
}

pub fn load_genes()
    -> Vec<GeneManifest>
{

    let mut genes =
        Vec::new();

    let paths =
        fs::read_dir(
            "genes/installed"
        )
        .unwrap();

    for entry in paths {

        let entry =
            entry.unwrap();

        let path =
            entry.path();

        println!(
            "LOADING: {:?}",
            path
        );

        let contents =
            if path.is_file() {

                match fs::read_to_string(&path) {

                    Ok(contents) => {

                        // existing parse logic
                }

                Err(error) => {

                    println!(
                        "[WARN] failed to read gene file {:?}: {}",
                        path,
                        error
                    );
                }
            }
        }

        match serde_json::from_str::<GeneManifest>(
            &contents
        ) {

            Ok(gene) => {

                println!(
                    "PARSED GENE: {}",
                    gene.name
                );

                if validate_gene(
                    &gene
                ) {

                    genes.push(
                        gene
                    );
                }
            }

            Err(error) => {

                println!(
                    "FAILED TO PARSE {:?}",
                    path
                );

                println!(
                    "ERROR: {}",
                    error
                );
            }
        }
    }

    genes
}

