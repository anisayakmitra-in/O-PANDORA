use pandora_ledger::{ExecutionLedger, LedgerEntry, LedgerOutcome};
use pandora_provider::traits::Provider;
use pandora_provider::types::GenerationRequest;
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let task = std::env::args().nth(1).unwrap_or_else(|| "write hello world in rust".to_string());
    println!("=== Pandora Run ===");
    println!("Task: {}\n", task);

    let provider = pandora_provider::ollama::OllamaProvider::new_default();
    println!("Provider: {} ({})", provider.name(), "http://localhost:11434");

    let request = GenerationRequest {
        model: "qwen2.5-coder:7b".into(),
        prompt: format!("Write code for: {}", task),
        temperature: 0.2,
        max_tokens: 2048,
    };

    println!("Executing...");
    let cancel = CancellationToken::new();
    let start = std::time::Instant::now();

    match provider.generate(request, cancel).await {
        Ok(response) => {
            let elapsed = start.elapsed();
            print!("\n=== Result ({}ms) ===\n{}\n", elapsed.as_millis(), response.text.trim());
            let mut ledger = ExecutionLedger::new();
            ledger.append(LedgerEntry {
                execution_id: format!("exec-{}", chrono::Utc::now().timestamp()),
                timestamp: chrono::Utc::now().to_rfc3339(),
                provider: "ollama".into(),
                workflow: "direct".into(),
                skill_version: None,
                reason: format!("Execute: {}", task),
                cost: 0.0,
                decision: "ollama/qwen2.5-coder".into(),
                outcome: LedgerOutcome::Success,
                previous_hash: None,
                hash: format!("hash-{}", rand::random::<u64>()),
                metadata: HashMap::new(),
            }).ok();
            println!("Ledger: {} entries", ledger.len());
        }
        Err(e) => {
            eprintln!("\nError: {}", e);
            eprintln!("Is Ollama running at http://localhost:11434?");
        }
    }
}
