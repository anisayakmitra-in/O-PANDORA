//! Agentic loop — the core LLM <-> tool execution cycle.
use pandora_types::gene::Gene;
use pandora_types::permissions_manifest::{PermissionManifest, PermissionVerdict};
use pandora_types::provider::{
    ChatCompletion, ChatMessage, GenerationRequest, Provider, ToolDefinition,
};
use std::collections::HashMap;
use std::time::Instant;

pub struct AgenticConfig {
    pub max_turns: u32,
    pub max_tokens: usize,
    pub temperature: f32,
}

impl Default for AgenticConfig {
    fn default() -> Self {
        Self {
            max_turns: 20,
            max_tokens: 4096,
            temperature: 0.2,
        }
    }
}

pub fn genes_to_tool_definitions(genes: &[&dyn Gene]) -> Vec<ToolDefinition> {
    genes.iter().map(|gene| {
        let m = gene.manifest();
        let description = if m.metadata.description.is_empty() {
            format!("Gene: {} ({})", m.name, m.kind.as_str())
        } else { m.metadata.description.clone() };
        let parameters = serde_json::json!({
            "type": "object",
            "properties": { "input": { "type": "string", "description": format!("Input for {} gene", m.name) } },
            "required": ["input"],
        });
        ToolDefinition { name: m.id.clone(), description, parameters }
    }).collect()
}

pub struct ToolResult {
    pub tool_name: String,
    pub tool_call_id: String,
    pub input: String,
    pub output: String,
    pub success: bool,
    pub duration_ms: u64,
}

pub struct AgenticResult {
    pub output: String,
    pub turns_used: u32,
    pub tool_calls_made: u32,
    pub tool_results: Vec<ToolResult>,
    pub total_tokens: usize,
    pub duration_ms: u128,
}

pub fn run_agentic_loop(
    task: &str,
    domain: &str,
    provider: &dyn Provider,
    genes: &[&dyn Gene],
    permissions: Option<&PermissionManifest>,
    config: &AgenticConfig,
) -> Result<AgenticResult, String> {
    let start = Instant::now();
    let tools = genes_to_tool_definitions(genes);
    let gene_map: HashMap<String, &dyn Gene> =
        genes.iter().map(|g| (g.id().to_string(), *g)).collect();
    let system_prompt = format!(
        "You are Pandora, a governed execution runtime.\nDomain: {domain}\n\nYou have access to tools (genes). Use them to accomplish the task.\nWhen done, respond with a summary without calling more tools."
    );
    let mut messages: Vec<ChatMessage> = vec![
        ChatMessage {
            role: "system".into(),
            content: system_prompt,
            tool_calls: vec![],
            tool_call_id: None,
        },
        ChatMessage {
            role: "user".into(),
            content: task.into(),
            tool_calls: vec![],
            tool_call_id: None,
        },
    ];
    let mut turns_used: u32 = 0;
    let mut tool_calls_made: u32 = 0;
    let mut tool_results: Vec<ToolResult> = Vec::new();
    let mut total_tokens: usize = 0;
    let final_output: String;

    loop {
        turns_used += 1;
        if turns_used > config.max_turns {
            final_output = format!("Goal paused - max turns ({}) reached", config.max_turns);
            break;
        }
        let request = GenerationRequest {
            model: String::new(),
            prompt: String::new(),
            temperature: config.temperature,
            max_tokens: config.max_tokens,
            ..Default::default()
        };
        let completion = if provider.supports_tools() && !tools.is_empty() {
            provider.generate_with_tools(request, &tools, &messages)?
        } else {
            let prompt = messages
                .iter()
                .map(|m| format!("{}: {}", m.role, m.content))
                .collect::<Vec<_>>()
                .join("\n");
            let req = GenerationRequest {
                prompt,
                temperature: config.temperature,
                max_tokens: config.max_tokens,
                ..Default::default()
            };
            let text = provider.generate(req)?;
            ChatCompletion {
                text,
                tool_calls: vec![],
                finish_reason: "stop".into(),
                tokens_used: 0,
            }
        };
        total_tokens += completion.tokens_used;
        messages.push(ChatMessage {
            role: "assistant".into(),
            content: completion.text.clone(),
            tool_calls: completion.tool_calls.clone(),
            tool_call_id: None,
        });
        if completion.tool_calls.is_empty() || completion.finish_reason == "stop" {
            final_output = completion.text;
            break;
        }
        for tc in &completion.tool_calls {
            tool_calls_made += 1;
            let gene = match gene_map.get(&tc.name) {
                Some(g) => *g,
                None => {
                    messages.push(ChatMessage {
                        role: "tool".into(),
                        content: format!("Error: unknown tool {}", tc.name),
                        tool_calls: vec![],
                        tool_call_id: Some(tc.id.clone()),
                    });
                    continue;
                }
            };
            let input = if let Ok(args) = serde_json::from_str::<serde_json::Value>(&tc.arguments) {
                args["input"].as_str().unwrap_or(&tc.arguments).to_string()
            } else {
                tc.arguments.clone()
            };
            if let Some(perm) = permissions {
                if !matches!(perm.is_shell_allowed(&input), PermissionVerdict::Allowed)
                    && gene.kind().as_str() == "tool"
                {
                    messages.push(ChatMessage {
                        role: "tool".into(),
                        content: format!(
                            "Permission denied for tool {}: shell not allowed",
                            tc.name
                        ),
                        tool_calls: vec![],
                        tool_call_id: Some(tc.id.clone()),
                    });
                    continue;
                }
            }
            let exec_start = Instant::now();
            let result = gene.execute(&input);
            let exec_ms = exec_start.elapsed().as_millis() as u64;
            let (output, success) = match result {
                Ok(out) => {
                    let t = if out.len() > 8000 {
                        format!("{}...(truncated, {} total)", &out[..8000], out.len())
                    } else {
                        out
                    };
                    (t, true)
                }
                Err(e) => (format!("Error: {e}"), false),
            };
            tool_results.push(ToolResult {
                tool_name: tc.name.clone(),
                tool_call_id: tc.id.clone(),
                input: input.clone(),
                output: output.clone(),
                success,
                duration_ms: exec_ms,
            });
            messages.push(ChatMessage {
                role: "tool".into(),
                content: output,
                tool_calls: vec![],
                tool_call_id: Some(tc.id.clone()),
            });
        }
    }
    Ok(AgenticResult {
        output: final_output,
        turns_used,
        tool_calls_made,
        tool_results,
        total_tokens,
        duration_ms: start.elapsed().as_millis(),
    })
}
