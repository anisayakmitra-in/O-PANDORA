//! Pandora Desktop — Ecosystem data helpers (Palace, Fleet, Scheduler)
//! Pure data functions, no Tauri macros. Wired from main.rs commands.

use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PalacePackage {
    pub id: String, pub name: String, pub version: String, pub kind: String,
    pub description: String, pub author: String, pub source: String,
    pub trust: String, pub installed: bool,
    pub permissions: Vec<String>, pub capabilities: Vec<String>,
}

#[derive(Serialize)]
pub struct FleetNode {
    pub id: String, pub name: String, pub platform: String, pub status: String,
    pub current_task: Option<String>, pub capabilities: Vec<String>, pub last_seen: String,
}

#[derive(Serialize)]
pub struct ScheduledJob {
    pub id: String, pub task: String, pub schedule: String, pub status: String,
    pub last_run: Option<String>, pub next_run: Option<String>, pub project: String,
}

pub fn seed_packages() -> Vec<PalacePackage> {
    vec![
        PalacePackage { id: "rust-gen".into(), name: "rust-gen".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Generate Rust code".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec![], capabilities: vec!["codegen".into()] },
        PalacePackage { id: "python-gen".into(), name: "python-gen".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Generate Python code".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec![], capabilities: vec!["codegen".into()] },
        PalacePackage { id: "code-review".into(), name: "code-review".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Review code for bugs and style".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec!["filesystem read".into()], capabilities: vec!["review".into()] },
        PalacePackage { id: "dep-audit".into(), name: "dep-audit".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Audit dependencies for vulnerabilities".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec!["network".into()], capabilities: vec!["security".into()] },
        PalacePackage { id: "secret-scan".into(), name: "secret-scan".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Scan for secrets and credentials".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec!["filesystem read".into()], capabilities: vec!["security".into()] },
        PalacePackage { id: "vuln-check".into(), name: "vuln-check".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Check for known vulnerabilities".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec!["network".into()], capabilities: vec!["security".into()] },
        PalacePackage { id: "web-search".into(), name: "web-search".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Search the web".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec!["network".into()], capabilities: vec!["search".into()] },
        PalacePackage { id: "summarize".into(), name: "summarize".into(), version: "0.1.0".into(), kind: "Gene".into(), description: "Summarize text and documents".into(), author: "pandora-project".into(), source: "palace".into(), trust: "verified".into(), installed: false, permissions: vec![], capabilities: vec!["text".into()] },
    ]
}
