//! Pandora Genes — all tool and evaluator genes.

use pandora_types::evaluation_verdict::EvaluationVerdict;
use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn capabilities_for(id: &str) -> &'static [&'static str] {
    match id {
        "filesystem" => &["filesystem", "filesystem.read", "filesystem.write"],
        "shell" => &["shell", "execution"],
        "git" => &["git", "vcs"],
        "http" => &["http", "network"],
        "rust-tool" => &["rust", "cargo", "compilation"],
        "python-tool" => &["python", "scripting"],
        "workflow" => &["workflow", "automation"],
        "docker" | "docker-compose" => &["docker", "containers"],
        "terraform" => &["terraform", "infrastructure"],
        "kubectl" => &["kubernetes", "deployment"],
        "browser" => &["browser", "web"],
        "sqlite" => &["sqlite", "database"],
        "github" => &["github", "devops"],
        "mcp" => &["mcp", "protocol"],
        "code-review" => &["code-review", "quality"],
        "benchmark" => &["benchmark", "performance"],
        "postgres" => &["postgres", "database"],
        "go" => &["go", "compilation"],
        "node" => &["node", "javascript"],
        "java" => &["java", "compilation"],
        id if id.starts_with("evaluator-") => &["evaluation", "quality"],
        _ => &[],
    }
}

fn mk(id: &str, kind: GeneKind) -> GeneManifest {
    let mut builder = GeneManifestBuilder::default()
        .id(id)
        .name(id)
        .kind(kind)
        .version(env!("CARGO_PKG_VERSION"))
        .author("pandora")
        .description(format!("{id} gene"))
        .permission(format!("{id}.execute"))
        .metadata("trust_level", "Official");
    for capability in capabilities_for(id) {
        builder = builder.capability(*capability);
    }
    builder.build().expect("genes")
}

fn run(bin: &str, args: &[&str]) -> Result<String, pandora_types::PandoraError> {
    let o = Command::new(bin)
        .args(args)
        .output()
        .map_err(|e| pandora_types::PandoraError::Internal(format!("{bin} not found: {e}")))?;
    let sd = String::from_utf8_lossy(&o.stderr).to_string();
    if o.status.success() {
        Ok(String::from_utf8_lossy(&o.stdout).to_string())
    } else {
        Err(if sd.is_empty() {
            format!("{bin} exit {}", o.status)
        } else {
            sd.trim().to_string()
        }
        .into())
    }
}

fn shell_output(input: &str) -> Result<std::process::Output, pandora_types::PandoraError> {
    #[cfg(windows)]
    {
        Command::new("cmd")
            .args(["/C", input])
            .output()
            .map_err(|e| pandora_types::PandoraError::Internal(format!("cmd not found: {e}")))
    }
    #[cfg(not(windows))]
    {
        Command::new("sh")
            .args(["-c", input])
            .output()
            .map_err(|e| pandora_types::PandoraError::Internal(format!("sh not found: {e}")))
    }
}

