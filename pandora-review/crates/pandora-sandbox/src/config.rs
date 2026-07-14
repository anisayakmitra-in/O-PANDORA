use std::path::PathBuf;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub memory_bytes: i64,

    pub nano_cpus: i64,

    pub pids_limit: i64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_bytes: 256 * 1024 * 1024,

            nano_cpus: 1_000_000_000,

            pids_limit: 64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub image: String,

    pub network_disabled: bool,

    pub limits: ResourceLimits,

    pub mounts: Vec<(PathBuf, PathBuf)>,

    pub drop_capabilities: Vec<String>,

    pub readonly_rootfs: bool,

    pub no_new_privileges: bool,

    pub user_namespace: String,

    pub seccomp_profile: Option<String>,
}

impl SandboxConfig {
    pub fn default_isolated() -> Self {
        Self {
            image: String::from("debian:bookworm-slim"),

            network_disabled: true,

            limits: ResourceLimits::default(),

            mounts: Vec::new(),

            drop_capabilities: vec![String::from("ALL")],

            readonly_rootfs: true,

            no_new_privileges: true,

            user_namespace: String::from("1000:1000"),

            seccomp_profile: None,
        }
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self::default_isolated()
    }
}

#[derive(Debug, Clone)]
pub struct SandboxCommand {
    pub cmd: Vec<String>,

    pub env: Vec<String>,

    pub working_dir: String,

    pub timeout: Duration,
}

impl SandboxCommand {
    pub fn new<I, S>(cmd: I, timeout_secs: u64) -> Self
    where
        I: IntoIterator<Item = S>,

        S: Into<String>,
    {
        Self {
            cmd: cmd.into_iter().map(Into::into).collect(),

            env: Vec::new(),

            working_dir: String::from("/tmp"),

            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub fn with_workdir<S>(mut self, dir: S) -> Self
    where
        S: Into<String>,
    {
        self.working_dir = dir.into();

        self
    }
}
