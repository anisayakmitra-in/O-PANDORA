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

#[derive(Debug)]
pub struct DockerGene {
    m: GeneManifest,
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
        let out = std::process::Command::new("docker").args(&args).output()
            .map_err(|e| format!("docker not found: {}. Install Docker first.", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() { format!("docker exit {}", out.status) } else { stderr.trim().to_string() });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct BrowserGene {
    m: GeneManifest,
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
            return Err("Usage: gh <command>\nExample: gh pr list\nExample: gh issue view 123".into());
        }
        let args: Vec<&str> = input.split_whitespace().collect();
        let out = std::process::Command::new("gh").args(&args).output()
            .map_err(|e| format!("gh not found: {}. Install GitHub CLI (https://cli.github.com).", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() { format!("gh exit {}", out.status) } else { stderr.trim().to_string() });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct MCPGene {
    m: GeneManifest,
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
        for a in &args { cmd.arg(a); }
        let out = cmd.output()
            .map_err(|e| format!("npx not found: {}. Install Node.js.", e))?;
        let stdout = String::from_utf8_lossy(&out.stdout).to_string();
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        if !out.status.success() {
            return Err(if stderr.is_empty() { format!("npx exit {}", out.status) } else { stderr.trim().to_string() });
        }
        Ok(stdout)
    }
}

#[derive(Debug)]
pub struct CodeReviewGene {
    m: GeneManifest,
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
        let out = std::process::Command::new("sh").arg("-c").arg(input).output()
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
        let genes: [&dyn Gene; 14] = [
            &FilesystemGene::new(),
            &ShellGene::new(),
            &GitGene::new(),
            &HTTPGene::new(),
            &RustToolGene::new(),
            &PythonToolGene::new(),
            &WorkflowGene::new(),
            &DockerGene::new(),
            &BrowserGene::new(),
            &SQLiteGene::new(),
            &GitHubGene::new(),
            &MCPGene::new(),
            &CodeReviewGene::new(),
            &BenchmarkGene::new(),
        ];
        for g in &genes {
            assert!(!g.id().is_empty());
        }
    }
}
