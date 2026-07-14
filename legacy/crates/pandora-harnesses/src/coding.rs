//! Coding Domain Harness — developer workflow domain.

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use std::process::Command;
use std::sync::Arc;

fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd).args(args).output().map_err(|e| format!("{cmd} not found: {e}"))?;
    if out.status.success() { Ok(String::from_utf8_lossy(&out.stdout).to_string()) }
    else { let s = String::from_utf8_lossy(&out.stderr).to_string(); Err(if s.is_empty() { format!("{cmd} exit {}", out.status) } else { s.trim().to_string() }) }
}

fn sh(cmd: &str) -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg(cmd).output().map_err(|e| format!("shell failed: {e}"))?;
    if out.status.success() { Ok(String::from_utf8_lossy(&out.stdout).to_string()) }
    else { let s = String::from_utf8_lossy(&out.stderr).to_string(); Err(if s.is_empty() { format!("exit {}", out.status) } else { s.trim().to_string() }) }
}

#[derive(Debug)]
pub struct CodingDomainHarness { manifest: HarnessManifest, genes: Vec<Arc<dyn Gene>>, tool_map: Vec<(&'static str, &'static str)> }

impl CodingDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("coding-domain").name("Coding").version("0.2.0").author("pandora").kind(HarnessKind::Domain)
                .description("Developer workflow — code, review, build, test, simplify")
                .capability("code-review").capability("build").capability("test").capability("lint")
                .capability("simplify").capability("audit").capability("coding")
                .slash_command("/build", "Build project: /build [dir]")
                .slash_command("/test", "Run tests: /test [filter]")
                .slash_command("/lint", "Run linter: /lint [path]")
                .slash_command("/review", "Review diff: /review [base]")
                .slash_command("/simplify", "Suggest simplifications: /simplify [path]")
                .slash_command("/audit", "Full over-engineering audit: /audit [path]").build().unwrap(),
            genes: Vec::new(),
            tool_map: vec![("rust", "cargo"), ("python", "python3"), ("node", "node"), ("make", "make"), ("go", "go")],
        }
    }

    fn detect_language(&self, dir: &str) -> &'static str {
        for &(lang, _) in &self.tool_map {
            let m = match lang {
                "rust" => "Cargo.toml", "python" => "setup.py", "node" => "package.json",
                "make" => "Makefile", "go" => "go.mod", _ => continue,
            };
            if std::path::Path::new(dir).join(m).exists() { return lang; }
        }
        "unknown"
    }

    pub fn build(&self, dir: &str) -> Result<String, String> {
        match self.detect_language(dir) {
            "rust" => run("cargo", &["build", "--manifest-path", &format!("{dir}/Cargo.toml")]),
            "python" => run("python3", &["-m", "build", dir]),
            "node" => run("npm", &["run", "build", "--prefix", dir]),
            "make" => run("make", &["-C", dir]),
            "go" => run("go", &["build", "./..."]),
            l => Err(format!("No supported build system found for {l}")),
        }
    }

    pub fn test(&self, dir: &str, filter: Option<&str>) -> Result<String, String> {
        match self.detect_language(dir) {
            "rust" => {
                let mp = format!("{dir}/Cargo.toml"); let mut a = vec!["test", "--manifest-path", &mp];
                if let Some(f) = filter { a.push("--"); a.push(f); }
                run("cargo", &a)
            }
            "python" => run("python3", &["-m", "pytest", dir]),
            "node" => run("npm", &["test", "--prefix", dir]),
            "make" => run("make", &["test", "-C", dir]),
            "go" => run("go", &["test", "./..."]),
            _ => Err("No test framework detected".into()),
        }
    }

    pub fn lint(&self, dir: &str) -> Result<String, String> {
        match self.detect_language(dir) {
            "rust" => run("cargo", &["clippy", "--workspace", "--manifest-path", &format!("{dir}/Cargo.toml"), "--", "-D", "warnings"]),
            "python" => run("ruff", &["check", dir]),
            "node" => run("npx", &["eslint", dir]),
            l => Err(format!("No linter for {l} project")),
        }
    }

    pub fn review(&self, base: &str) -> Result<String, String> { sh(&format!("git diff {base} 2>/dev/null || git diff --staged")) }

    pub fn simplify_suggestions(&self, dir: &str) -> Result<Vec<String>, String> {
        let mut findings = Vec::new();
        let cargo = format!("{dir}/Cargo.toml");
        if std::path::Path::new(&cargo).exists() {
            let c = std::fs::read_to_string(&cargo).map_err(|e| format!("Cannot read {cargo}: {e}"))?;
            for &dep in &["chrono", "regex", "lazy_static", "once_cell", "thiserror", "anyhow"] {
                if c.contains(&format!("{dep} ")) || c.contains(&format!("{dep}/")) {
                    let r = match dep {
                        "chrono" => "std::time::SystemTime", "regex" => "string methods",
                        "lazy_static" => "std::sync::LazyLock", "once_cell" => "std::sync::OnceLock",
                        "thiserror" => "manual Display impl", "anyhow" => "concrete error type", _ => "stdlib equivalent",
                    };
                    findings.push(format!("  {dep}: replace with {r} (ponytail)"));
                }
            }
        }
        Ok(findings)
    }

    pub fn audit(&self, dir: &str) -> Result<String, String> {
        let lang = self.detect_language(dir);
        let mut r = format!("Ponytail Audit: {dir}\n{}\n Language: {lang}\n\n", "─".repeat(50));
        let s = self.simplify_suggestions(dir)?;
        if s.is_empty() { r.push_str(" No obvious over-engineering found.\n"); }
        else { r.push_str(&format!(" {} simplification(s):\n", s.len())); for f in s { r.push_str(&f); r.push('\n'); } }
        r.push('\n');
        let f = sh(&format!("find {dir} -name '*.rs' -not -path '*/target/*' 2>/dev/null | wc -l")).unwrap_or_default();
        r.push_str(&format!(" Rust source files: {} (consider if all are needed)\n", f.trim()));
        Ok(r)
    }

    pub fn add_gene(&mut self, gene: Arc<dyn Gene>) { self.genes.push(gene); }
    pub fn gene_count(&self) -> usize { self.genes.len() }
}

impl Default for CodingDomainHarness { fn default() -> Self { Self::new() } }
impl Harness for CodingDomainHarness { fn manifest(&self) -> &HarnessManifest { &self.manifest } fn initialize(&mut self) -> Result<(), String> { Ok(()) } fn shutdown(&mut self) -> Result<(), String> { Ok(()) } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn detect_rust() { assert_eq!(CodingDomainHarness::new().detect_language(&std::env::current_dir().unwrap().to_string_lossy()), "rust"); }
    #[test] fn manifest_is_set() { let h = CodingDomainHarness::new(); assert_eq!(h.manifest().id, "coding-domain"); }
    #[test] fn lint_self() { let h = CodingDomainHarness::new(); let r = h.lint(env!("CARGO_MANIFEST_DIR")); if let Err(ref m) = r { assert!(!m.is_empty()); } }
}
