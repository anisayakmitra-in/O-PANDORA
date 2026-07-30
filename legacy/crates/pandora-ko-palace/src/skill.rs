//! Skill System — skill.toml manifests that install multiple genes.

use crate::{KoPalace, Skill, SkillManifest};

pub fn discover(root: &str) -> Vec<SkillManifest> {
    let mut r = Vec::new();
    if let Ok(dir) = std::fs::read_dir(root) {
        for entry in dir.flatten() {
            let p = entry.path();
            if p.is_dir() && p.join("skill.toml").exists() {
                if let Ok(c) = std::fs::read_to_string(p.join("skill.toml")) {
                    if let Ok(m) = toml::from_str(&c) {
                        r.push(m);
                    }
                }
            }
        }
    }
    r
}

pub fn install(ko_palace: &mut KoPalace, path: &str) -> Result<Skill, pandora_types::PandoraError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot read skill: {e}")))?;
    let manifest: SkillManifest = toml::from_str(&content)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Invalid skill.toml: {e}")))?;
    for gene in &manifest.genes {
        match ko_palace.install(&gene.id) {
            Ok(_) => println!("  [gene] {}", gene.id),
            Err(e) => eprintln!("  [warn] {}: {e}", gene.id),
        }
    }
    println!("Installed skill: {} v{}", manifest.name, manifest.version);
    Ok(Skill { manifest })
}

pub fn scaffold(name: &str, dir: &str) -> Result<String, pandora_types::PandoraError> {
    let sd = std::path::Path::new(dir).join(name);
    std::fs::create_dir_all(&sd)
        .map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot create: {e}")))?;
    std::fs::write(sd.join("skill.toml"), format!("id = \"{name}\"\nname = \"{name}\"\nversion = \"0.2.0\"\nauthor = \"\"\ndescription = \"\"\n\n[[genes]]\nid = \"\"\nversion = \"\"\n")).map_err(|e| pandora_types::PandoraError::Internal(format!("Cannot write: {e}")))?;
    Ok(sd.to_string_lossy().to_string())
}
