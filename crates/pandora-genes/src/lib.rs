use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(id)
        .kind(kind)
        .version("0.1.0")
        .author("pandora")
        .description(format!("{} gene", id))
        .build()
        .unwrap()
}

#[derive(Debug)]
pub struct FilesystemGene {
    m: GeneManifest,
}
impl Default for FilesystemGene {
    fn default() -> Self {
        Self::new()
    }
}

impl FilesystemGene {
    pub fn new() -> Self {
        Self {
            m: mk("filesystem", GeneKind::Tool),
        }
    }
}
impl Gene for FilesystemGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let p: Vec<&str> = input.splitn(2, ' ').collect();
        if p.is_empty() || p[0] == "help" {
            return Ok("Usage: read|write|list <path>".into());
        }
        match p[0] {
            "read" => std::fs::read_to_string(p.get(1).unwrap_or(&"")).map_err(|e| e.to_string()),
            "write" => {
                let path = p.get(1).ok_or("Missing path")?;
                std::fs::write(path, "content").map_err(|e| e.to_string())?;
                Ok(format!("wrote {}", path))
            }
            "list" => {
                let dir = std::fs::read_dir(p.get(1).unwrap_or(&".")).map_err(|e| e.to_string())?;
                Ok(dir
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
            _ => Err(format!("Unknown: {}", p[0])),
        }
    }
}

#[derive(Debug)]
pub struct ShellGene {
    m: GeneManifest,
}
impl Default for ShellGene {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellGene {
    pub fn new() -> Self {
        Self {
            m: mk("shell", GeneKind::Tool),
        }
    }
}
impl Gene for ShellGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("sh")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        if !err.is_empty() {
            return Err(err);
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct GitGene {
    m: GeneManifest,
}
impl Default for GitGene {
    fn default() -> Self {
        Self::new()
    }
}

impl GitGene {
    pub fn new() -> Self {
        Self {
            m: mk("git", GeneKind::Tool),
        }
    }
}
impl Gene for GitGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("git")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).to_string());
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct HTTPGene {
    m: GeneManifest,
}
impl Default for HTTPGene {
    fn default() -> Self {
        Self::new()
    }
}

impl HTTPGene {
    pub fn new() -> Self {
        Self {
            m: mk("http", GeneKind::Tool),
        }
    }
}
impl Gene for HTTPGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("curl")
            .arg("-s")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct RustToolGene {
    m: GeneManifest,
}
impl Default for RustToolGene {
    fn default() -> Self {
        Self::new()
    }
}

impl RustToolGene {
    pub fn new() -> Self {
        Self {
            m: mk("rust-tool", GeneKind::Tool),
        }
    }
}
impl Gene for RustToolGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("cargo")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| e.to_string())?;
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        if !err.is_empty() {
            return Err(err);
        }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct PythonToolGene {
    m: GeneManifest,
}
impl Default for PythonToolGene {
    fn default() -> Self {
        Self::new()
    }
}

impl PythonToolGene {
    pub fn new() -> Self {
        Self {
            m: mk("python-tool", GeneKind::Tool),
        }
    }
}
impl Gene for PythonToolGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("python3")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct WorkflowGene {
    m: GeneManifest,
}
impl Default for WorkflowGene {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowGene {
    pub fn new() -> Self {
        Self {
            m: mk("workflow", GeneKind::Workflow),
        }
    }
}
impl Gene for WorkflowGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let mut r = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let out = Command::new("sh")
                .arg("-c")
                .arg(line)
                .output()
                .map_err(|e| e.to_string())?;
            r.push(format!(
                "step {}: {}",
                i + 1,
                String::from_utf8_lossy(&out.stdout).trim()
            ));
        }
        Ok(r.join("\n"))
    }
}

/// Execute docker-compose commands.
#[derive(Debug)]
pub struct DockerComposeGene {
    m: GeneManifest,
}

