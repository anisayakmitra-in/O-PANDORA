use serde::{
    Deserialize,
    Serialize,
};

use std::fs;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GeneLineage {

    pub parent:
        String,

    pub generation:
        u32,

    pub mutation:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GeneSignature {

    pub algorithm:
        String,

    pub signed_by:
        String,

    pub public_key_id:
        String,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GeneTier {

    pub classification:
        String,

    pub monetized:
        bool,

    pub trusted:
        bool,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct GeneManifest {

    pub name:
        String,

    pub namespace:
        String,

    pub version:
        String,

    pub schema_version:
        String,

    pub gene_type:
        String,

    pub tags:
        Vec<String>,

    pub entry:
        String,

    pub lineage:
        GeneLineage,

    pub signature:
        GeneSignature,

    pub tier:
        GeneTier,
}

pub fn load_genes()
    -> Vec<GeneManifest>
{
    
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

    if gene.schema_version != "1.0" {

        return false;
    }

    if !valid_types.contains(
        &gene.gene_type.as_str()
    ) {

        return false;
    }

    if gene.lineage.generation == 0 {

        return false;
    }

    if gene.signature.algorithm
        != "RSA-2048"
    {

        return false;
    }

    true
}

    let mut genes =
        Vec::new();

    let paths =
        fs::read_dir(
            "genes"
        )
        .unwrap();

    for path in paths {

        let entry =
            path.unwrap();

        let file_path =
            entry.path();

        if file_path
            .extension()
            .and_then(
                |ext|
                ext.to_str()
            ) == Some("json")
        {

            let contents =
                fs::read_to_string(
                    &file_path
                )
                .unwrap_or_default();

            let parsed =
                serde_json::from_str::<
                    GeneManifest
                >(
                    &contents
                );

            if let Ok(gene) =
                parsed
            {

                genes.push(
                    gene
                );
            }
        }
    }

    genes
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

    if gene.schema_version != "1.0" {

        return false;
    }

    if !valid_types.contains(
        &gene.gene_type.as_str()
    ) {

        return false;
    }

    if gene.lineage.generation == 0 {

        return false;
    }

    if gene.signature.algorithm
        != "RSA-2048"
    {

        return false;
    }

    true
}
