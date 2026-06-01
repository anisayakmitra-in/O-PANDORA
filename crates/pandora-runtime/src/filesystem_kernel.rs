use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperation {
    pub operation: String,

    pub path: String,

    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    pub success: bool,

    pub output: String,
}

pub struct FilesystemKernel;

impl FilesystemKernel {
    pub fn execute(operation: &FileOperation) -> FileResult {
        println!(
            "[FS] operation={} path={}",
            operation.operation, operation.path
        );

        match operation.operation.as_str() {
            "read" => match fs::read_to_string(&operation.path) {
                Ok(content) => FileResult {
                    success: true,

                    output: content,
                },

                Err(error) => FileResult {
                    success: false,

                    output: error.to_string(),
                },
            },

            "write" => {
                let content = operation.content.clone().unwrap_or_default();

                match fs::write(&operation.path, content) {
                    Ok(_) => FileResult {
                        success: true,

                        output: "write successful".into(),
                    },

                    Err(error) => FileResult {
                        success: false,

                        output: error.to_string(),
                    },
                }
            }

            "delete" => match fs::remove_file(&operation.path) {
                Ok(_) => FileResult {
                    success: true,

                    output: "delete successful".into(),
                },

                Err(error) => FileResult {
                    success: false,

                    output: error.to_string(),
                },
            },

            _ => FileResult {
                success: false,

                output: "unknown operation".into(),
            },
        }
    }
}
