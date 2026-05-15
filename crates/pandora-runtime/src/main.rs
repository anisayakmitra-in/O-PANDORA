use std::fs;

pub mod permission;
mod abi;
mod plugin;
mod dependency;
mod contract;
mod manifest;
mod loader;
mod negotiation;
mod delegation;
mod evolution;
mod gene;
mod harness;
mod panoptes;
mod trace;
mod capability;
mod registry;
mod sandbox;
mod runtime;
mod task;
mod scheduler;
mod provider;
mod config;

use evolution::{
    EvolutionCandidate,
    evaluate_candidate,
    generate_candidates,
    promote_winner,
    select_winner,
};

use gene::{
    load_genes,
};

use harness::MetaHarness;

use panoptes::PanoptesHarness;

use trace::RuntimeTrace;

use capability::{
    CapabilityDecision,
    CapabilityRequest,
};

use registry::{
    capability_registry,
};

use sandbox::{
    determine_sandbox,
    SandboxLevel,
};

use runtime::RuntimeState;

use task::{
    RuntimeTask,
    TaskStatus,
};

use scheduler::{
    runtime_heartbeat,
    schedule_task,
};

use provider::Provider;

use provider::ollama::OllamaProvider;

use config::RuntimeConfig;

use pandora_tools::filesystem::read_file;

