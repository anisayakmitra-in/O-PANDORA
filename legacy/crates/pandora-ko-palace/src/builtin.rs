use crate::PackageInfo;

fn pkg(id: &str, desc: &str, caps: &[&str]) -> PackageInfo {
    PackageInfo {
        id: id.into(),
        name: id.into(),
        kind: match id {
            "workflow" => "Workflow",
            "code-review" => "Agent",
            "mcp" => "MCP",
            "benchmark" => "Benchmark",
            _ => "Tool",
        }
        .into(),
        version: env!("CARGO_PKG_VERSION").into(),
        author: "pandora".into(),
        description: desc.into(),
        source: "builtin".into(),
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
            "browser",
            "Headless browser via playwright",
            &["browser", "web", "automation"],
        ),
        pkg(
            "youtube",
            "YouTube transcript download",
            &["youtube", "video", "transcript"],
        ),
        pkg(
            "scrape",
            "Web page to clean text",
            &["scrape", "web", "extract"],
        ),
        pkg("rss", "RSS/Atom feed reader", &["rss", "feed", "monitor"]),
        pkg(
            "github-issues",
            "GitHub Issues browser",
            &["github", "issues", "project"],
        ),
        pkg(
            "code-graph",
            "Static code analysis via tree-sitter",
            &["code", "graph", "analysis", "ast"],
        ),
        pkg(
            "api-scan",
            "HTTP route & dependency scanner",
            &["api", "scan", "routes"],
        ),
        pkg(
            "benchmark",
            "Time command execution",
            &["benchmark", "performance"],
        ),
        pkg(
            "computer-use",
            "Desktop automation harness — click, type, screenshot",
            &["desktop", "automation", "accessibility"],
        ),
    ]
}

pub fn find(id: &str) -> Option<PackageInfo> {
    all().into_iter().find(|p| p.id == id)
}
