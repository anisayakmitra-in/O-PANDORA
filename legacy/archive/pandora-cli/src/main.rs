use clap::{Parser, Subcommand};

#[path = "../../pandora-runtime/src/gene.rs"]
mod gene;

use gene::load_genes;

mod pipeline;

#[derive(Parser)]
#[command(author = "Pandora Systems", version = "0.1.0", about = "Pandora CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Tui {
        #[arg(short, long, default_value = "dashboard")]
        view: String,
        #[arg(short = 'C', long)]
        no_cat: bool,
    },

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

    /// Submit a user request to NARAD. The output is a
    /// structured PlanningContext that downstream stages
    /// (MOIRA, RAHU, the source harnesses) consume.
    Ask {
        input: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tui { view, no_cat } => {
            let self_path = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.join("pandora-tui")))
                .and_then(|p| if p.exists() { Some(p) } else { None });
            match self_path {
                Some(binary) => {
                    let mut cmd = std::process::Command::new(binary);
                    cmd.arg("--view").arg(view);
                    if no_cat {
                        cmd.arg("--no-cat");
                    }
                    let status = cmd.status().expect("failed to launch pandora-tui");
                    std::process::exit(status.code().unwrap_or(0));
                }
                None => {
                    eprintln!(
                        "pandora-tui binary not found. Build it with: cargo build -p pandora-tui"
                    );
                    std::process::exit(1);
                }
            }
        }

        Commands::Reflect => {
            println!("ANUBIS SYNTHESIS\n");
        }

        Commands::Export => {
            println!("ANUBIS EXPORT COMPLETE");
        }

        Commands::Import => {
            println!("ANUBIS IMPORT COMPLETE");
        }

        Commands::Genes => {
            println!("PANDORA GENE REGISTRY\n");

            let genes = load_genes();

            for gene in genes {
                println!("{}", gene.name);
            }
        }

        Commands::Verify => {
            println!("PANDORA GENE VERIFICATION\n");

            let genes = load_genes();

            for gene in genes {
                println!("[VALID] {} {}", gene.namespace, gene.name);

                println!("SCHEMA: {}", gene.schema_version);

                println!("GENERATION: {}", gene.lineage.generation);

                println!("SIGNATURE: {}", gene.signature.algorithm);
            }
        }

        Commands::Lineage => {
            println!("PANDORA GENE LINEAGE\n");

            let genes = load_genes();

            for gene in genes {
                println!("GENE: {}", gene.name);

                println!("NAMESPACE: {}", gene.namespace);

                println!("TYPE: {}", gene.gene_type);

                println!("GENERATION: {}", gene.lineage.generation);

                println!("PARENT: {}", gene.lineage.parent);

                println!("MUTATION: {}", gene.lineage.mutation);

                println!("SIGNATURE: {:?}\n", gene.signature);
            }
        }

        Commands::Install { gene } => {
            let source = format!("genes/{}.json", gene);

            let destination = format!("genes/installed/{}.json", gene);

            match std::fs::copy(&source, &destination) {
                Ok(_) => {
                    println!("INSTALLED GENE: {}", gene);
                }

                Err(error) => {
                    println!("INSTALL FAILED: {}", error);
                }
            }
        }

        Commands::Remove { gene } => {
            let target = format!("genes/installed/{}.json", gene);

            match std::fs::remove_file(&target) {
                Ok(_) => {
                    println!("REMOVED GENE: {}", gene);
                }

                Err(error) => {
                    println!("REMOVE FAILED: {}", error);
                }
            }
        }

        Commands::Ask { input } => {
            // NARAD -> LoopRegistry: end-to-end pipeline.
            // NARAD extracts the intent, the registry
            // resolves a loop and runs it, and the
            // combined result is emitted as JSON.
            let result = futures::executor::block_on(pipeline::run_pipeline(&input));
            let json =
                serde_json::to_string_pretty(&result).expect("PipelineResult is serializable");
            println!("{}", json);
        }
    }
}
