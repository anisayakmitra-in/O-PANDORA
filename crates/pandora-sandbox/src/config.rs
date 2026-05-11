use std::path::PathBuf;
use std::time::Duration;

/// Defines strict cgroup resource boundaries to prevent containerized tasks 
/// from starving the host substrate or neighboring agents.
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum RAM allowed (in bytes).
    pub memory_bytes: i64,
    /// CPU quota (1 CPU = 1_000_000_000 nano_cpus).
    pub nano_cpus: i64,
    /// Strict limit on the number of processes to prevent fork bombs.
    pub pids_limit: i64,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_bytes: 512 * 1024 * 1024, // 512 MB default
            nano_cpus: 1_000_000_000,        // 1 Core default
            pids_limit: 64,                  // Low PID limit to prevent fork bombs
        }
    }
}

/// Defines a volume bind mount. 
/// In production, `host_path` must be passed through `security::validate_and_canonicalize_mount`
/// before being passed to the Docker API.
#[derive(Debug, Clone)]
pub struct MountConfig {
    pub host_path: PathBuf,
    pub container_path: PathBuf,
    pub read_only: bool,
}

/// The immutable specification for how a sandbox container is provisioned.
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub image: String,
    pub network_disabled: bool,
    pub limits: ResourceLimits,
    pub mounts: Vec<MountConfig>,
    pub drop_capabilities: Vec<String>,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            image: "debian:bookworm-slim".to_string(),
            network_disabled: true, // Secure-by-default: Air-gapped
            limits: ResourceLimits::default(),
            mounts: Vec::new(),
            // Secure-by-default: Strip all Linux capabilities
            drop_capabilities: vec!["ALL".to_string()],
        }
    }
}

/// Represents a single execution intent inside an active sandbox.
#[derive(Debug, Clone)]
pub struct SandboxCommand {
    pub cmd: Vec<String>,
    pub env: Vec<String>,
    pub working_dir: String,
    /// Hard execution timeout. The active sandbox will multiplex this alongside 
    /// the CancellationToken to guarantee the execution yields.
    pub timeout: Duration,
}

impl Default for SandboxCommand {
    fn default() -> Self {
        Self {
            cmd: vec!["echo".to_string(), "NO_COMMAND_SPECIFIED".to_string()],
            env: Vec::new(),
            working_dir: "/tmp".to_string(), // Default to a standard volatile directory
            timeout: Duration::from_secs(30), // Aggressive default timeout
        }
    }
}

impl SandboxCommand {
    /// Builder pattern helper to easily construct commands with a specific timeout.
    pub fn new<I, S>(cmd: I, timeout_secs: u64) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            cmd: cmd.into_iter().map(Into::into).collect(),
            timeout: Duration::from_secs(timeout_secs),
            ..Default::default()
        }
    }

    /// Adds environment variables securely.
    pub fn with_env<K, V>(mut self, key: K, value: V) -> Self
    where
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.env.push(format!("{}={}", key.as_ref(), value.as_ref()));
        self
    }

    /// Overrides the default working directory.
    pub fn with_workdir<S: Into<String>>(mut self, dir: S) -> Self {
        self.working_dir = dir.into();
        self
    }
}
