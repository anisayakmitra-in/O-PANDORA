use crate::PackageInfo;

fn kind(id: &str) -> &'static str {
    match id {
        "workflow" => "Workflow",
        "code-review" => "Agent",
        "mcp" => "MCP",
        "benchmark" => "Benchmark",
        _ => "Tool",
    }
}

fn pkg(id: &str, desc: &str, caps: &[&str]) -> PackageInfo {
    PackageInfo {
        id: id.to_string(),
        name: id.to_string(),
        kind: kind(id).to_string(),
        version: "0.1.0".to_string(),
        author: "pandora".to_string(),
        description: desc.to_string(),
        source: "builtin".to_string(),
        capabilities: caps.iter().map(|s| s.to_string()).collect(),
        slash_commands: vec![],
    }
}

pub fn all() -> Vec<PackageInfo> {
    vec![
        pkg(
            "filesystem",
            "Read/write/list files",
            &["filesystem", "storage"],
        ),
        pkg("shell", "Execute shell commands", &["shell", "execution"]),
        pkg("git", "Git operations", &["git", "vcs"]),
        pkg("http", "HTTP requests via curl", &["http", "network"]),
        pkg(
            "rust-tool",
            "Cargo subcommands",
            &["rust", "cargo", "compilation"],
        ),
        pkg("python-tool", "Python evaluation", &["python", "scripting"]),
        pkg(
            "workflow",
            "Multi-step workflows",
            &["workflow", "automation"],
        ),
        pkg("docker", "Docker container ops", &["docker", "containers"]),
        pkg(
            "browser",
            "Web page content via Scrapling",
            &["browser", "scraping"],
        ),
        pkg("sqlite", "SQLite queries", &["sqlite", "database"]),
        pkg("github", "GitHub CLI ops", &["github", "devops"]),
        pkg("mcp", "MCP tools via npx", &["mcp", "protocol"]),
        pkg(
            "code-review",
            "Git diff analysis",
            &["code-review", "quality"],
        ),
        pkg(
            "benchmark",
            "Time command execution",
            &["benchmark", "performance"],
        ),
    ]
}

pub fn find(id: &str) -> Option<PackageInfo> {
    all().into_iter().find(|p| p.id == id)
}
