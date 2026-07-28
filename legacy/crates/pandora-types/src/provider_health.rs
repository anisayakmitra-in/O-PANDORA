//! Provider Health — check availability, list models, benchmark latency.
//!
//! Config: `OLLAMA_HOST`, `LLAMA_CPP_HOST`, `OLLAMA_MODEL`.
//! All endpoints come from environment variables with documented defaults.

use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::time::{Duration, Instant};

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
    match http_get_text(&format!("{host}/api/tags")) {
        Ok((code, body)) => ProviderHealth {
            name: "Ollama".into(),
            status: if code == StatusCode::OK {
                "OK".into()
            } else {
                format!("HTTP {}", code.as_u16())
            },
            model_count: body.matches(r#""name":"#).count() as u32,
            latency_ms: start.elapsed().as_millis() as u64,
            error: None,
        },
        Err(e) => ProviderHealth {
            name: "Ollama".into(),
            status: "OFFLINE".into(),
            model_count: 0,
            latency_ms: 0,
            error: Some(e.to_string()),
        },
    }
}

fn ollama_host() -> String {
    std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into())
}

/// Generic health check for any OpenAI-compatible endpoint.
pub fn check_openai_compat(name: &str, url: &str) -> ProviderHealth {
    let start = Instant::now();
    match http_get_text(url) {
        Ok((code, _)) => ProviderHealth {
            name: name.into(),
            status: if code == StatusCode::OK {
                "OK".into()
            } else {
                format!("HTTP {}", code.as_u16())
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
    let body = http_post_json(&format!("{host}/api/generate"), &payload.to_string())?;
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

fn http_client() -> Result<Client, crate::PandoraError> {
    Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| crate::PandoraError::Internal(format!("HTTP client init failed: {e}")))
}

fn http_get_text(url: &str) -> Result<(StatusCode, String), crate::PandoraError> {
    let response = http_client()?
        .get(url)
        .send()
        .map_err(|e| crate::PandoraError::Internal(format!("HTTP GET failed for {url}: {e}")))?;
    let status = response.status();
    let body = response.text().map_err(|e| {
        crate::PandoraError::Internal(format!("HTTP body read failed for {url}: {e}"))
    })?;
    Ok((status, body))
}

fn http_post_json(url: &str, data: &str) -> Result<String, crate::PandoraError> {
    let response = http_client()?
        .post(url)
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .map_err(|e| crate::PandoraError::Internal(format!("HTTP POST failed for {url}: {e}")))?;
    if !response.status().is_success() {
        return Err(crate::PandoraError::Internal(format!(
            "HTTP {} returned for {url}",
            response.status()
        )));
    }
    response
        .text()
        .map_err(|e| crate::PandoraError::Internal(format!("HTTP body read failed for {url}: {e}")))
}
