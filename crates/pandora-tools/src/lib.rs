pub trait Tool {
    fn name(&self) -> &str;

    fn execute(&self, input: &str) -> String;
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn execute(&self, input: &str) -> String {
        format!("READ FILE TOOL:\n{}", input)
    }
}

pub mod filesystem;

pub struct WebScrapeTool;

impl Tool for WebScrapeTool {
    fn name(&self) -> &str {
        "web_scrape"
    }

    fn execute(&self, input: &str) -> String {
        format!("SCRAPLING RESEARCH TOOL:\nScraped data from {}", input)
    }
}

pub struct ShellTool;

impl Tool for ShellTool {
    fn name(&self) -> &str {
        "shell"
    }

    fn execute(&self, input: &str) -> String {
        format!("SHELL TOOL EXECUTED:\n{}", input)
    }
}

pub fn tool_registry() -> Vec<Box<dyn Tool>> {
    vec![
        Box::new(ReadFileTool),
        Box::new(WebScrapeTool),
        Box::new(ShellTool),
    ]
}
