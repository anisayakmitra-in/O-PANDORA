mod evolution;

use evolution::{
    generate_candidates,
    evaluate_candidate,
};

mod ingest;

use ingest::ingest_traces;

use std::fs;

use serde_json;

mod trace;

use trace::RuntimeTrace;

mod gene;

use gene::{
    load_genes,
    validate_gene,
};

use anubis_memory::{
    load_memories,
    search_memories,
    store_memory,
    summarize_memories,
    restore_context,
    export_memories,
    import_memories,
    MemoryRecord,
    MemoryType,
};

fn main() {

    println!(
        "\nPANDORA SYSTEMS\n"
    );

    let loaded_genes =
        load_genes();
    
    let genes =
        load_genes(); 

    println!(
        "LOADED GENES: {}",
        genes.len()
    );

    println!();

    for gene in &genes {

        if gene.gene_type == "provider" {

            println!(
                "[PROVIDER] {}",
                gene.name
         );
     }
 }

    for gene in
        &loaded_genes
    {

        let valid =
            validate_gene(
                gene
            );

        if valid {

            println!(
                "[VALID {}] {} {}",
                gene.gene_type,
                gene.namespace,
                gene.name
            );

        } else {

            println!(
                "[INVALID] {} {}",
                gene.namespace,
                gene.name
            );
        }
    }

    println!(
        "\nMODEL INFERENCE\n"
    );

    println!(
        "OLLAMA PROVIDER\n"
    );

    println!(
        "MODEL: qwen2.5-coder:7b\n"
    );

    let prompt =
        "Explain Rust ownership";

    println!(
        "PROMPT:\n{}\n",
        prompt
    );

    println!(
        "RESPONSE:\nSimulated Ollama inference response.\n"
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

    let memory =
        MemoryRecord {

            id:
                "memory-1".to_string(),

            prompt:
                prompt.to_string(),

            response:
                "Simulated Ollama inference response."
                    .to_string(),

            embedding:
                vec![
                    0.1,
                    0.2,
                    0.3,
                ],

            timestamp:
                "2026-05-09T00:00:00Z"
                    .to_string(),

            memory_type:
                MemoryType::Semantic,

            session_id:
                "session-alpha"
                    .to_string(),

            gene:
                "coding-v1"
                    .to_string(),

            harness:
                "coding"
                    .to_string(),

            model:
                "qwen2.5-coder:7b"
                    .to_string(),

            memory_layer:
                "long_term"
                    .to_string(),

            related_memories:
                vec![
                    "memory-0".to_string()
                ],
            
            salience:
                0.95,
               
            tags:
                vec![
                    "rust".to_string(),
                    "ownership".to_string(),
                    "coding".to_string(),
                ],
        };

    store_memory(
        &memory
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

    export_memories(
        &memories
    );

    let imported =
        import_memories();

    println!(
        "\nIMPORTED MEMORIES: {}\n",
        imported.len()
    );

    let results =
        search_memories(
            &memories,
            prompt,
        );

    println!(
        "\nSEARCH RESULTS\n"
    );

    for result in
        &results
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

    println!(
        "\nGRAPH INDEX SIZE: {}\n",
        memories.len()
    );

    println!(
        "TEMPORAL MEMORIES: {}\n",
        memories.len()
    );

    println!(
        "\nEVENT STREAM\n"
    );

    println!(
        "[BOOT] Initializing runtime"
    );

    println!(
        "[GENE] {} genes loaded",
        loaded_genes.len()
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
        memories.len()
    );

    println!(
        "[PANOPTES] {} relevant memories",
        results.len()
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
            &memories
        );

    println!(
        "\nANUBIS MEMORY SUMMARY\n"
    );

    println!(
        "{}",
        summary
    );

    let trace =
    RuntimeTrace {

        session_id:
            "session-alpha"
            .to_string(),

        gene:
            "coding-v1"
            .to_string(),

        provider:
            "ollama"
            .to_string(),

        prompt:
            prompt.to_string(),

        approved_tools:
            vec![
                "read_file".to_string(),
                "web_scrape".to_string(),
            ],

        denied_tools:
            vec![
                "shell".to_string(),
            ],

        memory_hits:
            results.len(),

        success: true,
    };

    println!();

    println!(
        "ANUBIS TRACE\n"
    );

    println!(
        "{:#?}",
        trace
    );

    let trace_json =
        serde_json::to_string_pretty(
            &trace
        )
        .unwrap();

    let trace_path =
        format!(
            "traces/{}.json",
            trace.session_id
        );

    fs::write(
        &trace_path,
        trace_json,
    )
    .unwrap();

    println!();

    println!(
        "[ANUBIS] Trace persisted: {}",
        trace_path
    ); 
    
    let traces =
        ingest_traces();

    println!();

    println!(
        "ANUBIS TRACE INGESTION\n"
    );

    println!(
        "TOTAL TRACES: {}",
        traces.len()
    );

    let mut total_score =
        0.0;

    for trace in &traces {

        let mut score =
            0.0;

        if trace.success {

           score += 5.0;
        }

        score +=
            trace.approved_tools.len()
            as f32;

        score -=
           trace.denied_tools.len()
           as f32;

        score +=
            trace.memory_hits
            as f32 * 0.1;

        total_score += score;
    }

    println!(
        "EVOLUTIONARY FITNESS: {}",
        total_score
    );

    let mut evolving_gene =
        genes[0].clone();

    let candidates =
        generate_candidates(
            &genes[0],
            5,
        );

    println!();

    println!(
        "GEPA CANDIDATES\n"
    );

    for mut candidate in candidates {

        evaluate_candidate(
            &mut candidate
    );

    println!(
        "{:#?}",
        candidate
    );
}

    println!();

    println!(
        "GEPA EVOLUTION\n"
    );

    println!(
        "GENERATION: {}",
        evolving_gene.lineage.generation
    );

    println!(
        "MUTATION: {}",
        evolving_gene.lineage.mutation
    );

    println!(
        "INSTRUCTIONS:\n{}",
        evolving_gene.instructions
    );

}
    









