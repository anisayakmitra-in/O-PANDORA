//! Security Domain Harness — static analysis, dependency audit, secrets.

use pandora_types::gene::Gene;
use pandora_types::harness::{Harness, HarnessKind, HarnessManifest, HarnessManifestBuilder};
use std::process::Command;
use std::sync::Arc;

fn run(cmd: &str, args: &[&str]) -> Result<String, String> {
    let out = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("{cmd} not found: {e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(if stderr.is_empty() {
            format!("exit {}", out.status)
        } else {
            stderr.trim().to_string()
        })
    }
}

#[derive(Debug)]
pub struct SecurityDomainHarness {
    manifest: HarnessManifest,
    genes: Vec<Arc<dyn Gene>>,
}

impl SecurityDomainHarness {
    pub fn new() -> Self {
        Self {
            manifest: HarnessManifestBuilder::default()
                .id("security-domain")
                .name("Security")
                .version("0.2.0")
                .author("pandora")
                .kind(HarnessKind::Domain)
                .description("Security analysis — audit, scan, secrets, threat model")
                .capability("security-audit")
                .capability("dependency-scan")
                .capability("secrets-detection")
                .capability("static-analysis")
                .slash_command("/audit", "Run security audit: /audit [path]")
                .slash_command("/scan-deps", "Scan dependencies: /scan-deps [path]")
                .slash_command("/find-secrets", "Find secrets: /find-secrets [path]")
                .build()
                .unwrap(),
            genes: Vec::new(),
        }
    }

    pub fn audit(&self, dir: &str) -> Result<String, String> {
        run(
            "cargo",
            &["audit", "--manifest-path", &format!("{dir}/Cargo.toml")],
        )
        .or_else(|_| run("trivy", &["fs", "--format", "table", dir]))
    }

    pub fn scan_deps(&self, dir: &str) -> Result<String, String> {
        run(
            "cargo",
            &["audit", "--manifest-path", &format!("{dir}/Cargo.toml")],
        )
    }

    pub fn find_secrets(&self, dir: &str) -> Result<String, String> {
        run(
            "gitleaks",
            &["detect", "--source", dir, "--no-git", "--verbose"],
        )
    }

    pub fn add_gene(&mut self, gene: Arc<dyn Gene>) {
        self.genes.push(gene);
    }
    pub fn gene_count(&self) -> usize {
        self.genes.len()
    }
}

impl Default for SecurityDomainHarness {
    fn default() -> Self {
        Self::new()
    }
}
impl Harness for SecurityDomainHarness {
    fn manifest(&self) -> &HarnessManifest {
        &self.manifest
    }
    fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
}
