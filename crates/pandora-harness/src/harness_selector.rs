use pandora_model::OllamaClient;
use pandora_types::HarnessSpec;

pub async fn select_harness(
    client: &OllamaClient,
    model: &str,
    input: &str,
    specs: &[HarnessSpec],
) -> String {
    let options = specs
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    let prompt = format!(
        "You are a strict classifier.\n\
Available labels: {}\n\
Task: {}\n\
Return ONLY one label from the list.\n\
No explanation.",
        options, input
    );

    match client.chat(model, &prompt).await {
        Ok(res) => {
            let raw = res.message.content.trim().to_lowercase();

            let candidate = raw
                .split_whitespace()
                .next()
                .unwrap_or("default")
                .to_string();

            // ✅ validation
            if specs.iter().any(|s| s.name == candidate) {
                return candidate;
            }

            // ✅ fallback logic
            let input_lc = input.to_lowercase();

            if input_lc.contains("rust")
                || input_lc.contains("code")
                || input_lc.contains("bug")
            {
                return "coding".to_string();
            }

            if input_lc.contains("business")
                || input_lc.contains("market")
                || input_lc.contains("strategy")
            {
                return "business".to_string();
            }

            "default".to_string()
        }

        Err(_) => {
            let input_lc = input.to_lowercase();

            if input_lc.contains("rust") {
                "coding".to_string()
            } else {
                "default".to_string()
            }
        }
    }
}

pub fn select_best_by_performance(
    specs: &[HarnessSpec],
    performance: &pandora_memory::HarnessPerformance,
) -> String {
    let mut best = "default".to_string();
    let mut best_score = f32::MIN;

    for spec in specs {
        let avg = performance.average(&spec.name);

        if avg > best_score {
            best_score = avg;
            best = spec.name.clone();
        }
    }

    best
}
