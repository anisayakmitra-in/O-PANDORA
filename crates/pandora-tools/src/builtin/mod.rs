//! Built-in tool implementations.
//!
//! These tools are provided out-of-the-box for convenience. They are
//! NOT part of the tool contract — they are real implementations
//! built on top of it. Each tool here has a corresponding entry in
//! [`register_builtin`] that can be called explicitly, or skipped
//! entirely by callers who prefer to wire their own tools.

pub mod filesystem;
pub mod shell;
pub mod web_scrape;

use std::sync::Arc;

use crate::registry::ToolRegistry;
use crate::traits::Tool;

/// Register every built-in tool into the given registry.
///
/// Returns the number of tools registered. Idempotent: a tool that
/// is already present is skipped, not duplicated.
pub async fn register_builtin(registry: &ToolRegistry) -> usize {
    let builtins: Vec<Arc<dyn Tool>> = vec![
        Arc::new(filesystem::ReadFileTool::new()),
        Arc::new(web_scrape::WebScrapeTool::new()),
        Arc::new(shell::ShellTool::new()),
    ];

    let mut registered = 0;
    for tool in builtins {
        let id = tool.manifest().id;
        if registry.register(tool).await.is_ok() {
            registered += 1;
        } else {
            // Tool already present; skip silently.
            let _ = id;
        }
    }
    registered
}
