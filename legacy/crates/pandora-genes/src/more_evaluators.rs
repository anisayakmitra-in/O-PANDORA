// Evaluator genes added to the builtin set. These are appended to lib.rs.

// ── Dockerfile Evaluator ──
// Verifies Dockerfile builds. Config: DOCKER_CMD
#[derive(Debug)]
pub struct DockerfileEvaluator { m: GeneManifest }
impl Default for DockerfileEvaluator { fn default() -> Self { Self::new() } }
impl DockerfileEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-dockerfile", GeneKind::Tool) } }
}
impl Gene for DockerfileEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        let docker = std::env::var("DOCKER_CMD").unwrap_or_else(|_| "docker".into());
        Command::new(&docker).args(&["build", "-q", input]).output()
            .map_err(|e| pandora_types::PandoraError::Internal(format!("{}: {}", docker, e))).and_then(|o| {
                if o.status.success() { Ok("Docker build succeeded".into()) }
                else { Err(String::from_utf8_lossy(&o.stderr).to_string()) }
            })
    }
}
impl Evaluator for DockerfileEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> Result<String, pandora_types::PandoraError> {
        if output.contains("succeeded") { Ok("Docker build verified".into()) }
        else { Err("Docker build failed".into()) }
    }
}

// ── ShellCheck Evaluator ──
// Verifies shell scripts. Requires: shellcheck
#[derive(Debug)]
pub struct ShellCheckEvaluator { m: GeneManifest }
impl Default for ShellCheckEvaluator { fn default() -> Self { Self::new() } }
impl ShellCheckEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-shellcheck", GeneKind::Tool) } }
}
impl Gene for ShellCheckEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        Command::new("shellcheck").arg(input).output()
            .map_err(|e| pandora_types::PandoraError::Internal(format!("shellcheck: {}", e))).and_then(|o| {
                if o.status.success() { Ok("ShellCheck: no issues".into()) }
                else { Err(String::from_utf8_lossy(&o.stderr).to_string()) }
            })
    }
}
impl Evaluator for ShellCheckEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> Result<String, pandora_types::PandoraError> {
        if output.contains("no issues") { Ok("Shell scripts verified".into()) }
        else { Err(output.into()) }
    }
}

// ── MarkdownLint Evaluator ──
// Verifies markdown files. Requires: markdownlint or mdl
#[derive(Debug)]
pub struct MarkdownLintEvaluator { m: GeneManifest }
impl Default for MarkdownLintEvaluator { fn default() -> Self { Self::new() } }
impl MarkdownLintEvaluator {
    pub fn new() -> Self { Self { m: mk("evaluator-markdownlint", GeneKind::Tool) } }
}
impl Gene for MarkdownLintEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, pandora_types::PandoraError> {
        let result = Command::new("markdownlint").arg(input).output();
        match result {
            Ok(o) if o.status.success() => Ok("Markdown: no issues".into()),
            Ok(o) => Err(String::from_utf8_lossy(&o.stderr).to_string()),
            Err(_) => {
                // Fallback to mdl
                Command::new("mdl").arg(input).output()
                    .map_err(|e| pandora_types::PandoraError::Internal(format!("markdownlint/mdl: {}", e))).and_then(|o| {
                        if o.status.success() { Ok("Markdown: no issues".into()) }
                        else { Err(String::from_utf8_lossy(&o.stderr).to_string()) }
                    })
            }
        }
    }
}
impl Evaluator for MarkdownLintEvaluator {
    fn evaluate(&self, output: &str, _goal: &str) -> Result<String, pandora_types::PandoraError> {
        if output.contains("no issues") { Ok("Markdown verified".into()) }
        else { Err(output.into()) }
    }
}
