//! Full pipeline example — uses PandoraRuntime::run() with all 9 stages.
//! Run: cargo run --example pandora_run_full --features ollama -- "your task"

use pandora_orchestrator::PandoraRuntime;

#[tokio::main]
async fn main() {
    let task = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "write hello world in rust".to_string());

    let domain = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "coding".to_string());

    println!("=== Pandora Full Pipeline ===\n");
    println!("  Task:   {task}");
    println!("  Domain: {domain}\n");

    let mut runtime = PandoraRuntime::new();

    match runtime.run(&task, &domain).await {
        Ok(report) => {
            println!("  Execution ID: {}", report.execution_id);
            println!("  Provider:     {}/{}", report.provider, report.model);
            println!("  Duration:     {} ms (pipeline)", report.duration_ms);
            println!("  Workflow:     {} steps", report.workflow_steps);
            println!("  Workflow complete.\n");
            println!("  Recorder:     replay_id={}", report.replay_id);
            println!("  Telemetry:    {} spans", report.telemetry_spans);
            println!("  Intel:        {} root causes", report.root_causes_found);
            println!("  Knowledge:    {} nodes", report.knowledge_nodes);
            println!("  Ledger:       {} entries\n", report.ledger_entries);
            println!("  Success:      {}\n", report.success);

            // Print first 500 chars of output
            let preview: String = report.output.chars().take(500).collect();
            println!("=== Output (first 500 chars) ===");
            println!("{preview}");
            if report.output.len() > 500 {
                println!("... ({} more chars)", report.output.len() - 500);
            }
            println!("=== End ===\n");

            if report.success {
                println!("✓ Pipeline completed successfully");
            } else {
                println!("✗ Pipeline completed with empty response");
            }
        }
        Err(e) => {
            eprintln!("\nPipeline failed: {e}");
            eprintln!("Is Ollama running at http://localhost:11434?");
        }
    }
}
