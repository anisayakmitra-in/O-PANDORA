use chrono::Utc;

use pandora_gene::load_genes;

use pandora_harness::{
    CodingHarness,
    Harness,
    ResearchHarness,
};

use pandora_events::{
    EventBus,
    RuntimeEvent,
};

use anubis_memory::{
    load_memories,
    search_memories,
    store_memory,
    summarize_memories,
    MemoryRecord,
};

use pandora_tools::{
    tool_registry,
};

use pandora_provider::{
    model_for_harness,
    OllamaProvider,
    Provider,
};

use pandora_governance::{
    GovernanceDecision,
    Ketu,
    PermissionTier,
    Rahu,
};

fn emit(
    bus: &EventBus,
    event: RuntimeEvent,
) {

    bus.sender
        .send(event)
        .unwrap();
}

fn main() {

    println!(
        "\nPANDORA SYSTEMS\n"
    );

    let bus =
        EventBus::new();

    emit(
        &bus,
        RuntimeEvent::Boot(
            "Initializing runtime"
                .to_string()
        )
    );

    let genes =
        load_genes("genes");

    emit(
        &bus,
        RuntimeEvent::GeneLoaded(
            format!(
                "{} genes loaded",
                genes.len()
            )
        )
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

    emit(
        &bus,
        RuntimeEvent::Runtime(
            format!(
                "Active Gene: {}",
                best_gene.gene_id
            )
        )
    );

    let harness_name =
        "coding";

    let harness:
        Box<dyn Harness> =
        match harness_name {

            "coding" => {
                Box::new(
                    CodingHarness
                )
            }

            "research" => {
                Box::new(
                    ResearchHarness
                )
            }

            _ => {
                Box::new(
                    CodingHarness
                )
            }
        };

    emit(
        &bus,
        RuntimeEvent::Harness(
            format!(
                "Harness selected: {}",
                harness.name()
            )
        )
    );

    let task =
        "Explain Rust ownership";

    let model =
        model_for_harness(
            harness.name()
        );

    let provider:
        Box<dyn Provider> =
        Box::new(
            OllamaProvider
        );

   emit(
       &bus,
       RuntimeEvent::Runtime(
           format!(
               "Provider: {}",
               provider.name()
           )
       )
   );

   emit(
       &bus,
       RuntimeEvent::Runtime(
           format!(
               "Model routed: {}",
               model
           )
       )
   );

   let inference =
       provider.infer(
           &model,
           task
       );

    println!(
        "\nMODEL INFERENCE\n"
    );

    println!(
        "{}",
        inference
    );

    let output =
        harness.execute(task);

    let tools =
        tool_registry();

    emit(
        &bus,
        RuntimeEvent::Runtime(
            format!(
                "{} tools loaded",
                tools.len()
            )
        )
    );

    println!(
        "\nTOOL EXECUTION\n"
    );

   for tool in tools {

    let tier =
        match tool.name() {

            "shell" => {
                PermissionTier::T2
            }

            _ => {
                PermissionTier::T1
            }
        };

    let request =
        Rahu::propose(
            harness.name(),
            tool.name(),
            tier,
        );

    emit(
        &bus,
        RuntimeEvent::Runtime(
            format!(
                "RAHU proposed: {}",
                tool.name()
            )
        )
    );

    match Ketu::validate(
        &request
    ) {

        GovernanceDecision::Approved => {

            emit(
                &bus,
                RuntimeEvent::Runtime(
                    format!(
                        "KETU approved: {}",
                        tool.name()
                    )
                )
            );

            let result =
                tool.execute(
                    "https://example.com"
                );

            println!(
                "[TOOL: {}]\n{}\n",
                tool.name(),
                result
            );
        }

        GovernanceDecision::Denied(reason) => {

            emit(
                &bus,
                RuntimeEvent::Runtime(
                    format!(
                        "KETU denied: {}",
                        tool.name()
                    )
                )
            );

            println!(
                "[DENIED: {}]\n{}\n",
                tool.name(),
                reason
            );
        }
    }
}

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
                harness
                    .name()
                    .to_string(),

            model:
                model.clone(),
                    

            prompt:
                task
                    .to_string(),

            response:
                output.clone(),

            score:
                best_gene
                    .avg_score,
        };

    store_memory(
        &memory
    );

    emit(
        &bus,
        RuntimeEvent::MemoryStored(
            "Runtime memory stored"
                .to_string()
        )
    );

    let memories =
        load_memories();

    emit(
        &bus,
        RuntimeEvent::MemoryRetrieved(
            memories.len()
        )
    );

    let results =
        search_memories(
            &memories,
            "Rust"
        );

    emit(
        &bus,
        RuntimeEvent::Telemetry(
            format!(
                "{} relevant memories",
                results.len()
            )
        )
    );

    let summary =
        summarize_memories(
            &memories
        );

    emit(
        &bus,
        RuntimeEvent::Runtime(
            "System operational"
                .to_string()
        )
    );

    println!(
        "\nEVENT STREAM\n"
    );

    while let Ok(event) =
        bus.receiver.try_recv() {

        match event {

            RuntimeEvent::Boot(msg) => {
                println!(
                    "[BOOT] {}",
                    msg
                );
            }

            RuntimeEvent::GeneLoaded(msg) => {
                println!(
                    "[GENE] {}",
                    msg
                );
            }

            RuntimeEvent::MemoryStored(msg) => {
                println!(
                    "[ANUBIS] {}",
                    msg
                );
            }

            RuntimeEvent::MemoryRetrieved(count) => {
                println!(
                    "[ANUBIS] Loaded {} memories",
                    count
                );
            }

            RuntimeEvent::Telemetry(msg) => {
                println!(
                    "[PANOPTES] {}",
                    msg
                );
            }

            RuntimeEvent::Harness(msg) => {
                println!(
                    "[HARNESS] {}",
                    msg
                );
            }

            RuntimeEvent::Mutation(msg) => {
                println!(
                    "[MUTATION] {}",
                    msg
                );
            }

            RuntimeEvent::Runtime(msg) => {
                println!(
                    "[RUNTIME] {}",
                    msg
                );
            }
        }
    }

    println!(
        "\nHARNESS OUTPUT\n"
    );

    println!(
        "{}",
        output
    );

    println!(
        "\n{}",
        summary
    );
}