impl Default for DockerComposeGene {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerComposeGene {
    pub fn new() -> Self {
        Self {
            m: mk("docker-compose", GeneKind::Tool),
        }
    }
}

impl Gene for DockerComposeGene {
    fn id(&self) -> &str {
        &self.m.id
    }
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: docker-compose <args>".into());
        }
        let out = std::process::Command::new("docker-compose")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| format!("docker-compose not found: {}", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

/// Execute terraform commands.
#[derive(Debug)]
pub struct TerraformGene {
    m: GeneManifest,
}

impl Default for TerraformGene {
    fn default() -> Self {
        Self::new()
    }
}

impl TerraformGene {
    pub fn new() -> Self {
        Self {
            m: mk("terraform", GeneKind::Tool),
        }
    }
}

impl Gene for TerraformGene {
    fn id(&self) -> &str {
        &self.m.id
    }
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: terraform <args>".into());
        }
        let out = std::process::Command::new("terraform")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| format!("terraform not found: {}", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

/// Execute kubectl commands.
#[derive(Debug)]
pub struct KubectlGene {
    m: GeneManifest,
}

impl Default for KubectlGene {
    fn default() -> Self {
        Self::new()
    }
}

impl KubectlGene {
    pub fn new() -> Self {
        Self {
            m: mk("kubectl", GeneKind::Tool),
        }
    }
}

impl Gene for KubectlGene {
    fn id(&self) -> &str {
        &self.m.id
    }
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: kubectl <args>".into());
        }
        let out = std::process::Command::new("kubectl")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| format!("kubectl not found: {}", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct DockerGene {
    m: GeneManifest,
}
impl Default for DockerGene {
    fn default() -> Self {
        Self::new()
    }
}

impl DockerGene {
    pub fn new() -> Self {
        Self {
            m: mk("docker", GeneKind::Tool),
        }
    }
}
impl Gene for DockerGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: docker <args>\nExample: docker ps\nExample: docker images".into());
        }
        let args: Vec<&str> = input.split_whitespace().collect();
        let out = std::process::Command::new("docker")
            .args(&args)
            .output()
            .map_err(|e| format!("docker not found: {}. Install Docker first.", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("docker exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct BrowserGene {
    m: GeneManifest,
}
impl Default for BrowserGene {
    fn default() -> Self {
        Self::new()
    }
}

impl BrowserGene {
    pub fn new() -> Self {
        Self {
            m: mk("browser", GeneKind::Tool),
        }
    }
}
impl Gene for BrowserGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: browser <url>".into());
        }
        let out = Command::new("curl")
            .arg("-sL")
            .arg(input.trim())
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct SQLiteGene {
    m: GeneManifest,
}
impl Default for SQLiteGene {
    fn default() -> Self {
        Self::new()
    }
}

impl SQLiteGene {
    pub fn new() -> Self {
        Self {
            m: mk("sqlite", GeneKind::Tool),
        }
    }
}
impl Gene for SQLiteGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let p: Vec<&str> = input.splitn(2, ' ').collect();
        if p.len() < 2 {
            return Err("Usage: sqlite <db> <query>".into());
        }
        let out = Command::new("sqlite3")
            .arg(p[0])
            .arg(p[1])
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

#[derive(Debug)]
pub struct GitHubGene {
    m: GeneManifest,
}
impl Default for GitHubGene {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHubGene {
    pub fn new() -> Self {
        Self {
            m: mk("github", GeneKind::Tool),
        }
    }
}
impl Gene for GitHubGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err(
                "Usage: gh <command>\nExample: gh pr list\nExample: gh issue view 123".into(),
            );
        }
        let args: Vec<&str> = input.split_whitespace().collect();
        let out = std::process::Command::new("gh")
            .args(&args)
            .output()
            .map_err(|e| {
                format!(
                    "gh not found: {}. Install GitHub CLI (https://cli.github.com).",
                    e
                )
            })?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("gh exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct MCPGene {
    m: GeneManifest,
}
impl Default for MCPGene {
    fn default() -> Self {
        Self::new()
    }
}

impl MCPGene {
    pub fn new() -> Self {
        Self {
            m: mk("mcp", GeneKind::MCP),
        }
    }
}
impl Gene for MCPGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() {
            return Err("Usage: mcp <package> [args]\nRequires Node.js (npx)".into());
        }
        let args: Vec<&str> = input.split_whitespace().collect();
        let mut cmd = std::process::Command::new("npx");
        cmd.arg("-y");
        for a in &args {
            cmd.arg(a);
        }
        let out = cmd
            .output()
            .map_err(|e| format!("npx not found: {}. Install Node.js.", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() {
                format!("npx exit {}", out.status)
            } else {
                stderr.trim().to_string()
            });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct CodeReviewGene {
    m: GeneManifest,
}
impl Default for CodeReviewGene {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeReviewGene {
    pub fn new() -> Self {
        Self {
            m: mk("code-review", GeneKind::Agent),
        }
    }
}
impl Gene for CodeReviewGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("git")
            .arg("diff")
            .args(input.split_whitespace())
            .output()
            .map_err(|e| e.to_string())?;
        let diff = String::from_utf8_lossy(&out.stdout);
        if diff.is_empty() {
            return Ok("No changes.".into());
        }
        let lines: Vec<&str> = diff.lines().collect();
        let a = lines.iter().filter(|l| l.starts_with('+')).count();
        let r = lines.iter().filter(|l| l.starts_with('-')).count();
        Ok(format!("{} lines, +{}/-{}", lines.len(), a, r))
    }
}

#[derive(Debug)]
pub struct BenchmarkGene {
    m: GeneManifest,
}
impl Default for BenchmarkGene {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchmarkGene {
    pub fn new() -> Self {
        Self {
            m: mk("benchmark", GeneKind::Benchmark),
        }
    }
}
impl Gene for BenchmarkGene {
    fn manifest(&self) -> &GeneManifest {
        &self.m
    }
    fn execute(&self, input: &str) -> Result<String, String> {
        use std::time::Instant;
        let start = Instant::now();
        if input.trim().is_empty() {
            return Err("Usage: benchmark <command>\nExample: benchmark curl -s http://localhost:11434/api/tags".into());
        }
        let out = std::process::Command::new("sh")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| format!("Failed: {}", e))?;
        let elapsed = start.elapsed();
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(format!("Exit {}: {}", out.status, stderr.trim()));
        }
        Ok(format!("{:.2?}\n{}", elapsed, stdout))
    }
}

// ── Postgres Gene ──
// Config: PG_HOST, PG_PORT, PG_USER, PG_DB (default: localhost:5432/postgres)
#[derive(Debug)]
pub struct PostgresGene { m: GeneManifest }
impl Default for PostgresGene { fn default() -> Self { Self::new() } }
impl PostgresGene { pub fn new() -> Self { Self { m: mk("postgres", GeneKind::Tool) } } }
impl Gene for PostgresGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: postgres <SQL query>".into()); }
        let host = std::env::var("PG_HOST").unwrap_or_else(|_| "localhost".into());
        let port = std::env::var("PG_PORT").unwrap_or_else(|_| "5432".into());
        let user = std::env::var("PG_USER").unwrap_or_else(|_| "postgres".into());
        let db = std::env::var("PG_DB").unwrap_or_else(|_| "postgres".into());
        let out = std::process::Command::new("psql").args(&["-h", &host, "-p", &port, "-U", &user, "-d", &db, "-c", input]).output().map_err(|e| e.to_string())?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if out.status.success() { Ok(stdout) } else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
    }
}

// ── Go Gene ──
// Config: GO_CMD (default: go), GO_FLAGS
#[derive(Debug)]
pub struct GoGene { m: GeneManifest }
impl Default for GoGene { fn default() -> Self { Self::new() } }
impl GoGene { pub fn new() -> Self { Self { m: mk("go", GeneKind::Tool) } } }
impl Gene for GoGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: go <subcommand> [args]".into()); }
        let mut go = std::env::var("GO_CMD").unwrap_or_else(|_| "go".into());
        if let Ok(flags) = std::env::var("GO_FLAGS") { go.push(' '); go.push_str(&flags); }
        let mut cmd = go.split_whitespace().map(String::from).collect::<Vec<_>>();
        cmd.extend(input.split_whitespace().map(String::from));
        let refs: Vec<&str> = cmd.iter().map(String::as_str).collect();
        let out = std::process::Command::new(refs[0]).args(&refs[1..]).output().map_err(|e| format!("{} not found: {}", refs[0], e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if out.status.success() { Ok(stdout) } else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
    }
}

// ── Node Gene ──
// Config: NODE_CMD (default: node), NODE_FLAGS
#[derive(Debug)]
pub struct NodeGene { m: GeneManifest }
impl Default for NodeGene { fn default() -> Self { Self::new() } }
impl NodeGene { pub fn new() -> Self { Self { m: mk("node", GeneKind::Tool) } } }
impl Gene for NodeGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: node <script.js> [args]".into()); }
        let mut node = std::env::var("NODE_CMD").unwrap_or_else(|_| "node".into());
        if let Ok(flags) = std::env::var("NODE_FLAGS") { node.push(' '); node.push_str(&flags); }
        let mut cmd = node.split_whitespace().map(String::from).collect::<Vec<_>>();
        cmd.extend(input.split_whitespace().map(String::from));
        let refs: Vec<&str> = cmd.iter().map(String::as_str).collect();
        let out = std::process::Command::new(refs[0]).args(&refs[1..]).output().map_err(|e| format!("{} not found: {}", refs[0], e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if out.status.success() { Ok(stdout) } else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
    }
}

// ── Java Gene ──
// Config: JAVA_CMD (default: java), JAVA_FLAGS, JAVA_CLASSPATH
#[derive(Debug)]
pub struct JavaGene { m: GeneManifest }
impl Default for JavaGene { fn default() -> Self { Self::new() } }
impl JavaGene { pub fn new() -> Self { Self { m: mk("java", GeneKind::Tool) } } }
impl Gene for JavaGene {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: java <class> [args]".into()); }
        let mut java = std::env::var("JAVA_CMD").unwrap_or_else(|_| "java".into());
        if let Ok(flags) = std::env::var("JAVA_FLAGS") { java.push(' '); java.push_str(&flags); }
        let mut cmd = java.split_whitespace().map(String::from).collect::<Vec<_>>();
        if let Ok(cp) = std::env::var("JAVA_CLASSPATH") { cmd.push("-cp".into()); cmd.push(cp); }
        cmd.extend(input.split_whitespace().map(String::from));
        let refs: Vec<&str> = cmd.iter().map(String::as_str).collect();
        let out = std::process::Command::new(refs[0]).args(&refs[1..]).output().map_err(|e| format!("{} not found: {}", refs[0], e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if out.status.success() { Ok(stdout) } else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
    }
}

// ── Evaluator Genes ──


pub trait Evaluator: Gene {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String>;
}

// ── Rust Tests Evaluator ──
#[derive(Debug)]
pub struct RustTestsEvaluator { m: GeneManifest }
impl Default for RustTestsEvaluator { fn default() -> Self { Self::new() } }
impl RustTestsEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-rust-tests", GeneKind::Tool) } }
}
impl Gene for RustTestsEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let dir = if input.trim().is_empty() { "." } else { input.trim() };
        {
        let out = std::process::Command::new("cargo")
            .args(&["test", "--manifest-path", &format!("{}/Cargo.toml", dir)])
            .output()
            .map_err(|e| format!("cargo: {}", e))?;
        if out.status.success() { Ok("Tests pass".into()) }
        else { Err(String::from_utf8_lossy(&out.stderr).to_string()) }
    }
    }
}
impl Evaluator for RustTestsEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains("test result: ok") { Ok(format!("Goal met: {}", goal)) }
        else { Err(format!("Goal NOT met: {}", goal)) }
    }
}