fn python_output(input: &str) -> Result<std::process::Output, pandora_types::PandoraError> {
    let candidates: &[(&str, &[&str])] = if cfg!(windows) {
        &[("python", &[]), ("py", &["-3"]), ("python3", &[])]
    } else {
        &[("python3", &[]), ("python", &[])]
    };

    for (program, prefix) in candidates {
        let mut command = Command::new(program);
        command.args(*prefix).args(["-c", input]);
        if let Ok(output) = command.output() {
            return Ok(output);
        }
    }

    Err("Python interpreter not found; install Python 3 and retry".into())
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
            fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
                let a: Vec<&str> = input.split_whitespace().collect();
                if a.is_empty() {
                    return Err(format!("Usage: {} <args>", $id).into());
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        let o = python_output(input)?;
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
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
                let canonical = std::fs::canonicalize(path)
                    .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
                if canonical.to_string_lossy().contains("..") {
                    return Err("Path traversal not allowed".into());
                }
                std::fs::read_to_string(&canonical)
                    .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))
            }
            "write" => {
                let raw_path = p
                    .get(1)
                    .copied()
                    .ok_or(pandora_types::PandoraError::NotFound(
                        "Missing path".to_string(),
                    ))?;
                if raw_path.is_empty() {
                    return Err("Missing path".into());
                }
                let path_obj = std::path::Path::new(raw_path);
                let canonical = std::fs::canonicalize(path_obj)
                    .or_else(|_| {
                        // File doesn't exist yet — create parent and canonicalize
                        if let Some(parent) = path_obj.parent() {
                            std::fs::create_dir_all(parent).ok();
                        }
                        std::fs::canonicalize(path_obj)
                    })
                    .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
                if canonical.to_string_lossy().contains("..") {
                    return Err("Path traversal not allowed".into());
                }
                std::fs::write(&canonical, "content")
                    .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
                Ok(format!("wrote {}", canonical.display()))
            }
            "list" => {
                let dir = std::fs::read_dir(p.get(1).unwrap_or(&"."))
                    .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
                Ok(dir
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"))
            }
            _ => Err(format!("Unknown: {}", p[0]).into()),
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        if std::env::var("PANDORA_SHELL_UNSAFE").is_err() {
            return Err(
                "Shell execution requires PANDORA_SHELL_UNSAFE=1 to acknowledge risks".into(),
            );
        }
        let o = shell_output(input)?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if sd.is_empty() {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        } else {
            Err(sd.into())
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        if std::env::var("PANDORA_SHELL_UNSAFE").is_err() {
            return Err(
                "Shell execution requires PANDORA_SHELL_UNSAFE=1 to acknowledge risks".into(),
            );
        }
        let mut r = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let o = shell_output(line)?;
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        if input.trim().is_empty() {
            return Err("Usage: mcp <package> [args]\nRequires Node.js (npx)".into());
        }
        let a: Vec<&str> = input.split_whitespace().collect();
        let mut c = Command::new("npx");
        c.arg("-y");
        for a in &a {
            c.arg(a);
        }
        let o = c.output().map_err(|e| {
            pandora_types::PandoraError::Internal(format!("npx not found: {e}. Install Node.js."))
        })?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if o.status.success() {
            Ok(String::from_utf8_lossy(&o.stdout).to_string())
        } else {
            Err(if sd.is_empty() {
                format!("npx exit {}", o.status)
            } else {
                sd.trim().to_string()
            }
            .into())
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        let o = Command::new("git")
            .arg("diff")
            .args(input.split_whitespace().collect::<Vec<_>>())
            .output()
            .map_err(|e| pandora_types::PandoraError::Internal(e.to_string()))?;
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
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        if input.trim().is_empty() {
            return Err("Usage: benchmark <command>".into());
        }
        if std::env::var("PANDORA_SHELL_UNSAFE").is_err() {
            return Err(
                "Shell execution requires PANDORA_SHELL_UNSAFE=1 to acknowledge risks".into(),
            );
        }
        let s = std::time::Instant::now();
        let o = shell_output(input)?;
        let sd = String::from_utf8_lossy(&o.stderr).to_string();
        if !o.status.success() {
            return Err(pandora_types::PandoraError::Internal(format!(
                "Exit {}: {}",
                o.status,
                sd.trim()
            )));
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
            fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
                if input.trim().is_empty() {
                    return Err(format!("Usage: {} <args>", $id).into());
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
                let o = Command::new(r[0]).args(&r[1..]).output().map_err(|e| {
                    pandora_types::PandoraError::Internal(format!("{} not found: {e}", r[0]))
                })?;
                let sd = String::from_utf8_lossy(&o.stderr).to_string();
                if o.status.success() {
                    Ok(String::from_utf8_lossy(&o.stdout).to_string())
                } else {
                    Err(if sd.is_empty() {
                        format!("exit {}", o.status)
                    } else {
                        sd.trim().to_string()
                    }
                    .into())
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
            fn execute(&self, i: &str) -> Result<String, pandora_types::PandoraError> {
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
    use std::ffi::OsString;
    use std::sync::Mutex;

    static SHELL_ENV_LOCK: Mutex<()> = Mutex::new(());

    struct EnvVarGuard {
        key: &'static str,
        previous: Option<OsString>,
    }

    impl EnvVarGuard {
        fn set(key: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(key);
            std::env::set_var(key, value);
            Self { key, previous }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                std::env::set_var(self.key, value);
            } else {
                std::env::remove_var(self.key);
            }
        }
    }

    #[test]
    fn shell_echo() {
        let _lock = SHELL_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvVarGuard::set("PANDORA_SHELL_UNSAFE", "1");
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
        let _lock = SHELL_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvVarGuard::set("PANDORA_SHELL_UNSAFE", "1");
        let r = WorkflowGene::new()
            .execute("echo a\necho b")
            .expect("genes");
        assert!(r.contains("step 1: a") && r.contains("step 2: b"));
    }
    #[test]
    fn benchmark_uses_platform_shell() {
        let _lock = SHELL_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvVarGuard::set("PANDORA_SHELL_UNSAFE", "1");
        assert!(BenchmarkGene::new()
            .execute("echo hi")
            .expect("genes")
            .contains("hi"));
    }
    #[test]
    fn builtins_have_routing_capabilities() {
        assert!(FilesystemGene::new()
            .manifest()
            .capabilities
            .contains(&"filesystem".to_string()));
        assert!(RustToolGene::new()
            .manifest()
            .capabilities
            .contains(&"compilation".to_string()));
        assert!(RustTestsEvaluator::new()
            .manifest()
            .capabilities
            .contains(&"evaluation".to_string()));
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
pub mod sandbox_gene;
pub mod skill_gene;
/// Construct the built-in genes shipped with Pandora.
pub fn builtin_genes() -> Vec<Box<dyn Gene>> {
    vec![
        Box::new(FilesystemGene::new()),
        Box::new(ShellGene::new()),
        Box::new(GitGene::new()),
        Box::new(HTTPGene::new()),
        Box::new(RustToolGene::new()),
        Box::new(PythonToolGene::new()),
        Box::new(WorkflowGene::new()),
        Box::new(DockerGene::new()),
        Box::new(DockerComposeGene::new()),
        Box::new(TerraformGene::new()),
        Box::new(KubectlGene::new()),
        Box::new(BrowserGene::new()),
        Box::new(SQLiteGene::new()),
        Box::new(GitHubGene::new()),
        Box::new(MCPGene::new()),
        Box::new(CodeReviewGene::new()),
        Box::new(BenchmarkGene::new()),
        Box::new(PostgresGene::new()),
        Box::new(GoGene::new()),
        Box::new(NodeGene::new()),
        Box::new(JavaGene::new()),
        Box::new(RustTestsEvaluator::new()),
        Box::new(PythonTestsEvaluator::new()),
        Box::new(OutputMatchEvaluator::new()),
        Box::new(DockerfileEvaluator::new()),
        Box::new(ShellCheckEvaluator::new()),
        Box::new(MarkdownLintEvaluator::new()),
    ]
}
