//! Pandora Genes — all tool and evaluator genes.

use pandora_types::evaluation_verdict::EvaluationVerdict;
use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id)
        .name(id)
        .kind(kind)
        .version("0.1.0")
        .author("pandora")
        .description(format!("{id} gene"))
        .build()
        .expect("genes")
}

fn run(bin: &str, args: &[&str]) -> Result<String, String> {
    let o = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| format!("{bin} not found: {e}"))?;
    let sd = String::from_utf8_lossy(&o.stderr).to_string();
    if o.status.success() {
        Ok(String::from_utf8_lossy(&o.stdout).to_string())
    } else {
        Err(if sd.is_empty() {
            format!("{bin} exit {}", o.status)
        } else {
            sd.trim().to_string()
        })
    }
}

macro_rules! cmd_gene {
    ($Struct:ident, $id:expr, $kind:expr, $bin:expr) => {
        #[derive(Debug)]
        pub struct $Struct {
            m: GeneManifest,
        }
        impl Default for $Struct {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $Struct {
            pub fn new() -> Self {
                Self { m: mk($id, $kind) }
            }
        }
        impl Gene for $Struct {
            fn manifest(&self) -> &GeneManifest {
                &self.m
            }
            fn execute(&self, input: &str) -> Result<String, String> {
                let a: Vec<&str> = input.split_whitespace().collect();
                if a.is_empty() {
                    return Err(format!("Usage: {} <args>", $id));
                }
                run($bin, &a)
            }
        }
    };
}

// Simple tool genes
cmd_gene!(GitGene, "git", GeneKind::Tool, "git");
cmd_gene!(HTTPGene, "http", GeneKind::Tool, "curl");
cmd_gene!(RustToolGene, "rust-tool", GeneKind::Tool, "cargo");
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
        let o = Command::new("python3")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&o.stdout).to_string())
    }
}
cmd_gene!(DockerGene, "docker", GeneKind::Tool, "docker");
cmd_gene!(
    DockerComposeGene,
    "docker-compose",
    GeneKind::Tool,
    "docker-compose"
);
cmd_gene!(TerraformGene, "terraform", GeneKind::Tool, "terraform");
cmd_gene!(KubectlGene, "kubectl", GeneKind::Tool, "kubectl");
cmd_gene!(BrowserGene, "browser", GeneKind::Tool, "curl");
cmd_gene!(SQLiteGene, "sqlite", GeneKind::Tool, "sqlite3");
cmd_gene!(GitHubGene, "github", GeneKind::Tool, "gh");

// ── Filesystem Gene ── (custom logic)
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
            "read" => {
                let path = p.get(1).copied().unwrap_or("");
                if path.is_empty() {
                    return Err("Missing path".into());
                }
                let canonical = std::fs::canonicalize(path).map_err(|e| e.to_string())?;
                if canonical.to_string_lossy().contains("..") {
                    return Err("Path traversal not allowed".into());
                }
                std::fs::read_to_string(&canonical).map_err(|e| e.to_string())
            }
            "write" => {
                let path = p.get(1).ok_or("Missing path")?;
                std::fs::write(path, "content").map_err(|e| e.to_string())?;
                Ok(format!("wrote {path}"))
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

// ── ShellGene ── (sh -c)
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
        let o = Command::new("sh")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| e.to_string())?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if sd.is_empty() {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        } else {
            Err(sd)
        }
    }
}

// ── Workflow Gene ──
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
            let o = Command::new("sh")
                .arg("-c")
                .arg(line)
                .output()
                .map_err(|e| e.to_string())?;
            r.push(format!(
                "step {}: {}",
                i + 1,
                String::from_utf8_lossy(&o.stdout).trim()
            ));
        }
        Ok(r.join("\n"))
    }
}

// ── MCP Gene ──
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
        let a: Vec<&str> = input.split_whitespace().collect();
        let mut c = Command::new("npx");
        c.arg("-y");
        for a in &a {
            c.arg(a);
        }
        let o = c
            .output()
            .map_err(|e| format!("npx not found: {e}. Install Node.js."))?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if o.status.success() {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        } else {
            Err(if sd.is_empty() {
                format!("npx exit {}", o.status)
            } else {
                sd.trim().to_string()
            })
        }
    }
}

// ── CodeReview Gene ──
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
        let o = Command::new("git")
            .arg("diff")
            .args(input.split_whitespace().collect::<Vec<_>>())
            .output()
            .map_err(|e| e.to_string())?;
        let d = String::from_utf8_lossy(&o.stdout);
        if d.is_empty() {
            return Ok("No changes.".into());
        }
        let l: Vec<&str> = d.lines().collect();
        Ok(format!(
            "{} lines, +{}/-{}",
            l.len(),
            l.iter().filter(|l| l.starts_with('+')).count(),
            l.iter().filter(|l| l.starts_with('-')).count()
        ))
    }
}

// ── Benchmark Gene ──
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
        if input.trim().is_empty() {
            return Err("Usage: benchmark <command>".into());
        }
        let s = std::time::Instant::now();
        let o = Command::new("sh")
            .arg("-c")
            .arg(input)
            .output()
            .map_err(|e| format!("Failed: {e}"))?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if !o.status.success() {
            return Err(format!("Exit {}: {}", o.status, sd.trim()));
        }
        Ok(format!(
            "{:?}\n{}",
            s.elapsed(),
            String::from_utf8_lossy(&o.stdout)
        ))
    }
}