// ── Python Tests Evaluator ──
#[derive(Debug)]
pub struct PythonTestsEvaluator { m: GeneManifest }
impl Default for PythonTestsEvaluator { fn default() -> Self { Self::new() } }
impl PythonTestsEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-python-tests", GeneKind::Tool) } }
}
impl Gene for PythonTestsEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let dir = if input.trim().is_empty() { "." } else { input.trim() };
        {
        let out = std::process::Command::new("pytest")
            .args(&[&format!("{}/tests", dir)])
            .output()
            .map_err(|e| format!("pytest: {}", e))?;
        if out.status.success() { Ok("Tests pass".into()) }
        else { Err(String::from_utf8_lossy(&out.stderr).to_string()) }
    }
    }
}
impl Evaluator for PythonTestsEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains("passed") { Ok(format!("Goal met: {}", goal)) }
        else { Err(format!("Goal NOT met: {}", goal)) }
    }
}

// ── Output Match Evaluator ──
#[derive(Debug)]
pub struct OutputMatchEvaluator { m: GeneManifest }
impl Default for OutputMatchEvaluator { fn default() -> Self { Self::new() } }
impl OutputMatchEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-output-match", GeneKind::Tool) } }
}
impl Gene for OutputMatchEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        Ok(format!("Will check output contains: {}", input))
    }
}
impl Evaluator for OutputMatchEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains(goal) { Ok(format!("Output matches: {}", goal)) }
        else { Err(format!("Output does NOT contain: {}", goal)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_echo() {
        assert_eq!(ShellGene::new().execute("echo hi").unwrap().trim(), "hi");
    }

    #[test]
    fn python_math() {
        assert_eq!(
            PythonToolGene::new().execute("print(2+2)").unwrap().trim(),
            "4"
        );
    }

    #[test]
    fn workflow_steps() {
        let r = WorkflowGene::new().execute("echo a\necho b").unwrap();
        assert!(r.contains("step 1: a") && r.contains("step 2: b"));
    }

    #[test]
    fn all_have_ids() {
        let genes: [&dyn Gene; 24] = [
            &FilesystemGene::new(),
            &ShellGene::new(),
            &GitGene::new(),
            &HTTPGene::new(),
            &RustToolGene::new(),
            &PythonToolGene::new(),
            &WorkflowGene::new(),
            &DockerGene::new(),
            &DockerComposeGene::new(),
            &TerraformGene::new(),
            &KubectlGene::new(),
            &BrowserGene::new(),
            &SQLiteGene::new(),
            &GitHubGene::new(),
            &MCPGene::new(),
            &CodeReviewGene::new(),
            &BenchmarkGene::new(),
            &PostgresGene::new(),
            &GoGene::new(),
            &NodeGene::new(),
            &JavaGene::new(),
            &RustTestsEvaluator::new(),
            &PythonTestsEvaluator::new(),
            &OutputMatchEvaluator::new(),
        ];
        for g in &genes {
            assert!(!g.id().is_empty());
        }
    }
}
