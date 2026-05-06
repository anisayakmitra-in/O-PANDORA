use pandora_gene::{
    find_best_gene,
    load_genes,
    save_genes,
    update_gene_stats,
};

use pandora_harness::HarnessRunner;
use pandora_types::HarnessSpec;

#[tokio::main]
async fn main() {

    let mut runner =
        HarnessRunner::new(
            "http://127.0.0.1:11434"
        );

    let harnesses = vec![

        HarnessSpec {
            name: "coding".to_string(),
            domain: "rust".to_string(),
            allowed_tools: vec![],
            max_steps: 5,
            requires_validation: false,
        },

        HarnessSpec {
            name: "business".to_string(),
            domain: "business".to_string(),
            allowed_tools: vec![],
            max_steps: 5,
            requires_validation: false,
        },
    ];

    let mut harness_genes =
        load_genes("genes");

    let input =
        "Explain Rust ownership in 2 lines";

    println!(
        "LOADED GENES: {}",
        harness_genes.len()
    );

    if let Some(gene) =
        find_best_gene(
            input,
            &harness_genes
        )
    {
        println!(
            "BEST GENE: {}",
            gene.name
        );
    }

    let result = runner
        .run_with_specs(
            "qwen2.5-coder:7b",
            input,
            &harnesses,
        )
        .await;

    match result {

        Ok(output) => {

            update_gene_stats(
                &mut harness_genes,
                "coding",
                2,
            );
            save_genes(
                "genes",
                 &harness_genes,
            );

            println!(
                "RESULT:\n{}",
                output
            );
        }

        Err(e) => {

            println!(
                "ERROR: {}",
                e
            );
        }
    }
}
