use std::path::{Path, PathBuf};

pub fn workspace_root(project_path: Option<String>) -> PathBuf {
    project_path
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn validate_safe_name(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{label} cannot be empty"));
    }
    if value.starts_with('.') {
        return Err(format!("{label} cannot start with '.'"));
    }
    if value.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')) {
        Ok(())
    } else {
        Err(format!(
            "Invalid {label}: only ASCII letters, numbers, '.', '_' and '-' are allowed"
        ))
    }
}

pub fn canonicalize_existing(path: &Path) -> Result<PathBuf, String> {
    path.canonicalize()
        .map_err(|e| format!("Invalid path {}: {e}", path.display()))
}

pub fn canonicalize_with_missing(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return canonicalize_existing(path);
    }

    let mut missing = Vec::new();
    let mut current = path;
    while !current.exists() {
        let file_name = current
            .file_name()
            .ok_or_else(|| format!("Invalid path {}", path.display()))?;
        missing.push(file_name.to_os_string());
        current = current
            .parent()
            .ok_or_else(|| format!("Invalid path {}", path.display()))?;
    }

    let mut resolved = canonicalize_existing(current)?;
    while let Some(component) = missing.pop() {
        resolved.push(component);
    }
    Ok(resolved)
}

pub fn resolve_rooted_path(root: &Path, input: &str) -> Result<PathBuf, String> {
    let root = canonicalize_existing(root)?;
    let candidate = if Path::new(input).is_absolute() {
        PathBuf::from(input)
    } else {
        root.join(input)
    };
    let resolved = canonicalize_with_missing(&candidate)?;
    if resolved.starts_with(&root) {
        Ok(resolved)
    } else {
        Err(format!("Path escapes project root: {input}"))
    }
}

pub fn read_dir_entries(
    dir: &Path,
    root: &Path,
    max_depth: usize,
) -> Result<Vec<crate::FileEntry>, String> {
    if max_depth == 0 {
        return Ok(vec![]);
    }

    let root = canonicalize_existing(root)?;
    let dir = canonicalize_existing(dir)?;
    if !dir.starts_with(&root) {
        return Err(format!("Path escapes project root: {}", dir.display()));
    }

    let mut entries = vec![];
    if let Ok(read_dir) = std::fs::read_dir(&dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            if name.starts_with('.') || name == "target" || name == "node_modules" {
                continue;
            }
            let resolved = match canonicalize_existing(&path) {
                Ok(path) => path,
                Err(_) => continue,
            };
            if !resolved.starts_with(&root) {
                continue;
            }
            let is_dir = resolved.is_dir();
            entries.push(crate::FileEntry {
                name,
                path: resolved.to_string_lossy().to_string(),
                is_dir,
                children: if is_dir && max_depth > 1 {
                    Some(read_dir_entries(&resolved, &root, max_depth - 1).unwrap_or_default())
                } else {
                    None
                },
            });
        }
    }
    entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.cmp(&b.name)
        }
    });
    Ok(entries)
}
