//! Configurable genes — Postgres, Slack, Jira, Go, Node, Java.
//!
//! Nothing hardcoded: all endpoints, paths, and tokens come from
//! environment variables with documented defaults.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id).name(id).kind(kind).version("0.1.0").author("pandora")
        .description(desc).build().unwrap()
}

fn run(_cmd: &str, args: &[&str]) -> Result<String, String> {
    if args.is_empty() { return Err("No command provided".into()); }
    let out = Command::new(args[0]).args(&args[1..]).output()
        .map_err(|e| format!("{} not found: {}. Install it or check PATH.", args[0], e))?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    if out.status.success() { Ok(stdout) }
    else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
}

// ── Postgres Gene ──
// Config: PG_HOST, PG_PORT, PG_USER, PG_DB (default: localhost:5432/postgres)
#[derive(Debug, Default)]
pub struct PostgresGene { m: GeneManifest }
impl PostgresGene {
    pub fn new() -> Self {
        Self { m: mk("postgres", GeneKind::Tool,
            "Execute SQL via psql. Config: PG_HOST/PORT/USER/DB") }
    }
}
impl Gene for PostgresGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: postgres <SQL query>".into()); }
        let host = std::env::var("PG_HOST").unwrap_or_else(|_| "localhost".into());
        let port = std::env::var("PG_PORT").unwrap_or_else(|_| "5432".into());
        let user = std::env::var("PG_USER").unwrap_or_else(|_| "postgres".into());
        let db = std::env::var("PG_DB").unwrap_or_else(|_| "postgres".into());
        run("psql", &["-h", &host, "-p", &port, "-U", &user, "-d", &db, "-c", input])
    }
}

// ── Go Gene ──
// Config: GO_CMD (default: go), GO_FLAGS
#[derive(Debug, Default)]
pub struct GoGene { m: GeneManifest }
impl GoGene {
    pub fn new() -> Self {
        Self { m: mk("go", GeneKind::Tool,
            "Run Go commands. Config: GO_CMD (default: go), GO_FLAGS") }
    }
}
impl Gene for GoGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: go <subcommand> [args]".into()); }
        let cmd = std::env::var("GO_CMD").unwrap_or_else(|_| "go".into());
        let mut full = vec![cmd.as_str()];
        if let Ok(flags) = std::env::var("GO_FLAGS") { full.extend(flags.split_whitespace()); }
        full.extend(input.split_whitespace());
        run(&full[0], &full[1..])
    }
}

// ── Node Gene ──
// Config: NODE_CMD (default: node), NODE_FLAGS
#[derive(Debug, Default)]
pub struct NodeGene { m: GeneManifest }
impl NodeGene {
    pub fn new() -> Self {
        Self { m: mk("node", GeneKind::Tool,
            "Run Node.js scripts. Config: NODE_CMD (default: node), NODE_FLAGS") }
    }
}
impl Gene for NodeGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: node <script.js> [args]".into()); }
        let cmd = std::env::var("NODE_CMD").unwrap_or_else(|_| "node".into());
        let mut full = vec![cmd.as_str()];
        if let Ok(flags) = std::env::var("NODE_FLAGS") { full.extend(flags.split_whitespace()); }
        full.extend(input.split_whitespace());
        run(&full[0], &full[1..])
    }
}

// ── Java Gene ──
// Config: JAVA_CMD (default: java), JAVA_FLAGS, JAVA_CLASSPATH
#[derive(Debug, Default)]
pub struct JavaGene { m: GeneManifest }
impl JavaGene {
    pub fn new() -> Self {
        Self { m: mk("java", GeneKind::Tool,
            "Run Java. Config: JAVA_CMD (default: java), JAVA_FLAGS, JAVA_CLASSPATH") }
    }
}
impl Gene for JavaGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: java <class> [args]".into()); }
        let cmd = std::env::var("JAVA_CMD").unwrap_or_else(|_| "java".into());
        let mut full = vec![cmd.as_str()];
        if let Ok(flags) = std::env::var("JAVA_FLAGS") { full.extend(flags.split_whitespace()); }
        if let Ok(cp) = std::env::var("JAVA_CLASSPATH") { full.push("-cp"); full.push(&cp); }
        full.extend(input.split_whitespace());
        run(&full[0], &full[1..])
    }
}
