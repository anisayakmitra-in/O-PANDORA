//! MCP (Model Context Protocol) Server for Pandora.
//!
//! Translates MCP protocol messages into ExecutionPlans and
//! runs them through the PandoraRuntime. Exposes `pandora_execute`
//! and `pandora_pipeline` as MCP tools. Built on top of the Runtime API.

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

// ── MCP Protocol Types ──

#[derive(Debug, Deserialize)]
#[expect(dead_code)]
struct McpRequest {
    jsonrpc: String,
    id: Option<u64>,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Debug, Serialize)]
struct McpResponse {
    jsonrpc: String,
    id: Option<u64>,
    result: Option<Value>,
    error: Option<Value>,
}

// ── MCP State ──

pub struct McpState {
    pub runtime: Arc<Mutex<pandora_orchestrator::PandoraRuntime>>,
}

/// Start an MCP server on the given address. Advertises pandora_execute
/// and pandora_pipeline as MCP tools.
pub async fn serve_mcp(addr: &str) -> Result<(), anyhow::Error> {
    let state = Arc::new(McpState {
        runtime: Arc::new(Mutex::new(pandora_orchestrator::PandoraRuntime::new())),
    });
    let app = Router::new()
        .route("/", post(mcp_handler))
        .route("/health", get(|| async { "ok" }))
        .with_state(state);
    println!("[MCP] Listening on {addr}");
    println!("[MCP] Tools: pandora_execute, pandora_pipeline");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Handle MCP JSON-RPC messages.
async fn mcp_handler(
    State(state): State<Arc<McpState>>,
    Json(req): Json<McpRequest>,
) -> Json<McpResponse> {
    match req.method.as_str() {
        "initialize" => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(serde_json::json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {"name":"pandora-mcp","version":"0.2.0"},
                "capabilities": {"tools": {}}
            })),
            error: None,
        }),
        "tools/list" => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(serde_json::json!({"tools": [
                {"name":"pandora_execute","description":"Execute a task through Pandora governed runtime","inputSchema":{"type":"object","properties":{"task":{"type":"string"},"strategy":{"type":"string"},"provider":{"type":"string"}},"required":["task"]}},
                {"name":"pandora_pipeline","description":"Run a full pipeline: plan, harness, gene, provider, evaluator, outcome","inputSchema":{"type":"object","properties":{"goal":{"type":"string"},"domain":{"type":"string"}},"required":["goal"]}},
                {"name":"pandora_genes","description":"List all registered genes available as tools","inputSchema":{"type":"object","properties":{}}},
                {"name":"pandora_harnesses","description":"List all registered harnesses","inputSchema":{"type":"object","properties":{}}},
                {"name":"pandora_sessions","description":"List recent sessions","inputSchema":{"type":"object","properties":{"limit":{"type":"number"}}}},
                {"name":"pandora_memory_search","description":"Search Pandora memory for relevant context","inputSchema":{"type":"object","properties":{"query":{"type":"string"}},"required":["query"]}},
                {"name":"pandora_install","description":"Install a package from K-O-Palace registry","inputSchema":{"type":"object","properties":{"id":{"type":"string"}},"required":["id"]}}
            ]})),
            error: None,
        }),
        "resources/list" => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(
                serde_json::json!({"resources":[{"uri":"pandora://sessions","name":"Sessions","mimeType":"application/json"},{"uri":"pandora://providers","name":"Providers","mimeType":"application/json"}]}),
            ),
            error: None,
        }),
        "notifications/cancelled" => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(serde_json::json!({"cancelled":true})),
            error: None,
        }),
        "prompts/list" => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: Some(
                serde_json::json!({"prompts":[{"name":"execute","description":"Run a Pandora execution","arguments":[{"name":"task","required":true}]}]}),
            ),
            error: None,
        }),
        "tools/call" => {
            let tool = req
                .params
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let args = req.params.get("arguments").unwrap_or(&Value::Null);
            let task = args
                .get("task")
                .or_else(|| args.get("goal"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let _strategy = args
                .get("strategy")
                .and_then(|v| v.as_str())
                .unwrap_or("single_shot");
            let mut runtime = state.runtime.lock().await;
            match runtime.run(task, "default").await {
                Ok(report) => Json(McpResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: Some(serde_json::json!({
                        "content": [{"type":"text","text": format!("Tool: {} | Status: {} | Duration: {}ms | Output: {}",
                            tool, if report.success {"ok"} else {"failed"}, report.duration_ms,
                            &report.output.chars().take(1000).collect::<String>())}],
                        "isError": !report.success
                    })),
                    error: None,
                }),
                Err(e) => Json(McpResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: None,
                    error: Some(serde_json::json!({"code":-1,"message":e.to_string()})),
                }),
            }
        }
        _ => Json(McpResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            error: Some(serde_json::json!({"code":-32601,"message":"Method not found"})),
            result: None,
        }),
    }
}
