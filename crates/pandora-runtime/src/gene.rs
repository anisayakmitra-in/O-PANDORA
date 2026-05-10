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

    pub instructions: String,

    pub schema_version: String,

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
            "skill",
            "mcp",
            "provider",
            "memory",
            "tool",
            "interface",
            "runtime",
            "agent",
            "subagent",
        ];

    if gene.name.is_empty() {

        return false;
    }

    if gene.namespace.is_empty() {

        return false;
    }

    if gene.instructions.is_empty() {

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
            fs::read_to_string(
                &path
            )
            .unwrap();

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
                else {

                    println!(
                        "INVALID GENE: {:?}",
                        path
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
