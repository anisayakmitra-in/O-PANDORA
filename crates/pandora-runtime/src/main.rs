use chrono::Utc;

use pandora_gene::load_genes;

use anubis_memory::{
    MemoryRecord,
    load_memories,
    search_memories,
    store_memory,
    summarize_memories,
};

fn main() {

    println!(
        "\nPANDORA SYSTEMS\n"
    );

    println!(
        "[BOOT] Initializing runtime..."
    );

    let genes =
        load_genes("genes");

    println!(
        "[GENE] Loaded {} genes",
        genes.len()
    );

    let best_gene =
        genes
            .iter()
            .max_by(
                |a, b| {
                    a.avg_score
                        .partial_cmp(
                            &b.avg_score
                        )
                        .unwrap()
                }
            )
            .unwrap();

    println!(
        "[GENE] Active Gene: {}",
        best_gene.gene_id
    );

    println!(
        "[ANUBIS] Memory online"
    );

    let memory =
        MemoryRecord {

            timestamp:
                Utc::now()
                    .to_rfc3339(),

            gene:
                best_gene
                    .gene_id
                    .clone(),

            harness:
                "coding"
                    .to_string(),

            model:
                "qwen2.5-coder:7b"
                    .to_string(),

            prompt:
                "Explain Rust ownership"
                    .to_string(),

            response:
                "Rust ownership ensures memory safety without garbage collection."
                    .to_string(),

            score:
                best_gene
                    .avg_score,
        };

    store_memory(
        &memory
    );

    println!(
        "[ANUBIS] Runtime memory stored"
    );

    let memories =
        load_memories();

    println!(
        "[ANUBIS] Loaded {} memories",
        memories.len()
    );

    let results =
        search_memories(
            &memories,
            "ownership"
        );

    println!(
        "[ANUBIS] Relevant memories: {}",
        results.len()
    );

    let summary =
        summarize_memories(
            &memories
        );

    println!(
        "\n{}",
        summary
    );

    println!(
        "[PANOPTES] Telemetry online"
    );

    println!(
        "[RUNTIME] System operational\n"
    );
}
