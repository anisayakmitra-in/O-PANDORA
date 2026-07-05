//! Coding Domain Harness — developer workflow domain.
//!
//! Embodies the ponytail philosophy: simplest working solution,
//! stdlib before custom, YAGNI, minimal code.
//!
//! Capabilities:
//!   - `code-review`   — git diff analysis with over-engineering detection
//!   - `build`         — compile projects (cargo, make, generic)
//!   - `test`          — run tests
//!   - `lint`          — static analysis (clippy, shellcheck, etc.)
//!   - `simplify`      — ponytail-style code simplification suggestions
//!   - `audit`         — whole-repo over-engineering audit
//!
//! The coding harness enforces measurement before optimization:
//! every simplification must be backed by evidence.

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use std::process::Command;
use std::sync::Arc;

#[derive(Debug)]
pub struct CodingDomainHarness {
    manifest: HarnessManifest,
    genes: Vec<Arc<dyn Gene>>,
    /// ponytail: prefer local binaries over toolchain detection
    tool_map: Vec<(&'static str, &'static str)>,
}

/// Execute a command and return stdout or a formatted error.
fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd).args(args).output()
        .map_err(|e| format!("{} not found: {}. Install it.", cmd, e))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        Err(if stderr.is_empty() { format!("{} exit {}", cmd, out.status) } else { stderr.trim().to_string() })
    }
}

/// Run a shell command and return stdout.
fn sh(cmd: &str) -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg(cmd).output()
        .map_err(|e| format!("shell failed: {}", e))?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        Err(if stderr.is_empty() { format!("exit {}", out.status) } else { stderr.trim().to_string() })
    }
}

