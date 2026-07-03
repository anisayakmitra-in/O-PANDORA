//! Pandora Repository Indexer — extracted from pandora-runtime (Phase 1A).
//!
use serde::{Deserialize, Serialize};

use std::{fs, path::Path};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,

    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryIndex {
    pub files: Vec<IndexedFile>,

    pub total_files: usize,
}

pub struct RepositoryIndexer;

impl RepositoryIndexer {
    pub fn index(root: &str) -> RepositoryIndex {
        let mut indexed = Vec::new();

        Self::walk(Path::new(root), &mut indexed);

        RepositoryIndex {
            total_files: indexed.len(),

            files: indexed,
        }
    }

    fn walk(path: &Path, indexed: &mut Vec<IndexedFile>) {
        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    Self::walk(&entry.path(), indexed);
                }
            }
        } else {
            if let Ok(metadata) = fs::metadata(path) {
                println!("[INDEXER] indexed {}", path.display());

                indexed.push(IndexedFile {
                    path: path.display().to_string(),

                    size: metadata.len(),
                });
            }
        }
    }
}
