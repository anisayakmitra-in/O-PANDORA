//! Import — migrate settings from other AI agents.
//!
//! Supports importing from: Claude Code, OpenCode, Goose, Cline, Hermes.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub tool: String,
    pub imported: Vec<String>,
    pub skipped: Vec<String>,
    pub errors: Vec<String>,
}

/// Detect and import settings from other AI agents.
pub fn import_from(tool: &str, source_path: &str) -> Result<ImportResult, String> {
    let path = std::path::Path::new(source_path);
    if !path.exists() {
        return Err(format!("Source path does not exist: {source_path}"));
    }

    match tool {
        "claude-code" | "claude" => import_claude_code(path),
        "opencode" => import_opencode(path),
        "goose" => import_goose(path),
        "cline" => import_cline(path),
        "hermes" => import_hermes(path),
        _ => Err(format!("Unknown tool: {tool}")),
    }
}

fn import_claude_code(path: &std::path::Path) -> Result<ImportResult, String> {
    let mut imported = Vec::new();
    let skipped = Vec::new();

    // Claude Code stores settings in ~/.claude/settings.json
    if let Ok(content) = std::fs::read_to_string(path.join("settings.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(api_key) = json.get("apiKey").and_then(|v| v.as_str()) {
                std::env::set_var("ANTHROPIC_API_KEY", api_key);
                imported.push("API key".into());
            }
            if let Some(model) = json.get("model").and_then(|v| v.as_str()) {
                std::env::set_var("PANDORA_DEFAULT_MODEL", model);
                imported.push(format!("Model: {model}"));
            }
        }
    }

    // Import CLAUDE.md as a skill gene
    if path.join("CLAUDE.md").exists() {
        imported.push("CLAUDE.md (available as skill source)".into());
    }

    Ok(ImportResult {
        tool: "claude-code".into(),
        imported,
        skipped,
        errors: vec![],
    })
}

fn import_opencode(path: &std::path::Path) -> Result<ImportResult, String> {
    let mut imported = Vec::new();

    if let Ok(content) = std::fs::read_to_string(path.join("config.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(provider) = json.get("provider").and_then(|v| v.as_str()) {
                std::env::set_var("PANDORA_PROVIDER", provider);
                imported.push(format!("Provider: {provider}"));
            }
        }
    }

    Ok(ImportResult {
        tool: "opencode".into(),
        imported,
        skipped: vec![],
        errors: vec![],
    })
}

fn import_goose(path: &std::path::Path) -> Result<ImportResult, String> {
    let mut imported = Vec::new();

    // Goose stores config in ~/.config/goose/config.yaml
    if let Ok(content) = std::fs::read_to_string(path.join("config.yaml")) {
        if content.contains("openai") || content.contains("anthropic") {
            imported.push("Provider config detected".into());
        }
    }

    Ok(ImportResult {
        tool: "goose".into(),
        imported,
        skipped: vec![],
        errors: vec![],
    })
}

fn import_cline(path: &std::path::Path) -> Result<ImportResult, String> {
    let mut imported = Vec::new();

    // Cline stores settings in VS Code settings
    if let Ok(content) = std::fs::read_to_string(path.join("settings.json")) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(model) = json.get("cline.model").and_then(|v| v.as_str()) {
                std::env::set_var("PANDORA_DEFAULT_MODEL", model);
                imported.push(format!("Model: {model}"));
            }
        }
    }

    Ok(ImportResult {
        tool: "cline".into(),
        imported,
        skipped: vec![],
        errors: vec![],
    })
}

fn import_hermes(path: &std::path::Path) -> Result<ImportResult, String> {
    let mut imported = Vec::new();

    // Hermes stores config in ~/.hermes/.env
    if let Ok(content) = std::fs::read_to_string(path.join(".env")) {
        for line in content.lines() {
            if let Some(api_key) = line.strip_prefix("ANTHROPIC_API_KEY=") {
                std::env::set_var("ANTHROPIC_API_KEY", api_key);
                imported.push("Anthropic API key".into());
            }
            if let Some(model) = line.strip_prefix("DEFAULT_MODEL=") {
                std::env::set_var("PANDORA_DEFAULT_MODEL", model);
                imported.push(format!("Model: {model}"));
            }
        }
    }

    // Import skills directory
    if path.join("skills").is_dir() {
        imported.push("Skills directory detected (available as gene sources)".into());
    }

    Ok(ImportResult {
        tool: "hermes".into(),
        imported,
        skipped: vec![],
        errors: vec![],
    })
}

/// List supported import sources.
pub fn supported_tools() -> Vec<&'static str> {
    vec!["claude-code", "opencode", "goose", "cline", "hermes"]
}
