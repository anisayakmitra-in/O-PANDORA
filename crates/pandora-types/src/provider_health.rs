//! Provider Health — check availability, list models, benchmark latency.
//!
//! Config: OLLAMA_HOST, LLAMA_CPP_HOST, OPENAI_KEY, etc.
//! ponytail: delegates to curl for health checks, simple timing for benchmarks.

use std::process::Command;
use std::time::Instant;

/// Provider health status.
#[derive(Debug)]
pub struct ProviderHealth {
    pub name: String,
    pub status: String,
    pub model_count: u32,
    pub latency_ms: u64,
    pub error: Option<String>,
}

/// Check a provider's health by hitting its API endpoint.
pub fn check_ollama() -> ProviderHealth {
    let start = Instant::now();
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    match Command::new("curl").args(["-s", "-o", "/dev/null", "-w", "%{http_code}", &format!("{}/api/tags", host)]).output() {
        Ok(out) => {
            let code = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let models = count_ollama_models(&host);
            ProviderHealth {
                name: "Ollama".into(),
                status: if code == "200" { "OK".into() } else { format!("HTTP {}", code) },
                model_count: models,
                latency_ms: start.elapsed().as_millis() as u64,
                error: None,
            }
        }
        Err(e) => ProviderHealth {
            name: "Ollama".into(),
            status: "OFFLINE".into(),
            model_count: 0,
            latency_ms: 0,
            error: Some(format!("{}", e)),
        },
    }
}

fn count_ollama_models(host: &str) -> u32 {
    let out = Command::new("curl").args(["-s", &format!("{}/api/tags", host)]).output().ok();
    if let Some(o) = out {
        let body = String::from_utf8_lossy(&o.stdout);
        // Count "name": occurrences as a rough model count
        body.matches("\"name\":").count() as u32
    } else { 0 }
}

/// Check a provider's health. Generic for any OpenAI-compatible endpoint.
pub fn check_openai_compat(name: &str, url: &str) -> ProviderHealth {
    let start = Instant::now();
    match Command::new("curl").args(["-s", "-o", "/dev/null", "-w", "%{http_code}", url]).output() {
        Ok(out) => {
            let code = String::from_utf8_lossy(&out.stdout).trim().to_string();
            ProviderHealth {
                name: name.into(),
                status: if code == "200" { "OK".into() } else { format!("HTTP {}", code) },
                model_count: 0,
                latency_ms: start.elapsed().as_millis() as u64,
                error: None,
            }
        }
        Err(e) => ProviderHealth {
            name: name.into(),
            status: "OFFLINE".into(),
            model_count: 0,
            latency_ms: 0,
            error: Some(format!("{}", e)),
        },
    }
}

/// Benchmark a provider by timing a simple prompt.
/// Returns (latency_ms, tokens_per_sec) or an error.
pub fn benchmark_provider(name: &str, host: &str, model: &str, prompt: &str) -> Result<(u64, f64), String> {
    let payload = format!(r#"{{"model":"{}","prompt":"{}","stream":false}}"#, model, prompt);
    let start = Instant::now();
    let out = Command::new("curl")
        .args(["-s", "-X", "POST", "-H", "Content-Type: application/json",
               "-d", &payload, &format!("{}/api/generate", host)])
        .output()
        .map_err(|e| format!("{} not reachable: {}", name, e))?;
    let elapsed = start.elapsed().as_millis() as u64;
    let body = String::from_utf8_lossy(&out.stdout);
    if !out.status.success() {
        return Err(format!("{} returned HTTP {}", name, out.status));
    }
    // Parse total_duration from response for tokens/sec
    let tokens = body.matches("\"eval_count\":")
        .next().and_then(|_| {
            body.split("\"eval_count\":").nth(1)
                .and_then(|s| s.split(',').next())
                .and_then(|s| s.trim().parse::<f64>().ok())
        }).unwrap_or(0.0);
    let tps = if elapsed > 0 { tokens / (elapsed as f64 / 1000.0) } else { 0.0 };
    Ok((elapsed, tps))
}

/// Run a full benchmark across all configured providers.
pub fn benchmark_all() -> Vec<(String, String, u64, f64)> {
    let mut results = Vec::new();
    let prompt = "def hello():\n    print('hello world')\n\nhello()";
    
    // Ollama
    let host = std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into());
    let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| "qwen2.5-coder:7b".into());
    match benchmark_provider("Ollama", &host, &model, prompt) {
        Ok((lat, tps)) => results.push(("Ollama".into(), model, lat, tps)),
        Err(e) => results.push(("Ollama".into(), format!("error: {}", e), 0, 0.0)),
    }

    // LlamaCpp
    let host2 = std::env::var("LLAMA_CPP_HOST").unwrap_or_else(|_| "http://localhost:8080".into());
    match benchmark_provider("LlamaCpp", &host2, "default", prompt) {
        Ok((lat, tps)) => results.push(("LlamaCpp".into(), "default".into(), lat, tps)),
        Err(e) => results.push(("LlamaCpp".into(), format!("error: {}", e), 0, 0.0)),
    }

    results
}