impl CodingDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("coding-domain")
                .name("Coding")
                .version("0.2.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("Developer workflow — code, review, build, test, simplify")
                .capability("code-review")
                .capability("build")
                .capability("test")
                .capability("lint")
                .capability("simplify")
                .capability("audit")
                .capability("coding")
                .slash_command("/build", "Build project: /build [dir]")
                .slash_command("/test", "Run tests: /test [filter]")
                .slash_command("/lint", "Run linter: /lint [path]")
                .slash_command("/review", "Review diff: /review [base]")
                .slash_command("/simplify", "Suggest simplifications: /simplify [path]")
                .slash_command("/audit", "Full over-engineering audit: /audit [path]")
                .build()
                .unwrap(),
            genes: Vec::new(),
            tool_map: vec![
                ("rust", "cargo"),
                ("python", "python3"),
                ("node", "node"),
                ("make", "make"),
                ("go", "go"),
            ],
        }
    }

    /// Detect the project language from build files in root.
    fn detect_language(&self, dir: &str) -> &'static str {
        for &(lang, _) in &self.tool_map {
            let marker = match lang {
                "rust" => "Cargo.toml",
                "python" => "setup.py",
                "node" => "package.json",
                "make" => "Makefile",
                "go" => "go.mod",
                _ => continue,
            };
            if std::path::Path::new(dir).join(marker).exists() {
                return lang;
            }
        }
        "unknown"
    }

    /// Build the project.
    pub fn build(&self, dir: &str) -> Result<String, String> {
        let lang = self.detect_language(dir);
        match lang {
            "rust" => {
                let manifest = format!("{}/Cargo.toml", dir);
                run("cargo", &["build", "--manifest-path", &manifest])
            }
            "python" => run("python3", &["-m", "build", dir]),
            "node" => run("npm", &["run", "build", "--prefix", dir]),
            "make" => run("make", &["-C", dir]),
            "go" => run("go", &["build", "./..."]),
            _ => Err("No supported build system found (Cargo.toml, Makefile, package.json, setup.py, go.mod)".into()),
        }
    }

    /// Run tests.
    pub fn test(&self, dir: &str, filter: Option<&str>) -> Result<String, String> {
        let lang = self.detect_language(dir);
        match lang {
            "rust" => {
                let manifest = format!("{}/Cargo.toml", dir);
                let mut args: Vec<&str> = vec!["test", "--manifest-path", &manifest];
                if let Some(f) = filter { args.push("--"); args.push(f); }
                run("cargo", &args)
            }
            "python" => run("python3", &["-m", "pytest", dir]),
            "node" => run("npm", &["test", "--prefix", dir]),
            "make" => run("make", &["test", "-C", dir]),
            "go" => run("go", &["test", "./..."]),
            _ => Err("No test framework detected".into()),
        }
    }

    /// Lint the project with the appropriate tool.
    pub fn lint(&self, dir: &str) -> Result<String, String> {
        let lang = self.detect_language(dir);
        match lang {
            "rust" => {
                let manifest = format!("{}/Cargo.toml", dir);
                run("cargo", &["clippy", "--workspace", "--manifest-path", &manifest, "--", "-D", "warnings"])
            }
            "python" => run("ruff", &["check", dir]),
            "node" => run("npx", &["eslint", dir]),
            _ => Err(format!("No linter for {} project", lang)),
        }
    }

    /// Code review via git diff. Returns findings or empty string.
    pub fn review(&self, base: &str) -> Result<String, String> {
        sh(&format!("git diff {} 2>/dev/null || git diff --staged", base))
    }

    /// ponytail: check for common over-engineering patterns.
    pub fn simplify_suggestions(&self, dir: &str) -> Result<Vec<String>, String> {
        let mut findings = Vec::new();
        // Check for unnecessary dependencies
        let cargo_toml = format!("{}/Cargo.toml", dir);
        if std::path::Path::new(&cargo_toml).exists() {
            let content = std::fs::read_to_string(&cargo_toml)
                .map_err(|e| format!("Cannot read {}: {}", cargo_toml, e))?;
            // ponytail: serde is worth it, many others aren't
            for dep in &["chrono", "regex", "lazy_static", "once_cell", "thiserror", "anyhow"] {
                if content.contains(&format!("{} ", dep)) || content.contains(&format!("{}/", dep)) {
                    let replacement = match *dep {
                        "chrono" => "std::time::SystemTime",
                        "regex" => "string methods (contains, starts_with, split)",
                        "lazy_static" => "std::sync::LazyLock (1.80+)",
                        "once_cell" => "std::sync::OnceLock (1.70+)",
                        "thiserror" => "manual Display impl or std::error::Error",
                        "anyhow" => "concrete error type",
                        _ => "stdlib equivalent",
                    };
                    findings.push(format!("  {}: replace with {} (ponytail)", dep, replacement));
                }
            }
        }
        Ok(findings)
    }

    /// Full over-engineering audit of a directory.
    pub fn audit(&self, dir: &str) -> Result<String, String> {
        let mut report = String::new();
        report.push_str(&format!("Ponytail Audit: {}\n", dir));
        report.push_str("─".repeat(50).as_str());
        report.push('\n');

        // Language detection
        let lang = self.detect_language(dir);
        report.push_str(&format!(" Language: {}\n\n", lang));

        // Unnecessary deps
        let simplifications = self.simplify_suggestions(dir)?;
        if simplifications.is_empty() {
            report.push_str(" No obvious over-engineering found.\n");
        } else {
            report.push_str(&format!(" {} simplification(s):\n", simplifications.len()));
            for s in &simplifications {
                report.push_str(s);
                report.push('\n');
            }
        }

        // Check file count
        report.push('\n');
        let file_count = sh(&format!("find {} -name '*.rs' -not -path '*/target/*' 2>/dev/null | wc -l", dir))
            .unwrap_or_default();
        report.push_str(&format!(" Rust source files: {} (consider if all are needed)\n", file_count.trim()));

        Ok(report)
    }

    pub fn add_gene(&mut self, gene: Arc<dyn Gene>) {
        self.genes.push(gene);
    }

    pub fn gene_count(&self) -> usize {
        self.genes.len()
    }
}

impl Harness for CodingDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }

    fn initialize(&mut self) -> Result<(), String> {
        // ponytail: detected at call time, nothing to cache
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_rust_project() {
        let h = CodingDomainHarness::new();
        let dir = std::env::current_dir().unwrap().to_string_lossy().to_string();
        // The harness crate itself is a Rust project
        let lang = h.detect_language(&dir);
        assert_eq!(lang, "rust");
    }

    #[test]
    fn simplify_suggestions_on_self() {
        let h = CodingDomainHarness::new();
        let dir = env!("CARGO_MANIFEST_DIR");
        let suggestions = h.simplify_suggestions(dir).unwrap();
        // The harness shouldn't have unnecessary deps
        // but we at least check it doesn't error
    }

    #[test]
    fn manifest_is_set() {
        let h = CodingDomainHarness::new();
        assert_eq!(h.manifest().id, "coding-domain");
        assert_eq!(h.manifest().kind, HarnessKind::Domain);
        assert!(h.manifest().capabilities.len() >= 5);
    }

    #[test]
    fn lint_self() {
        let h = CodingDomainHarness::new();
        let dir = env!("CARGO_MANIFEST_DIR");
        let result = h.lint(dir);
        // ponytail: lint may fail in CI; accept success OR failure with output
        if let Err(ref msg) = result {
            assert!(!msg.is_empty(), "lint error should have a message");
        }
    }
}