fn main() {

    println!(
        "\nPANDORA SYSTEMS\n"
    );

    let genes =
        load_genes();

    let runtime_state =
        RuntimeState::new(
            genes.clone()
        );

    let harness =
        PanoptesHarness;

    let config =
        RuntimeConfig::load();

    if genes.is_empty() {

        println!(
            "NO GENES INSTALLED"
        );

        return;
    }

    let shell_request =
    CapabilityRequest {

        capability:
            String::from(
                "shell.execute"
            ),

        requester:
            genes[0]
                .name
                .clone(),

        target:
            String::from(
                "localhost"
            ),

        reason:
            String::from(
                "Tool execution"
            ),
    };

let shell_decision =
    harness.authorize(
        &genes[0],
        &shell_request,
    );

if config.allow_shell {

    println!(
        "[AUTHORIZED: shell]"
    );

} else {

    match shell_decision {

        CapabilityDecision::Approved => {

            println!(
                "[AUTHORIZED: shell]"
            );
        }

        CapabilityDecision::Denied => {

            println!(
                "[DENIED: shell]"
            );

            println!(
                "Shell access denied under PANOPTES policy"
            );
       }

       CapabilityDecision::Escalated => {

           println!(
               "[ESCALATED: shell]"
           );
       }

           }
       }

println!(
    "LOADED GENES: {}\n",
    genes.len()
);

println!(
    "[RUNTIME STATUS] {}",
    runtime_state
        .runtime_status
);

println!(
    "[ACTIVE HARNESS] {}",
    runtime_state
        .active_harness
        .clone()
        .unwrap()
);

println!(
    "[ACTIVE PROVIDER] {}\n",
    runtime_state
        .active_provider
        .clone()
        .unwrap()
);

println!(
    "CAPABILITY REGISTRY\n"
);

let capabilities =
    capability_registry();

for capability
    in &capabilities
{

    println!(
        "[CAPABILITY] {}",
        capability.name
    );

    println!(
        "TRUST LEVEL: {}",
        capability.trust_level
    );

    println!(
        "SANDBOX: {}",
        capability.requires_sandbox
    );

    println!(
        "ESCALATION: {}\n",
        capability.requires_escalation
    );
}

let sandbox_level =
    determine_sandbox(
        &shell_request
    );

match sandbox_level {

    SandboxLevel::None => {

        println!(
            "[SANDBOX] none"
        );
    }

    SandboxLevel::Restricted => {

        println!(
            "[SANDBOX] restricted"
        );
    }

    SandboxLevel::Isolated => {

        println!(
            "[SANDBOX] isolated"
        );
    }
}     
    
    for gene in &genes {

        if gene.gene_type == "provider" {

            println!(
                "[PROVIDER] {}",
                gene.name
            );
        }

        println!(
            "[VALID {}] {} {}",
            gene.gene_type,
            gene.namespace,
            gene.name
        );
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
let inference_task =
    RuntimeTask {

        id:
            String::from(
                "task-001"
            ),

        task_type:
            String::from(
                "inference"
            ),

        target:
            String::from(
                "ollama"
            ),

        status:
            TaskStatus::Running,
    };

schedule_task(
    inference_task.clone()
);

println!(
    "[TASK CREATED] {}",
    inference_task.id
);

println!(
    "[TASK TYPE] {}",
    inference_task.task_type
);

println!(
    "[TASK TARGET] {}\n",
    inference_task.target
);
    
    let prompt =
        "Explain Rust ownership";

    println!(
        "PROMPT:\n{}\n",
        prompt
    );

    println!(
        "TOOL EXECUTION\n"
    );

    println!(
        "[TOOL: read_file]"
    );

    match read_file(
        "Cargo.toml"
    ) {

        Ok(content) => {

            println!(
                "FILE CONTENT:\n{}\n",
                content
            );
       }

       Err(error) => {

           println!(
               "FILE READ ERROR:\n{}\n",
               error
           );
       }
    }

    println!(
        "[TOOL: web_scrape]"
    );

    println!(
        "SCRAPLING RESEARCH TOOL:\nScraped data from https://example.com\n"
    );

    let trace =
        RuntimeTrace {

            session_id:
                String::from(
                    "session-alpha"
                ),

            gene:
                String::from(
                    "coding-v1"
                ),

            provider:
                String::from(
                    "ollama"
                ),

            prompt:
                prompt.to_string(),

            approved_tools:
                vec![
                    String::from(
                        "read_file"
                    ),
                    String::from(
                        "web_scrape"
                    ),
                ],

            denied_tools:
                vec![
                    String::from(
                        "shell"
                    ),
                ],

            memory_hits: 0,

            success: true,
        };

    fs::create_dir_all(
        "traces"
    )
    .unwrap();

    fs::write(
        "traces/session-alpha.json",
        serde_json::to_string_pretty(
            &trace
        )
        .unwrap(),
    )
    .unwrap();

    println!(
        "[ANUBIS] Trace persisted: traces/session-alpha.json\n"
    );

    println!(
        "EVOLUTIONARY PIPELINE\n"
    );

    let mut evolving_gene =
        genes[0].clone();

    let candidates:
        Vec<EvolutionCandidate> =
            generate_candidates(
                &evolving_gene,
                5,
            );

    let mut evaluated:
        Vec<EvolutionCandidate> =
            Vec::new();

    for mut candidate in candidates {

        evaluate_candidate(
            &mut candidate
        );

        evaluated.push(
            candidate
        );
    }

    if let Some(winner) =
       select_winner(
           &evaluated
       )
   {

       promote_winner(
           &mut evolving_gene,
           &winner,
       );

   } else {

       println!(
           "[WARN] no evolution candidates available"
       );
   }

    println!(
        "PROMOTED GENE:\n{}",
        evolving_gene.name
    );

    println!(
        "GENERATION: {}",
        evolving_gene
            .lineage
            .generation
    );

    println!(
        "FITNESS: {}",
        evolving_gene
            .lineage
            .fitness
    );

    let active_tasks =
        vec![
            inference_task
                 .clone()
        ];

    let heartbeat =
        runtime_heartbeat(
            &active_tasks
        );

    println!(
        "\nRUNTIME HEARTBEAT\n"
    );

    println!(
        "CYCLE: {}",
        heartbeat.cycle
    );

    println!(
        "ACTIVE TASKS: {}",
        heartbeat.active_tasks
    );

    println!(
        "STATUS: {}\n",
        heartbeat.runtime_status
    );  

    let provider =
        OllamaProvider;

    let response =
        provider.infer(
            "qwen2.5-coder:7b",
            prompt,
        );

    println!(
        "RESPONSE:\n{}",
        response
    );

    }



