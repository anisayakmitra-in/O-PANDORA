use clap::{
    Parser,
    Subcommand,
};

use anubis_memory::{
    load_memories,
    summarize_memories,
    search_memories,
    synthesize_reflection,
    build_graph,
    export_memories,
    import_memories,
};

#[derive(Parser)]
#[command(
    author = "Pandora Systems",
    version = "0.1.0",
    about = "Sovereign cognition CLI"
)]
struct Cli {

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {

    Status,

    Memory,

    Search {
        query: String,
    },

    Reflect,

    Graph,

    Export,

    Import,
}

fn main() {

    let cli = Cli::parse();

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
                "SEARCH RESULTS\n"
            );

            for (
                weight,
                memory,
            ) in results {

                println!(
                    "WEIGHT: {}\n",
                    weight
                );

                println!(
                    "MEMORY: {}\n",
                    memory.id
                );

                println!(
                    "PROMPT:\n{}\n",
                    memory.prompt
                );
            }
        }

        Commands::Reflect => {

            let memories =
                load_memories();

            let reflection =
                synthesize_reflection(
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
                "GRAPH NODES: {}\n",
                graph.len()
            );

            for (
                node,
                edges,
            ) in graph {

                println!(
                    "{} -> {:?}",
                    node,
                    edges
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
    }
}
