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
        .map(|s| format!(
            "{} specializes in {}",
            s.name,
            s.domain
        ))
        .collect::<Vec<_>>()
        .join(", ");

    let prompt = format!(
        "You are a strict routing system.\n\
Available harnesses: {}\n\
Task: {}\n\
Return EXACTLY one harness name.\n\
No explanation.\n\
No extra text.",
        options,
        input
    );

    match client.chat(model, &prompt).await {

        Ok(res) => {

            let raw = res.message.content.trim().to_lowercase();

            let candidate = raw
                .split_whitespace()
                .next()
                .unwrap_or("default")
                .to_string();

            if specs.iter().any(|s| s.name == candidate) {

                candidate

            } else {

                // fallback routing
                if input.to_lowercase().contains("rust") {
                    return "coding".to_string();
                }

                "default".to_string()
            }
        }

        Err(_) => {

            if input.to_lowercase().contains("rust") {
                return "coding".to_string();
            }

            "default".to_string()
        }
    }
}

pub fn select_best_by_performance(
    scores: &std::collections::HashMap<String, Vec<i32>>,
) -> Option<String> {

    let mut best_name = None;
    let mut best_avg = f32::MIN;

    for (name, values) in scores {

        if values.is_empty() {
            continue;
        }

        let sum: i32 = values.iter().sum();

        let avg = sum as f32 / values.len() as f32;

        if avg > best_avg {

            best_avg = avg;
            best_name = Some(name.clone());
        }
    }

    best_name
}
