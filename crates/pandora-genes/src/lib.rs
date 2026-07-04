//! First-party Genes — each implements the Gene trait.

use pandora_types::gene::{Gene, GeneKind, GeneManifest, GeneManifestBuilder};
use std::process::Command;

fn mk(id: &str, kind: GeneKind) -> GeneManifest {
    GeneManifestBuilder::default()
        .id(id).name(id).kind(kind).version("0.1.0").author("pandora")
        .description(format!("{} gene", id))
        .build().unwrap()
}

// ── FilesystemGene ──
#[derive(Debug)]
pub struct FilesystemGene { m: GeneManifest }
impl FilesystemGene {
    pub fn new() -> Self { Self { m: mk("filesystem", GeneKind::Tool) } }
}
impl Gene for FilesystemGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let p: Vec<&str> = input.splitn(2, ' ').collect();
        if p.is_empty() || p[0] == "help" { return Ok("Usage: read|write|list <path>".into()); }
        match p[0] {
            "read" => std::fs::read_to_string(p.get(1).unwrap_or(&"")).map_err(|e| e.to_string()),
            "write" => { let path = p.get(1).ok_or("Missing path")?; std::fs::write(path, "content").map_err(|e| e.to_string())?; Ok(format!("wrote {}", path)) }
            "list" => { let dir = std::fs::read_dir(p.get(1).unwrap_or(&".")).map_err(|e| e.to_string())?; Ok(dir.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).collect::<Vec<_>>().join("\n")) }
            _ => Err(format!("Unknown: {}", p[0])),
        }
    }
}

// ── ShellGene ──
#[derive(Debug)]
pub struct ShellGene { m: GeneManifest }
impl ShellGene {
    pub fn new() -> Self { Self { m: mk("shell", GeneKind::Tool) } }
}
impl Gene for ShellGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("sh").arg("-c").arg(input).output().map_err(|e| e.to_string())?;
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        if !err.is_empty() { return Err(err); }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

// ── GitGene ──
#[derive(Debug)]
pub struct GitGene { m: GeneManifest }
impl GitGene {
    pub fn new() -> Self { Self { m: mk("git", GeneKind::Tool) } }
}
impl Gene for GitGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("git").args(input.split_whitespace()).output().map_err(|e| e.to_string())?;
        if !out.status.success() { return Err(String::from_utf8_lossy(&out.stderr).to_string()); }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

// ── HTTPGene ──
#[derive(Debug)]
pub struct HTTPGene { m: GeneManifest }
impl HTTPGene {
    pub fn new() -> Self { Self { m: mk("http", GeneKind::Tool) } }
}
impl Gene for HTTPGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("curl").arg("-s").args(input.split_whitespace()).output().map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

// ── RustToolGene ──
#[derive(Debug)]
pub struct RustToolGene { m: GeneManifest }
impl RustToolGene {
    pub fn new() -> Self { Self { m: mk("rust-tool", GeneKind::Tool) } }
}
impl Gene for RustToolGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("cargo").args(input.split_whitespace()).output().map_err(|e| e.to_string())?;
        let err = String::from_utf8_lossy(&out.stderr).to_string();
        if !err.is_empty() { return Err(err); }
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

// ── PythonToolGene ──
#[derive(Debug)]
pub struct PythonToolGene { m: GeneManifest }
impl PythonToolGene {
    pub fn new() -> Self { Self { m: mk("python-tool", GeneKind::Tool) } }
}
impl Gene for PythonToolGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let out = Command::new("python3").arg("-c").arg(input).output().map_err(|e| e.to_string())?;
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

// ── WorkflowGene ──
#[derive(Debug)]
pub struct WorkflowGene { m: GeneManifest }
impl WorkflowGene {
    pub fn new() -> Self { Self { m: mk("workflow", GeneKind::Workflow) } }
}
impl Gene for WorkflowGene {
    fn manifest(&self) -> &GeneManifest { &self.m }
    fn execute(&self, input: &str) -> Result<String, String> {
        let mut r = Vec::new();
        for (i, line) in input.lines().enumerate() {
            let out = Command::new("sh").arg("-c").arg(line).output().map_err(|e| e.to_string())?;
            r.push(format!("step {}: {}", i + 1, String::from_utf8_lossy(&out.stdout).trim()));
        }
        Ok(r.join("\n"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_echo() { assert_eq!(ShellGene::new().execute("echo hi").unwrap().trim(), "hi"); }

    #[test]
    fn filesystem_list() { assert!(FilesystemGene::new().execute("list .").unwrap().contains("Cargo.toml")); }

    #[test]
    fn python_math() { assert_eq!(PythonToolGene::new().execute("print(2+2)").unwrap().trim(), "4"); }

    #[test]
    fn workflow_steps() {
        let r = WorkflowGene::new().execute("echo a\necho b").unwrap();
        assert!(r.contains("step 1: a") && r.contains("step 2: b"));
    }

    #[test]
    fn all_genes_have_ids() {
        let genes: [&dyn Gene; 7] = [
            &FilesystemGene::new(), &ShellGene::new(), &GitGene::new(),
            &HTTPGene::new(), &RustToolGene::new(), &PythonToolGene::new(),
            &WorkflowGene::new(),
        ];
        for g in &genes { assert!(!g.id().is_empty()); }
    }
}
