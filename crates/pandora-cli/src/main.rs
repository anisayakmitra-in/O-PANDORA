use clap::{
    Parser,
    Subcommand,
};

use std::fs;

#[path = "../../pandora-runtime/src/gene.rs"]
mod gene;

use gene::{
    load_genes,
    validate_gene,
};

use anubis_memory::{
    load_memories,
    search_memories,
    summarize_memories,
    export_memories,
    import_memories,
};

#[derive(Parser)]
#[command(
    author = "Pandora Systems",
    version = "0.1.0",
    about = "Pandora CLI"
)]
struct Cli {

    #[command(subcommand)]
    command:
        Commands,
}

#[derive(Subcommand)]
enum Commands {

    Status,

    Memory,

    Search,

    Export,

    Import,

    Genes,

    Verify,
}

fn main() {

    let cli =
        Cli::parse();

    match cli.command {

        Commands::Status => {

            println!(
                "PANDORA SYSTEMS\n"
            );

            println!(
                "STATUS: OPERATIONAL\n"
            );

            println!(
                "ANUBIS: ONLINE"
            );

            println!(
                "RAHU/KETU: ACTIVE"
            );

            println!(
                "EVENT BUS: ACTIVE"
            );
        }

        Commands::Memory => {

            let memories =
                load_memories();

            let summary =
                summarize_memories(
                    &memories
                );

            println!(
                "ANUBIS MEMORY SUMMARY\n"
            );

            println!(
                "{}",
                summary
            );
        }

        Commands::Search => {

            let memories =
                load_memories();

            let results =
                search_memories(
                    &memories,
                    "Rust",
                );

            println!(
                "ANUBIS SEARCH RESULTS\n"
            );

            for result in
                results
            {

                println!(
                    "WEIGHT: {}\n",
                    result.0
                );

                println!(
                    "MEMORY: {}\n",
                    result.1.id
                );

                println!(
                    "PROMPT:\n{}\n",
                    result.1.prompt
                );
            }
        }

        Commands::Export => {

            let memories =
                load_memories();

            export_memories(
                &memories
            );

            println!(
                "ANUBIS EXPORT COMPLETE"
            );
        }

        Commands::Import => {

            let memories =
                import_memories();

            println!(
                "IMPORTED {} MEMORIES",
                memories.len()
            );
        }

        Commands::Genes => {

            println!(
                "PANDORA GENE REGISTRY\n"
            );

            let paths =
                fs::read_dir(
                    "genes"
                )
                .unwrap();

            for path in
                paths
            {

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

                    println!(
                        "{}",
                        file_path.display()
                    );
                }
            }
        }

        Commands::Verify => {

            println!(
                "PANDORA GENE VERIFICATION\n"
            );

            let genes =
                load_genes();

            for gene in
                &genes
            {

                let valid =
                    validate_gene(
                        gene
                    );

                if valid {

                    println!(
                        "[VALID] {} {}",
                        gene.namespace,
                        gene.name
                    );

                    println!(
                        "SCHEMA: {}",
                        gene.schema_version
                    );

                    println!(
                        "GENERATION: {}",
                        gene.lineage.generation
                    );

                    println!(
                        "SIGNATURE: {}",
                        gene.signature.algorithm
                    );

                    println!(
                        "TRUSTED: {}\n",
                        gene.tier.trusted
                    );

                } else {

                    println!(
                        "[INVALID] {} {}\n",
                        gene.namespace,
                        gene.name
                    );
                }
            }
        }
    }
}
    
