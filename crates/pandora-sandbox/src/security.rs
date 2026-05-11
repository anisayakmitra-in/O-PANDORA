use std::path::{
    Path,
    PathBuf,
};

use bollard::models::HostConfig;

use crate::error::SandboxError;

/// Validates that a requested mount path
/// stays inside the allowed workspace boundary.
pub fn validate_and_canonicalize_mount(
    allowed_base_dir: &Path,
    requested_host_path: &Path,
)
    -> Result<PathBuf, SandboxError>
{
    let canonical_requested =
        requested_host_path
            .canonicalize()
            .map_err(|error| {

                SandboxError::SecurityViolation(
                    format!(
                        "Path resolution failed: {}",
                        error
                    )
                )
            })?;

    let canonical_base =
        allowed_base_dir
            .canonicalize()
            .map_err(|error| {

                SandboxError::SecurityViolation(
                    format!(
                        "Base path resolution failed: {}",
                        error
                    )
                )
            })?;

    if !canonical_requested.starts_with(
        &canonical_base
    ) {

        return Err(
            SandboxError::SecurityViolation(
                String::from(
                    "Path traversal escape attempt detected"
                )
            )
        );
    }

    Ok(
        canonical_requested
    )
}

/// Returns hardened Docker HostConfig.
pub fn hardened_host_config(
    memory_bytes: i64,
    nano_cpus: i64,
)
    -> HostConfig
{
    HostConfig {

        memory: Some(
            memory_bytes
        ),

        nano_cpus: Some(
            nano_cpus
        ),

        cap_drop: Some(
            vec![
                String::from("ALL")
            ]
        ),

        security_opt: Some(
            vec![
                String::from(
                    "no-new-privileges:true"
                )
            ]
        ),

        readonly_rootfs: Some(
            true
        ),

        network_mode: Some(
            String::from("none")
        ),

        ..Default::default()
    }
}
