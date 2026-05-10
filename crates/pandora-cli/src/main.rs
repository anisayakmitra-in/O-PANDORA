use clap::{
    Parser,
    Subcommand,
};

#[path = "../../pandora-runtime/src/gene.rs"]
mod gene;

use gene::load_genes;

#[derive(Parser)]
#[command(
    author = "Pandora Systems",
    version = "0.1.0",
    about = "Pandora CLI"
)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Reflect,

    Export,

    Import,

    Genes,

    Verify,

    Lineage,

    Install {
        gene: String,
    },

    Remove {
        gene: String,
    },
}

fn main() {

    let cli =
        Cli::parse();

    match cli.command {

        Commands::Reflect => {

            println!(
                "ANUBIS SYNTHESIS\n"
            );
        }

        Commands::Export => {

            println!(
                "ANUBIS EXPORT COMPLETE"
            );
        }

        Commands::Import => {

            println!(
                "ANUBIS IMPORT COMPLETE"
            );
        }

        Commands::Genes => {

            println!(
                "PANDORA GENE REGISTRY\n"
            );

            let genes =
                load_genes();

            for gene in genes {

                println!(
                    "{}",
                    gene.name
                );
            }
        }

        Commands::Verify => {

            println!(
                "PANDORA GENE VERIFICATION\n"
            );

            let genes =
                load_genes();

            for gene in genes {

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
            }
        }

        Commands::Lineage => {

            println!(
                "PANDORA GENE LINEAGE\n"
            );

            let genes =
                load_genes();

            for gene in genes {

                println!(
                    "GENE: {}",
                    gene.name
                );

                println!(
                    "NAMESPACE: {}",
                    gene.namespace
                );

                println!(
                    "TYPE: {}",
                    gene.gene_type
                );

                println!(
                    "GENERATION: {}",
                    gene.lineage.generation
                );

                println!(
                    "PARENT: {}",
                    gene.lineage.parent
                );

                println!(
                    "MUTATION: {}",
                    gene.lineage.mutation
                );

                println!(
                    "SIGNATURE: {:?}\n",
                    gene.signature
                );
            }
        }

        Commands::Install { gene } => {

            let source =
                format!(
                    "genes/{}.json",
                    gene
                );

            let destination =
                format!(
                    "genes/installed/{}.json",
                    gene
                );

            match std::fs::copy(
                &source,
                &destination,
            ) {

                Ok(_) => {

                    println!(
                        "INSTALLED GENE: {}",
                        gene
                    );
                }

                Err(error) => {

                    println!(
                        "INSTALL FAILED: {}",
                        error
                    );
                }
            }
        }

        Commands::Remove { gene } => {

            let target =
                format!(
                    "genes/installed/{}.json",
                    gene
                );

            match std::fs::remove_file(
                &target,
            ) {

                Ok(_) => {

                    println!(
                        "REMOVED GENE: {}",
                        gene
                    );
                }

                Err(error) => {

                    println!(
                        "REMOVE FAILED: {}",
                        error
                    );
                }
            }
        }
    }
}
