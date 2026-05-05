use pandora_harness::HarnessRunner;
use pandora_types::HarnessSpec;

#[tokio::main]
async fn main() {
    let mut runner = HarnessRunner::new("http://127.0.0.1:11434");

    // 🔥 DYNAMIC HARNESS CONFIG
    let harnesses = vec![
        HarnessSpec {
            name: "coding".to_string(),
            domain: "rust".to_string(),
            allowed_tools: vec![],
            max_steps: 5,
            requires_validation: false,
        },
        HarnessSpec {
            name: "business".to_string(),
            domain: "business".to_string(),
            allowed_tools: vec![],
            max_steps: 5,
            requires_validation: false,
        },
    ];

    let input = "Explain Rust ownership in 2 lines";

    let result = runner
        .run_with_specs("qwen2.5-coder:7b", input, &harnesses)
        .await;

    match result {
        Ok(output) => println!("RESULT:\n{}", output),
        Err(e) => println!("ERROR: {}", e),
    }
}
 
