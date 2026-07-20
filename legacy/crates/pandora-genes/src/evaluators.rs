//! Evaluator genes — domain-specific verification plugins.
//!
//! Each evaluator implements the Gene trait and is invoked by the
//! ExecutionController to determine whether execution succeeded.
//! This keeps the evaluation model consistent with the rest of Pandora:
//! evaluators ARE genes, not a separate engine.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind, desc: &str) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id).name(id).kind(kind).version("0.1.0").author("pandora")
        .description(desc).build().expect("hardcoded evaluator manifest must build")
}

fn run(args: &[&str]) -> Result<String, String> {
    let out = Command::new(args[0]).args(&args[1..]).output()
        .map_err(|e| format!("{} not found: {}", args[0], e))?;
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    if out.status.success() { Ok(stdout) }
    else { Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() }) }
}

/// Evaluator trait — adds verification to any gene.
/// An evaluator checks if execution output meets the goal.
pub trait Evaluator: Gene {
    /// Evaluate whether the output meets the goal.
    /// Returns Ok(pass) with details, or Err(fail) with reason.
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String>;
}

// ── Rust Tests Evaluator ──
// Verifies code by running cargo test. Config: CARGO_CMD, CARGO_FLAGS
#[derive(Debug)]
pub struct RustTestsEvaluator { m: GeneManifest, flags: String }
impl Default for RustTestsEvaluator { fn default() -> Self { Self::new() } }
impl RustTestsEvaluator {
    pub fn new() -> Self {
        Self {
            m: mk("evaluator-rust-tests", GeneKind::Tool,
                "Run cargo test to verify Rust code. Config: CARGO_CMD, CARGO_FLAGS"),
            flags: std::env::var("CARGO_FLAGS").unwrap_or_default(),
        }
    }
}
impl Gene for RustTestsEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let dir = if input.trim().is_empty() { "." } else { input.trim() };
        let cmd = std::env::var("CARGO_CMD").unwrap_or_else(|_| "cargo".into());
        let mut args = vec![cmd.as_str(), "test"];
        if !self.flags.is_empty() { args.extend(self.flags.split_whitespace()); }
        args.push("--manifest-path");
        args.push(&format!("{}/Cargo.toml", dir));
        let result = run(&args);
        match &result {
            Ok(_) => Ok("✓ Rust tests passed".into()),
            Err(e) => Err(format!("✗ Rust tests failed: {}", e)),
        }
    }
}
impl Evaluator for RustTestsEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains("test result: ok") {
            Ok(format!("✓ Goal met: {} (all tests pass)", goal))
        } else {
            Err(format!("✗ Goal not met: {}. Tests failed or never ran.", goal))
        }
    }
}

// ── Python Tests Evaluator ──
// Verifies code by running pytest. Config: PYTEST_CMD, PYTEST_FLAGS
#[derive(Debug)]
pub struct PythonTestsEvaluator { m: GeneManifest, flags: String }
impl Default for PythonTestsEvaluator { fn default() -> Self { Self::new() } }
impl PythonTestsEvaluator {
    pub fn new() -> Self {
        Self {
            m: mk("evaluator-python-tests", GeneKind::Tool,
                "Run pytest to verify Python code. Config: PYTEST_CMD, PYTEST_FLAGS"),
            flags: std::env::var("PYTEST_FLAGS").unwrap_or_default(),
        }
    }
}
impl Gene for PythonTestsEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let dir = if input.trim().is_empty() { "." } else { input.trim() };
        let cmd = std::env::var("PYTEST_CMD").unwrap_or_else(|_| "pytest".into());
        let mut args = vec![cmd.as_str()];
        if !self.flags.is_empty() { args.extend(self.flags.split_whitespace()); }
        args.push(&format!("{}/tests", dir));
        run(&args).map(|_| "✓ Python tests passed".into())
            .map_err(|e| format!("✗ Python tests failed: {}", e))
    }
}
impl Evaluator for PythonTestsEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains("passed") && !output.contains("failed") {
            Ok(format!("✓ Goal met: {} (pytest passed)", goal))
        } else {
            Err(format!("✗ Goal not met: {}. pytest output: {}", goal, output.lines().last().unwrap_or("")))
        }
    }
}

// ── Output Match Evaluator ──
// Verifies output contains expected string (simple goal check)
#[derive(Debug)]
pub struct OutputMatchEvaluator { m: GeneManifest }
impl Default for OutputMatchEvaluator { fn default() -> Self { Self::new() } }
impl OutputMatchEvaluator {
    pub fn new() -> Self {
        Self { m: mk("evaluator-output-match", GeneKind::Tool,
            "Check if output matches expected text") }
    }
}
impl Gene for OutputMatchEvaluator {
    fn id(&self) -> &str { &self.m.id }
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        if input.trim().is_empty() { return Err("Usage: <expected-text>".into()); }
        Ok(format!("Expecting output to contain: {}", input))
    }
}
impl Evaluator for OutputMatchEvaluator {
    fn evaluate(&self, output: &str, goal: &str) -> Result<String, String> {
        if output.contains(goal) {
            Ok(format!("✓ Goal met: output contains '{}'", goal))
        } else {
            Err(format!("✗ Goal not met: output does not contain '{}'", goal))
        }
    }
}

/// Register all built-in evaluators.
pub fn builtin_evaluators() -> Vec<Box<dyn Evaluator>> {
    vec![
        Box::new(RustTestsEvaluator::new()),
        Box::new(PythonTestsEvaluator::new()),
        Box::new(OutputMatchEvaluator::new()),
    ]
}
