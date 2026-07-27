//! Provider Health — check availability, list models, benchmark latency.
//!
//! Config: `OLLAMA_HOST`, `LLAMA_CPP_HOST`, `OLLAMA_MODEL`.
//! All endpoints come from environment variables with documented defaults.

use std::process::Command;
use std::time::Instant;

/// Provider health status.
#[derive(Debug)]
pub struct ProviderHealth {
    /// Provider display name.
    pub name: String,
    /// Status string, e.g. "OK", "OFFLINE", "HTTP 503".
    pub status: String,
    /// Number of models available (approximate).
    pub model_count: u32,
    /// Response time in milliseconds.
    pub latency_ms: u64,
    /// Error details, if any.
    pub error: Option<String>,
}

/// Check Ollama health by hitting `/api/tags`.
pub fn check_ollama() -> ProviderHealth {
    let start = Instant::now();
    let host = ollama_host();
    let code = match curl_http_code(&format!("{host}/api/tags")) {
        Ok(c) => c,
        Err(e) => {
            return ProviderHealth {
                name: "Ollama".into(),
                status: "OFFLINE".into(),
                model_count: 0,
                latency_ms: 0,
                error: Some(e.to_string()),
            };
        }
    };
    let models = count_ollama_models(&host);
    ProviderHealth {
        name: "Ollama".into(),
        status: if code == "200" {
            "OK".into()
        } else {
            format!("HTTP {code}")
        },
        model_count: models,
        latency_ms: start.elapsed().as_millis() as u64,
        error: None,
    }
}

fn ollama_host() -> String {
    std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into())
}

fn count_ollama_models(host: &str) -> u32 {
    let out = match Command::new("curl")
        .args(["-s", &format!("{host}/api/tags")])
        .output()
    {
        Ok(o) => o,
        Err(_) => return 0,
    };
    let body = String::from_utf8_lossy(&out.stdout);
    body.matches(r#""name":"#).count() as u32
}

/// Generic health check for any OpenAI-compatible endpoint.
pub fn check_openai_compat(name: &str, url: &str) -> ProviderHealth {
    let start = Instant::now();
    match curl_http_code(url) {
        Ok(code) => ProviderHealth {
            name: name.into(),
            status: if code == "200" {
                "OK".into()
            } else {
                format!("HTTP {code}")
            },
            model_count: 0,
            latency_ms: start.elapsed().as_millis() as u64,
            error: None,
        },
        Err(e) => ProviderHealth {
            name: name.into(),
            status: "OFFLINE".into(),
            model_count: 0,
            latency_ms: 0,
            error: Some(e.to_string()),
        },
    }
}

/// Benchmark a provider by timing a prompt completion.
pub fn benchmark_provider(
    _name: &str,
    host: &str,
    model: &str,
    prompt: &str,
) -> Result<(u64, f64), crate::PandoraError> {
    let payload = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
    });
    let start = Instant::now();
    let body = curl_post(&format!("{host}/api/generate"), &payload.to_string())?;
    let elapsed = start.elapsed().as_millis() as u64;
    let tokens: f64 = body
        .split(r#""eval_count":"#)
        .nth(1)
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0.0);
    let tps = if elapsed > 0 {
        tokens / (elapsed as f64 / 1000.0)
    } else {
        0.0
    };
    Ok((elapsed, tps))
}

/// Run a full benchmark across all configured providers.
pub fn benchmark_all() -> Vec<(String, String, u64, f64)> {
    let mut results = Vec::new();
    let prompt = "def hello():\n    print('hello world')\n\nhello()";

    let host = ollama_host();
    let model = std::env::var("OLLAMA_MODEL").unwrap_or_else(|_| {
        std::env::var("PANDORA_DEFAULT_MODEL").unwrap_or_else(|_| String::new())
    });
    match benchmark_provider("Ollama", &host, &model, prompt) {
        Ok((lat, tps)) => results.push(("Ollama".into(), model, lat, tps)),
        Err(e) => results.push(("Ollama".into(), format!("error: {e}"), 0, 0.0)),
    }

    let host2 = std::env::var("LLAMA_CPP_HOST").unwrap_or_else(|_| "http://localhost:8080".into());
    match benchmark_provider("LlamaCpp", &host2, "default", prompt) {
        Ok((lat, tps)) => results.push(("LlamaCpp".into(), "default".into(), lat, tps)),
        Err(e) => results.push(("LlamaCpp".into(), format!("error: {e}"), 0, 0.0)),
    }

    results
}

// ── Internal helpers ──

fn curl_http_code(url: &str) -> Result<String, crate::PandoraError> {
    let out = Command::new("curl")
        .args(["-s", "-o", "/dev/null", "-w", "%{http_code}", url])
        .output()
        .map_err(|e| crate::PandoraError::Internal(format!("curl not found: {e}")))?;
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn curl_post(url: &str, data: &str) -> Result<String, crate::PandoraError> {
    let out = Command::new("curl")
        .args([
            "-s",
            "-X",
            "POST",
            "-H",
            "Content-Type: application/json",
            "-d",
            data,
            url,
        ])
        .output()
        .map_err(|e| crate::PandoraError::Internal(format!("curl not found: {e}")))?;
    if !out.status.success() {
        return Err(format!("HTTP {}/{}", url, out.status).into());
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}
