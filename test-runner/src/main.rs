use pandora_gene::{
    mutate_gene,
    save_gene,
};

use pandora_gene::{
    find_best_gene,
    load_genes,
    save_genes,
    sync_genes_with_memory,
    update_gene_stats,
};

use pandora_memory::HarnessPerformance;
use pandora_harness::HarnessRunner;
use pandora_types::HarnessSpec;

#[tokio::main]
async fn main() {
      
    let genes =
    load_genes("genes");

    let coding_gene =
        genes.iter()
        .find(|g| g.name == "coding")
        .unwrap();

    let mutated =
        mutate_gene(
        coding_gene,
        "coding"
);

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
    
    let memory =
    HarnessPerformance::load(
        "memory/performance.json"
    );

sync_genes_with_memory(
    &mut harness_genes,
    &memory,
);

    let input =
        "Explain Rust ownership in 2 lines";

    println!("LOADED GENES: {}", genes.len());

    let coding_gene =
        genes.iter()
        .find(|g| g.name == "coding")
    .unwrap();

    let mutated =
    mutate_gene(
        coding_gene,
        "coding"
);

    let result = runner
        .run_with_specs(
            "qwen2.5-coder:7b",
            input,
            &harnesses,
        )
        .await;

    let coding_gene =
        genes.iter()
        .find(|g| g.name == "coding")
        .unwrap();

    let mutated =
        mutate_gene(
        coding_gene,
        "coding"
);

println!(
    "MUTATED GENE: {}",
    mutated.gene_id
);

save_gene(
    &mutated,
    "genes/coding-v2.json"
);  

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
   
