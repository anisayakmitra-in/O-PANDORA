use chrono::Utc;

use anubis_memory::{
    load_memories,
    search_memories,
    summarize_memories,
    store_memory,
    restore_context,
    export_memories,
    import_memories,
    MemoryRecord,
};

fn main() {

    println!(
        "\nPANDORA SYSTEMS\n"
    );

    let prompt =
        "Explain Rust ownership";

    let response =
        "Simulated Ollama inference response."
            .to_string();

    println!(
        "\nMODEL INFERENCE\n"
    );

    println!(
        "OLLAMA PROVIDER\n"
    );

    println!(
        "MODEL: qwen2.5-coder:7b\n"
    );

    println!(
        "PROMPT:\n{}\n",
        prompt
    );

    println!(
        "RESPONSE:\n{}\n",
        response
    );

    println!(
        "TOOL EXECUTION\n"
    );

    println!(
        "[TOOL: read_file]"
    );

    println!(
        "READ FILE TOOL:\nhttps://example.com\n"
    );

    println!(
        "[TOOL: web_scrape]"
    );

    println!(
        "SCRAPLING RESEARCH TOOL:\nScraped data from https://example.com\n"
    );

    println!(
        "[DENIED: shell]"
    );

    println!(
        "Shell access denied under T2 policy\n"
    );

    let memories =
        load_memories();

    let restored_context =
        restore_context(
            &memories,
            5,
        );

    println!(
        "\nRESTORED CONTEXT\n"
    );

    for memory in
        &restored_context
    {

        println!(
            "[{:?}] {}",
            memory.memory_type,
            memory.prompt
        );
    }

    let memory =
        MemoryRecord {

            id:
                format!(
                    "memory-{}",
                    memories.len() + 1
                ),

            session_id:
                "session-alpha"
                    .to_string(),

            timestamp:
                Utc::now()
                    .to_rfc3339(),

            gene:
                "coding-v1"
                    .to_string(),

            harness:
                "coding"
                    .to_string(),

            model:
                "qwen2.5-coder:7b"
                    .to_string(),

            prompt:
                prompt.to_string(),

            response:
                response.clone(),

            memory_layer:
                "long_term"
                    .to_string(),

            related_memories:
                vec![
                    "memory-0"
                        .to_string()
                ],

            embedding:
                vec![
                    0.1,
                    0.2,
                    0.3,
                ],

            salience:
                1.0,

            tags:
                vec![
                    "rust"
                        .to_string(),

                    "ownership"
                        .to_string(),

                    "coding"
                        .to_string(),
                ],

            memory_type:
                anubis_memory::MemoryType::Semantic,
        };

    store_memory(
        &memory
    );

    let loaded_memories =
        load_memories();

    let results =
        search_memories(
            &loaded_memories,
            "Rust",
        );

    export_memories(
        &loaded_memories
    );

   let imported_memories =
       import_memories();

   println!(
           "\nIMPORTED MEMORIES: {}\n",
           imported_memories.len()
   );

    println!(
            "\nSEARCH RESULTS\n"
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

    println!(
        "\nGRAPH INDEX SIZE: {}\n",
        loaded_memories.len()
    );

    println!(
        "TEMPORAL MEMORIES: {}\n",
        loaded_memories.len()
    );

    println!(
        "\nEVENT STREAM\n"
    );

    println!(
        "[BOOT] Initializing runtime"
    );

    println!(
        "[GENE] 3 genes loaded"
    );

    println!(
        "[RUNTIME] Active Gene: coding-v1"
    );

    println!(
        "[HARNESS] Harness selected: coding"
    );

    println!(
        "[RUNTIME] Provider: ollama"
    );

    println!(
        "[RUNTIME] Model routed: qwen2.5-coder:7b"
    );

    println!(
        "[RUNTIME] 3 tools loaded"
    );

    println!(
        "[RUNTIME] RAHU proposed: read_file"
    );

    println!(
        "[RUNTIME] KETU approved: read_file"
    );

    println!(
        "[RUNTIME] RAHU proposed: web_scrape"
    );

    println!(
        "[RUNTIME] KETU approved: web_scrape"
    );

    println!(
        "[RUNTIME] RAHU proposed: shell"
    );

    println!(
        "[RUNTIME] KETU denied: shell"
    );

    println!(
        "[ANUBIS] Loaded {} memories",
        loaded_memories.len()
    );

    println!(
        "[PANOPTES] {} relevant memories",
        loaded_memories.len()
    );

    println!(
        "[RUNTIME] System operational"
    );

    println!(
        "\nHARNESS OUTPUT\n"
    );

    println!(
        "CODING HARNESS EXECUTED:\n{}",
        prompt
    );

    let summary =
        summarize_memories(
            &loaded_memories
        );

    println!(
        "\n{}\n",
        summary
    );
}
