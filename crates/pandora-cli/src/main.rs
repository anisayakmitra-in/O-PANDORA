use clap::{
    Parser,
    Subcommand,
};

use anubis_memory::{
    load_memories,
    summarize_memories,
    search_memories,
    reflect,
    build_graph,
};

#[derive(Parser)]
#[command(
    name = "pandora",
    version = "0.1",
    about = "PANDORA SYSTEMS CLI"
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

    Search {

        query:
            String,
    },

    Reflect,

    Graph,
}

fn main() {

    let cli =
        Cli::parse();

    match cli.command {

        Commands::Status => {

            println!(
                "\
PANDORA SYSTEMS

STATUS: OPERATIONAL

ANUBIS: ONLINE
RAHU/KETU: ACTIVE
EVENT BUS: ACTIVE
"
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
                "{}",
                summary
            );
        }

        Commands::Search {
            query
        } => {

            let memories =
                load_memories();

            let results =
                search_memories(
                    &memories,
                    &query,
                );

            println!(
                "\
SEARCH RESULTS
"
            );

            for (
                weight,
                memory
            ) in results {

                println!(
                    "\
WEIGHT: {}

MEMORY: {}

PROMPT:
{}

",
                    weight,
                    memory.id,
                    memory.prompt,
                );
            }
        }

        Commands::Reflect => {

            let memories =
                load_memories();

            let reflection =
                reflect(
                    &memories
                );

            println!(
                "{}",
                reflection
            );
        }

        Commands::Graph => {

            let memories =
                load_memories();

            let graph =
                build_graph(
                    &memories
                );

            println!(
                "\
GRAPH NODES: {}
",
                graph.len()
            );

            for (
                node,
                edges
            ) in graph {

                println!(
                    "{} -> {:?}",
                    node,
                    edges,
                );
            }
        }
    }
}