// ── Env-configured Genes ──
macro_rules! env_gene {
    ($Struct:ident, $id:expr, $env:expr) => {
        #[derive(Debug)]
        pub struct $Struct {
            m: GeneManifest,
        }
        impl Default for $Struct {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $Struct {
            pub fn new() -> Self {
                Self {
                    m: mk($id, GeneKind::Tool),
                }
            }
        }
        impl Gene for $Struct {
            fn manifest(&self) -> &GeneManifest {
                &self.m
            }
            fn execute(&self, input: &str) -> Result<String, String> {
                if input.trim().is_empty() {
                    return Err(format!("Usage: {} <args>", $id));
                }
                let mut p: Vec<String> = std::env::var($env)
                    .unwrap_or_else(|_| $id.into())
                    .split_whitespace()
                    .map(String::from)
                    .collect();
                if let Ok(f) = std::env::var(concat!($env, "_FLAGS")) {
                    p.extend(f.split_whitespace().map(String::from));
                }
                p.extend(input.split_whitespace().map(String::from));
                let r: Vec<&str> = p.iter().map(String::as_str).collect();
                let o = Command::new(r[0])
                    .args(&r[1..])
                    .output()
                    .map_err(|e| format!("{} not found: {e}", r[0]))?;
                let sd = String::from_utf8_lossy(&o.stderr).to_string();
                if o.status.success() {
                    Ok(String::from_utf8_lossy(&o.stdout).to_string())
                } else {
                    Err(if sd.is_empty() {
                        format!("exit {}", o.status)
                    } else {
                        sd.trim().to_string()
                    })
                }
            }
        }
    };
}

env_gene!(PostgresGene, "postgres", "PG_CMD");
env_gene!(GoGene, "go", "GO_CMD");
env_gene!(NodeGene, "node", "NODE_CMD");
env_gene!(JavaGene, "java", "JAVA_CMD");

// ── Evaluator Genes ──

pub trait Evaluator: Gene {
    fn evaluate(&self, output: &str, goal: &str) -> EvaluationVerdict;
}

macro_rules! eval_gene {
    ($Struct:ident, $id:expr) => {
        #[derive(Debug)]
        pub struct $Struct {
            m: GeneManifest,
        }
        impl Default for $Struct {
            fn default() -> Self {
                Self::new()
            }
        }
        impl $Struct {
            pub fn new() -> Self {
                Self {
                    m: mk($id, GeneKind::Tool),
                }
            }
        }
        impl Gene for $Struct {
            fn manifest(&self) -> &GeneManifest {
                &self.m
            }
            fn execute(&self, i: &str) -> Result<String, String> {
                Ok(format!("eval: {i}"))
            }
        }
    };
}

eval_gene!(RustTestsEvaluator, "evaluator-rust-tests");
eval_gene!(OutputMatchEvaluator, "evaluator-output-match");
eval_gene!(DockerfileEvaluator, "evaluator-dockerfile");
eval_gene!(ShellCheckEvaluator, "evaluator-shellcheck");
eval_gene!(MarkdownLintEvaluator, "evaluator-markdownlint");
eval_gene!(PythonTestsEvaluator, "evaluator-python-tests");

impl Evaluator for RustTestsEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> EvaluationVerdict {
        if output.contains("passed") {
            EvaluationVerdict::pass(0.9)
        } else {
            EvaluationVerdict::fail(0.0, &format!("Goal NOT met: {output}"))
        }
    }
}
impl Evaluator for OutputMatchEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> EvaluationVerdict {
        if output.contains(goal) {
            EvaluationVerdict::pass(1.0)
        } else {
            EvaluationVerdict::fail(0.0, &format!("Output does NOT contain: {goal}"))
        }
    }
}
impl Evaluator for DockerfileEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> EvaluationVerdict {
        if output.contains("succeeded") {
            EvaluationVerdict::pass(1.0)
        } else {
            EvaluationVerdict::fail(0.0, "Docker build failed")
        }
    }
}
impl Evaluator for ShellCheckEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> EvaluationVerdict {
        if output.contains("no issues") {
            EvaluationVerdict::pass(1.0)
        } else {
            EvaluationVerdict::fail(0.5, &format!("ShellCheck issues: {output}"))
        }
    }
}
impl Evaluator for MarkdownLintEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> EvaluationVerdict {
        if output.contains("no issues") {
            EvaluationVerdict::pass(1.0)
        } else {
            EvaluationVerdict::fail(0.5, &format!("Markdown lint issues: {output}"))
        }
    }
}
impl Evaluator for PythonTestsEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> EvaluationVerdict {
        if output.contains("passed") {
            EvaluationVerdict::pass(0.9)
        } else {
            EvaluationVerdict::fail(0.0, &format!("Goal NOT met: {goal}"))
        }
    }
}

// ── Tests ──

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_echo() {
        assert_eq!(
            ShellGene::new().execute("echo hi").expect("genes").trim(),
            "hi"
        );
    }
    #[test]
    fn python_math() {
        assert_eq!(
            PythonToolGene::new()
                .execute("print(2+2)")
                .expect("genes")
                .trim(),
            "4"
        );
    }
    #[test]
    fn workflow_steps() {
        let r = WorkflowGene::new()
            .execute("echo a\necho b")
            .expect("genes");
        assert!(r.contains("step 1: a") && r.contains("step 2: b"));
    }
    #[test]
    fn all_have_ids() {
        let genes: [&dyn Gene; 27] = [
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
            &DockerfileEvaluator::new(),
            &ShellCheckEvaluator::new(),
            &MarkdownLintEvaluator::new(),
        ];
        for g in &genes {
            assert!(!g.id().is_empty());
        }
    }
}
pub mod code_graph;
