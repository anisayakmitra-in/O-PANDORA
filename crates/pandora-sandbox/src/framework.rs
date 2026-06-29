//! Sandbox Backend Framework.
//!
//! This module defines the contracts Phoenix uses to
//! dispatch execution. Every Sandbox Backend is
//! interchangeable: Phoenix only knows the
//!  trait.
//!
//! ## Architecture
//!
//! Phoenix (Execution Source Harness)
//!     |
//!     v
//! SandboxBackendRegistry
//!     |
//!     v
//! SandboxBackend trait
//!     |
//!     +-- NativeProcessBackend (reference impl)
//!     +-- DockerBackend (manifest only)
//!     +-- FirecrackerBackend (manifest only)
//!     +-- WasmtimeBackend (manifest only)
//!     +-- CommunityBackends (manifest + impl)
//!
//! ## Design rules
//!
//! - Backends never self-grant permissions. They
//!   receive  from the runtime's
//!   capability-leasing layer.
//! - Backends never call  directly. They
//!   return a  that the
//!   runtime routes through the leased capabilities.
//! - Backends are manifest-driven. Every installable
//!   backend ships a .
//! - The framework supports future backends (GPU,
//!   browser, VM, remote, distributed, confidential
//!   computing) without changing the trait.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The kind of sandbox backend. The framework
/// supports a fixed set of well-known backends
/// (for runtime use) plus a  variant for
/// community extensions.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SandboxBackendKind {
    Docker,
    Firecracker,
    Wasmtime,
    Gvisor,
    Bubblewrap,
    NativeProcess,
    Ssh,
    RemoteWorker,
    Kubernetes,
    Gpu,
    Browser,
    Vm,
    Confidential,
    Custom(String),
}

impl SandboxBackendKind {
    pub fn name(&self) -> &str {
        match self {
            SandboxBackendKind::Docker => "Docker",
            SandboxBackendKind::Firecracker => "Firecracker",
            SandboxBackendKind::Wasmtime => "Wasmtime",
            SandboxBackendKind::Gvisor => "gVisor",
            SandboxBackendKind::Bubblewrap => "Bubblewrap",
            SandboxBackendKind::NativeProcess => "NativeProcess",
            SandboxBackendKind::Ssh => "SSH",
            SandboxBackendKind::RemoteWorker => "RemoteWorker",
            SandboxBackendKind::Kubernetes => "Kubernetes",
            SandboxBackendKind::Gpu => "Gpu",
            SandboxBackendKind::Browser => "Browser",
            SandboxBackendKind::Vm => "Vm",
            SandboxBackendKind::Confidential => "Confidential",
            SandboxBackendKind::Custom(s) => s,
        }
    }

    pub fn all_known() -> &'static [SandboxBackendKind] {
        &[
            SandboxBackendKind::Docker,
            SandboxBackendKind::Firecracker,
            SandboxBackendKind::Wasmtime,
            SandboxBackendKind::Gvisor,
            SandboxBackendKind::Bubblewrap,
            SandboxBackendKind::NativeProcess,
            SandboxBackendKind::Ssh,
            SandboxBackendKind::RemoteWorker,
            SandboxBackendKind::Kubernetes,
            SandboxBackendKind::Gpu,
            SandboxBackendKind::Browser,
            SandboxBackendKind::Vm,
            SandboxBackendKind::Confidential,
        ]
    }
}

impl fmt::Display for SandboxBackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A manifest describing a sandbox backend. Every
/// installable backend ships one of these. The
/// manifest is the publication unit for KUBER Palace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxBackendManifest {
    pub name: String,
    pub kind: SandboxBackendKind,
    pub version: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub capabilities: SandboxCapabilities,
    pub health: SandboxHealth,
    pub limits: SandboxLimits,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: BTreeMap<String, String>,
}

impl SandboxBackendManifest {
    pub fn new(
        name: impl Into<String>,
        kind: SandboxBackendKind,
        version: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        SandboxBackendManifest {
            name: name.into(),
            kind,
            version: version.into(),
            description: description.into(),
            author: None,
            license: None,
            repository: None,
            signature: None,
            capabilities: SandboxCapabilities::default(),
            health: SandboxHealth::default(),
            limits: SandboxLimits::default(),
            dependencies: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_capabilities(mut self, c: SandboxCapabilities) -> Self {
        self.capabilities = c;
        self
    }

    pub fn with_health(mut self, h: SandboxHealth) -> Self {
        self.health = h;
        self
    }

    pub fn with_limits(mut self, l: SandboxLimits) -> Self {
        self.limits = l;
        self
    }

    pub fn with_dependency(mut self, d: impl Into<String>) -> Self {
        self.dependencies.push(d.into());
        self
    }
}

/// Capabilities a sandbox backend provides. The
/// runtime's leasing layer grants these; the backend
/// never self-grants.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SandboxCapabilities {
    pub filesystem: bool,
    pub network: bool,
    pub gpu: bool,
    pub provider: bool,
    pub shell: bool,
    pub browser: bool,
    pub memory: bool,
    pub clock: bool,
    pub process: bool,
    pub checkpoint: bool,
    pub snapshot: bool,
    pub migration: bool,
}

impl SandboxCapabilities {
    pub fn none() -> Self {
        SandboxCapabilities::default()
    }

    pub fn all() -> Self {
        SandboxCapabilities {
            filesystem: true,
            network: true,
            gpu: true,
            provider: true,
            shell: true,
            browser: true,
            memory: true,
            clock: true,
            process: true,
            checkpoint: true,
            snapshot: true,
            migration: true,
        }
    }
}

/// Health status of a sandbox backend.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SandboxHealth {
    pub healthy: bool,
    pub last_check_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SandboxHealth {
    pub fn healthy() -> Self {
        SandboxHealth {
            healthy: true,
            last_check_ms: 0,
            message: None,
        }
    }

    pub fn unhealthy(message: impl Into<String>) -> Self {
        SandboxHealth {
            healthy: false,
            last_check_ms: 0,
            message: Some(message.into()),
        }
    }
}

/// Resource limits the backend enforces on a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxLimits {
    pub cpu: f32,
    pub memory_bytes: u64,
    pub disk_bytes: u64,
    pub network_bps: u64,
    pub max_duration: Duration,
    pub max_processes: u32,
}

impl Default for SandboxLimits {
    fn default() -> Self {
        SandboxLimits {
            cpu: 1.0,
            memory_bytes: 512 * 1024 * 1024,
            disk_bytes: 1024 * 1024 * 1024,
            network_bps: 0,
            max_duration: Duration::from_secs(60),
            max_processes: 64,
        }
    }
}

/// Isolation level. The backend enforces this.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SandboxIsolation {
    /// Same process, same UID.
    #[default]
    None,
    /// Separate UID, same kernel.
    Uid,
    /// Separate mount namespace.
    Namespace,
    /// Separate PID, mount, network namespaces.
    Container,
    /// Hardware-virtualized microVM.
    MicroVm,
    /// Hardware-virtualized full VM.
    Vm,
    /// Hardware-isolated (TEE).
    Tee,
}

/// A budget. Backends consume budget; the runtime
/// refills it through capability leasing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SandboxBudget {
    pub tokens: u64,
    pub wall_ms: u64,
    pub operations: u64,
}

/// A filesystem view the backend exposes to a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SandboxFilesystem {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mounts: Vec<String>,
    pub read_only: bool,
}

/// Network configuration the backend exposes to a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SandboxNetwork {
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deny: Vec<String>,
}

/// Environment variables for a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct SandboxEnvironment {
    pub values: BTreeMap<String, String>,
}

impl SandboxEnvironment {
    pub fn new() -> Self {
        SandboxEnvironment::default()
    }
    pub fn set(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.values.insert(k.into(), v.into());
        self
    }
}

/// A process invocation inside a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxProcess {
    pub executable: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
}

impl SandboxProcess {
    pub fn new(executable: impl Into<String>) -> Self {
        SandboxProcess {
            executable: executable.into(),
            args: Vec::new(),
            env: BTreeMap::new(),
            cwd: None,
        }
    }

    pub fn with_arg(mut self, a: impl Into<String>) -> Self {
        self.args.push(a.into());
        self
    }
}

/// An execution request the runtime hands to a backend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxExecutionRequest {
    pub session_id: String,
    pub process: SandboxProcess,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<SandboxBudget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl SandboxExecutionRequest {
    pub fn new(session_id: impl Into<String>, process: SandboxProcess) -> Self {
        SandboxExecutionRequest {
            session_id: session_id.into(),
            process,
            timeout_ms: None,
            budget: None,
            input: None,
        }
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    pub fn with_budget(mut self, b: SandboxBudget) -> Self {
        self.budget = Some(b);
        self
    }

    pub fn with_input(mut self, s: impl Into<String>) -> Self {
        self.input = Some(s.into());
        self
    }
}

/// An execution result returned by a backend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxExecutionResult {
    pub session_id: String,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<SandboxArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl SandboxExecutionResult {
    pub fn success(
        session_id: impl Into<String>,
        exit_code: i32,
        stdout: impl Into<String>,
    ) -> Self {
        SandboxExecutionResult {
            session_id: session_id.into(),
            exit_code,
            stdout: stdout.into(),
            stderr: String::new(),
            duration_ms: 0,
            artifacts: Vec::new(),
            error: None,
        }
    }

    pub fn failure(session_id: impl Into<String>, error: impl Into<String>) -> Self {
        SandboxExecutionResult {
            session_id: session_id.into(),
            exit_code: -1,
            stdout: String::new(),
            stderr: String::new(),
            duration_ms: 0,
            artifacts: Vec::new(),
            error: Some(error.into()),
        }
    }
}

/// An artifact produced by an execution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxArtifact {
    pub name: String,
    pub content_type: String,
    pub data: String,
}

impl SandboxArtifact {
    pub fn new(
        name: impl Into<String>,
        content_type: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        SandboxArtifact {
            name: name.into(),
            content_type: content_type.into(),
            data: data.into(),
        }
    }
}

/// A log record from a session.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxLog {
    pub session_id: String,
    pub level: String,
    pub message: String,
    pub timestamp_ms: u64,
}

impl SandboxLog {
    pub fn new(
        session_id: impl Into<String>,
        level: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        SandboxLog {
            session_id: session_id.into(),
            level: level.into(),
            message: message.into(),
            timestamp_ms: 0,
        }
    }
}

/// Diagnostic information a backend emits on failure.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxDiagnostics {
    pub session_id: String,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub context: BTreeMap<String, String>,
}

impl SandboxDiagnostics {
    pub fn new(
        session_id: impl Into<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        SandboxDiagnostics {
            session_id: session_id.into(),
            code: code.into(),
            message: message.into(),
            context: BTreeMap::new(),
        }
    }
}

/// Telemetry reported by a backend. The runtime feeds
/// this to ANUBIS, PANOPTES, and SHANI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SandboxTelemetry {
    pub session_id: String,
    pub cpu_seconds: f64,
    pub gpu_seconds: f64,
    pub memory_peak_bytes: u64,
    pub disk_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub execution_duration_ms: u64,
    pub checkpoint_count: u64,
    pub rollback_count: u64,
    pub restart_count: u64,
}

impl Default for SandboxTelemetry {
    fn default() -> Self {
        SandboxTelemetry {
            session_id: String::new(),
            cpu_seconds: 0.0,
            gpu_seconds: 0.0,
            memory_peak_bytes: 0,
            disk_bytes: 0,
            network_rx_bytes: 0,
            network_tx_bytes: 0,
            execution_duration_ms: 0,
            checkpoint_count: 0,
            rollback_count: 0,
            restart_count: 0,
        }
    }
}

/// Statistics a backend reports periodically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SandboxStatistics {
    pub sessions_total: u64,
    pub sessions_active: u64,
    pub sessions_failed: u64,
    pub avg_session_duration_ms: u64,
}

/// A checkpoint produced by a backend.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxCheckpoint {
    pub session_id: String,
    pub checkpoint_id: String,
    pub parent: Option<String>,
    pub timestamp_ms: u64,
}

impl SandboxCheckpoint {
    pub fn new(session_id: impl Into<String>, checkpoint_id: impl Into<String>) -> Self {
        SandboxCheckpoint {
            session_id: session_id.into(),
            checkpoint_id: checkpoint_id.into(),
            parent: None,
            timestamp_ms: 0,
        }
    }
}

/// A snapshot is a checkpoint plus a serializable
/// representation of session state. The runtime can
/// persist a snapshot and rehydrate a session from it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxSnapshot {
    pub checkpoint: SandboxCheckpoint,
    pub data: String,
}

impl SandboxSnapshot {
    pub fn from_checkpoint(c: SandboxCheckpoint, data: impl Into<String>) -> Self {
        SandboxSnapshot {
            checkpoint: c,
            data: data.into(),
        }
    }
}

/// A rollback request. The backend reverts the session
/// to the named checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxRollback {
    pub session_id: String,
    pub to_checkpoint: String,
}

impl SandboxRollback {
    pub fn new(session_id: impl Into<String>, to_checkpoint: impl Into<String>) -> Self {
        SandboxRollback {
            session_id: session_id.into(),
            to_checkpoint: to_checkpoint.into(),
        }
    }
}

/// A migration request. The backend moves a session
/// to a different host.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxMigration {
    pub session_id: String,
    pub target: String,
}

impl SandboxMigration {
    pub fn new(session_id: impl Into<String>, target: impl Into<String>) -> Self {
        SandboxMigration {
            session_id: session_id.into(),
            target: target.into(),
        }
    }
}

/// A repair request. The runtime asks a backend to
/// recover a session from a known state.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxRepairRequest {
    pub session_id: String,
    pub strategy: String,
}

impl SandboxRepairRequest {
    pub fn new(session_id: impl Into<String>, strategy: impl Into<String>) -> Self {
        SandboxRepairRequest {
            session_id: session_id.into(),
            strategy: strategy.into(),
        }
    }
}

/// A repair result. The backend reports whether the
/// repair succeeded.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SandboxRepairResult {
    pub session_id: String,
    pub succeeded: bool,
    pub message: String,
}

impl SandboxRepairResult {
    pub fn success(session_id: impl Into<String>, message: impl Into<String>) -> Self {
        SandboxRepairResult {
            session_id: session_id.into(),
            succeeded: true,
            message: message.into(),
        }
    }

    pub fn failure(session_id: impl Into<String>, message: impl Into<String>) -> Self {
        SandboxRepairResult {
            session_id: session_id.into(),
            succeeded: false,
            message: message.into(),
        }
    }
}

/// A session handle. The runtime holds one of these
/// while a session is active. Drop is not implemented;
/// the runtime is expected to call .
pub struct SandboxSession {
    pub session_id: String,
    pub backend: String,
    pub state: SessionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SessionState {
    Initializing,
    Ready,
    Running,
    Paused,
    Checkpointed,
    Migrating,
    Repairing,
    Terminating,
    Terminated,
    Failed,
}

impl SessionState {
    pub fn as_str(self) -> &'static str {
        match self {
            SessionState::Initializing => "INITIALIZING",
            SessionState::Ready => "READY",
            SessionState::Running => "RUNNING",
            SessionState::Paused => "PAUSED",
            SessionState::Checkpointed => "CHECKPOINTED",
            SessionState::Migrating => "MIGRATING",
            SessionState::Repairing => "REPAIRING",
            SessionState::Terminating => "TERMINATING",
            SessionState::Terminated => "TERMINATED",
            SessionState::Failed => "FAILED",
        }
    }
}

impl SandboxSession {
    pub fn new(session_id: impl Into<String>, backend: impl Into<String>) -> Self {
        SandboxSession {
            session_id: session_id.into(),
            backend: backend.into(),
            state: SessionState::Initializing,
        }
    }
}

/// The contract every sandbox backend implements.
/// Phoenix talks to backends only through this trait.
///
/// All methods are async because backends may use
/// tokio for I/O. The default implementations return
///  so a backend can opt in to
/// capabilities incrementally.
#[async_trait::async_trait]
pub trait SandboxBackend: Send + Sync {
    /// The manifest of this backend. The runtime
    /// reads this for registration and validation.
    fn manifest(&self) -> &SandboxBackendManifest;

    /// The kind of backend (shorthand for
    /// ).
    fn kind(&self) -> SandboxBackendKind {
        self.manifest().kind.clone()
    }

    /// The name of the backend (shorthand for
    /// ).
    fn name(&self) -> &str {
        &self.manifest().name
    }

    /// Initialize the backend. Called once at
    /// registration time. Default: no-op.
    async fn initialize(&self) -> Result<(), String> {
        Ok(())
    }

    /// Prepare a session. The runtime calls this
    /// before spawning work in a session. Default:
    /// returns a  in .
    async fn prepare(&self, session_id: &str) -> Result<SandboxSession, String> {
        Ok(SandboxSession::new(
            session_id.to_string(),
            self.name().to_string(),
        ))
    }

    /// Lease capabilities into the session. Default:
    /// no-op. The runtime is responsible for
    /// revoking the lease.
    async fn lease_capabilities(
        &self,
        _session: &mut SandboxSession,
        _caps: &SandboxCapabilities,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Spawn a process inside the session. The default
    /// returns  so backends that don't
    /// support spawning return an error explicitly.
    async fn spawn(
        &self,
        _session: &mut SandboxSession,
        _process: &SandboxProcess,
    ) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Execute a previously spawned process. Default:
    /// returns .
    async fn execute(
        &self,
        session: &mut SandboxSession,
        request: &SandboxExecutionRequest,
    ) -> Result<SandboxExecutionResult, String> {
        let _ = (session, request);
        Err("NotImplemented".to_string())
    }

    /// Stream a chunk of stdout/stderr. Default:
    /// returns no chunks.
    async fn stream(&self, _session: &mut SandboxSession) -> Result<Vec<SandboxLog>, String> {
        Ok(Vec::new())
    }

    /// Pause the session. Default: returns
    /// .
    async fn pause(&self, _session: &mut SandboxSession) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Resume the session. Default: returns
    /// .
    async fn resume(&self, _session: &mut SandboxSession) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Checkpoint the session. Default: returns
    /// .
    async fn checkpoint(&self, _session: &mut SandboxSession) -> Result<SandboxCheckpoint, String> {
        Err("NotImplemented".to_string())
    }

    /// Snapshot the session. Default: returns
    /// .
    async fn snapshot(&self, _session: &mut SandboxSession) -> Result<SandboxSnapshot, String> {
        Err("NotImplemented".to_string())
    }

    /// Rollback the session. Default: returns
    /// .
    async fn rollback(
        &self,
        _session: &mut SandboxSession,
        _to: &SandboxRollback,
    ) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Repair the session. Default: returns
    /// .
    async fn repair(
        &self,
        _session: &mut SandboxSession,
        _req: &SandboxRepairRequest,
    ) -> Result<SandboxRepairResult, String> {
        Err("NotImplemented".to_string())
    }

    /// Migrate the session. Default: returns
    /// .
    async fn migrate(
        &self,
        _session: &mut SandboxSession,
        _m: &SandboxMigration,
    ) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Terminate the session. Default: returns
    /// .
    async fn terminate(&self, _session: &mut SandboxSession) -> Result<(), String> {
        Err("NotImplemented".to_string())
    }

    /// Cleanup resources. Default: returns Ok.
    async fn cleanup(&self, _session: &mut SandboxSession) -> Result<(), String> {
        Ok(())
    }

    /// Archive the session for later analysis.
    /// Default: returns .
    async fn archive(&self, _session: &mut SandboxSession) -> Result<String, String> {
        Err("NotImplemented".to_string())
    }

    /// Report telemetry. Default: returns an empty
    /// .
    async fn telemetry(&self, session: &SandboxSession) -> Result<SandboxTelemetry, String> {
        let t = SandboxTelemetry {
            session_id: session.session_id.clone(),
            ..Default::default()
        };
        Ok(t)
    }

    /// Report statistics. Default: returns an empty
    /// .
    async fn statistics(&self) -> Result<SandboxStatistics, String> {
        Ok(SandboxStatistics::default())
    }

    /// Health check. Default: returns .
    async fn health(&self) -> Result<SandboxHealth, String> {
        Ok(SandboxHealth::healthy())
    }
}

/// A registry of sandbox backends. The runtime uses
/// this to discover and dispatch to backends.
pub struct SandboxBackendRegistry {
    inner:
        std::sync::RwLock<std::collections::BTreeMap<String, std::sync::Arc<dyn SandboxBackend>>>,
}

impl SandboxBackendRegistry {
    pub fn new() -> Self {
        SandboxBackendRegistry {
            inner: std::sync::RwLock::new(std::collections::BTreeMap::new()),
        }
    }

    /// Register a backend. Returns the existing
    /// backend if the name is already taken.
    pub fn register(&self, backend: std::sync::Arc<dyn SandboxBackend>) -> Result<(), String> {
        let name = backend.name().to_string();
        let mut guard = self.inner.write().map_err(|e| e.to_string())?;
        if guard.contains_key(&name) {
            return Err(format!("backend {:?} already registered", name));
        }
        guard.insert(name, backend);
        Ok(())
    }

    pub fn unregister(&self, name: &str) -> bool {
        let mut guard = self.inner.write().expect("registry poisoned");
        guard.remove(name).is_some()
    }

    pub fn lookup(&self, name: &str) -> Option<std::sync::Arc<dyn SandboxBackend>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.get(name).cloned()
    }

    pub fn lookup_by_kind(
        &self,
        kind: &SandboxBackendKind,
    ) -> Vec<std::sync::Arc<dyn SandboxBackend>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard
            .values()
            .filter(|b| &b.kind() == kind)
            .cloned()
            .collect()
    }

    pub fn list(&self) -> Vec<std::sync::Arc<dyn SandboxBackend>> {
        let guard = self.inner.read().expect("registry poisoned");
        guard.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.read().expect("registry poisoned");
        guard.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for SandboxBackendRegistry {
    fn default() -> Self {
        SandboxBackendRegistry::new()
    }
}

/// Validation errors. The validator runs at
/// registration time to reject broken manifests.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SandboxBackendValidationError {
    #[error("backend name must not be empty")]
    EmptyName,
    #[error("backend version must not be empty")]
    EmptyVersion,
    #[error("backend description must not be empty")]
    EmptyDescription,
    #[error("backend manifest has no capabilities declared")]
    NoCapabilities,
}

/// The validator. The runtime calls this at
/// registration time.
pub struct SandboxBackendValidator;

impl SandboxBackendValidator {
    pub fn new() -> Self {
        SandboxBackendValidator
    }

    pub fn validate(
        &self,
        m: &SandboxBackendManifest,
    ) -> Result<(), SandboxBackendValidationError> {
        if m.name.trim().is_empty() {
            return Err(SandboxBackendValidationError::EmptyName);
        }
        if m.version.trim().is_empty() {
            return Err(SandboxBackendValidationError::EmptyVersion);
        }
        if m.description.trim().is_empty() {
            return Err(SandboxBackendValidationError::EmptyDescription);
        }
        if !m.capabilities.filesystem
            && !m.capabilities.network
            && !m.capabilities.shell
            && !m.capabilities.process
        {
            return Err(SandboxBackendValidationError::NoCapabilities);
        }
        Ok(())
    }
}

impl Default for SandboxBackendValidator {
    fn default() -> Self {
        SandboxBackendValidator::new()
    }
}

/// A loader discovers and loads backend manifests.
/// The runtime uses this to populate the registry.
pub trait SandboxBackendLoader: Send + Sync {
    fn loader_name(&self) -> &str;
    fn load(&self) -> Vec<SandboxBackendManifest> {
        Vec::new()
    }
}

/// An in-memory loader useful for tests and for
/// embedding manifests directly.
pub struct InMemorySandboxBackendLoader {
    name: String,
    manifests: Vec<SandboxBackendManifest>,
}

impl InMemorySandboxBackendLoader {
    pub fn new(name: impl Into<String>) -> Self {
        InMemorySandboxBackendLoader {
            name: name.into(),
            manifests: Vec::new(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, m: SandboxBackendManifest) -> Self {
        self.manifests.push(m);
        self
    }
}

impl SandboxBackendLoader for InMemorySandboxBackendLoader {
    fn loader_name(&self) -> &str {
        &self.name
    }
    fn load(&self) -> Vec<SandboxBackendManifest> {
        self.manifests.clone()
    }
}

/// The reference backend: a minimal in-process
/// backend that does NOT execute real work. It only
/// implements the trait so other crates can be tested
/// without a real sandbox engine. The runtime can
/// substitute it for a real backend in tests.
pub struct NativeProcessBackend {
    manifest: SandboxBackendManifest,
    state: std::sync::Mutex<Option<SessionState>>,
}

impl NativeProcessBackend {
    pub fn new() -> Self {
        NativeProcessBackend {
            manifest: SandboxBackendManifest::new(
                "native-process",
                SandboxBackendKind::NativeProcess,
                "0.1.0",
                "Reference backend that does not execute real work",
            )
            .with_capabilities(SandboxCapabilities {
                filesystem: true,
                process: true,
                shell: true,
                ..SandboxCapabilities::default()
            })
            .with_health(SandboxHealth::healthy())
            .with_limits(SandboxLimits::default()),
            state: std::sync::Mutex::new(None),
        }
    }
}

impl Default for NativeProcessBackend {
    fn default() -> Self {
        NativeProcessBackend::new()
    }
}

#[async_trait::async_trait]
impl SandboxBackend for NativeProcessBackend {
    fn manifest(&self) -> &SandboxBackendManifest {
        &self.manifest
    }

    async fn initialize(&self) -> Result<(), String> {
        Ok(())
    }

    async fn prepare(&self, session_id: &str) -> Result<SandboxSession, String> {
        let mut s = SandboxSession::new(session_id, self.name());
        s.state = SessionState::Ready;
        *self.state.lock().expect("backend poisoned") = Some(s.state);
        Ok(s)
    }

    async fn execute(
        &self,
        session: &mut SandboxSession,
        request: &SandboxExecutionRequest,
    ) -> Result<SandboxExecutionResult, String> {
        // The reference backend records what was
        // asked and returns a synthetic success.
        // It does not actually spawn a process.
        session.state = SessionState::Running;
        *self.state.lock().expect("backend poisoned") = Some(session.state);
        Ok(SandboxExecutionResult::success(
            request.session_id.clone(),
            0,
            format!(
                "reference backend: would run {:?}",
                request.process.executable
            ),
        ))
    }

    async fn terminate(&self, session: &mut SandboxSession) -> Result<(), String> {
        session.state = SessionState::Terminated;
        *self.state.lock().expect("backend poisoned") = Some(session.state);
        Ok(())
    }
}

/// Helpers for constructing common manifests. The
/// runtime uses these to register reference backends
/// (Docker, Firecracker, Wasmtime, etc.) by manifest
/// only. The implementation lives elsewhere (or in a
/// community package).
pub mod manifests {
    use super::*;

    /// Construct a Docker backend manifest.
    pub fn docker_manifest() -> SandboxBackendManifest {
        SandboxBackendManifest::new(
            "docker",
            SandboxBackendKind::Docker,
            "0.1.0",
            "Docker container backend",
        )
        .with_capabilities(SandboxCapabilities {
            filesystem: true,
            network: true,
            process: true,
            shell: true,
            checkpoint: true,
            snapshot: true,
            ..SandboxCapabilities::default()
        })
        .with_health(SandboxHealth::healthy())
        .with_limits(SandboxLimits {
            cpu: 4.0,
            memory_bytes: 8 * 1024 * 1024 * 1024,
            disk_bytes: 100 * 1024 * 1024 * 1024,
            network_bps: 1024 * 1024 * 1024,
            max_duration: Duration::from_secs(3600),
            max_processes: 1024,
        })
        .with_dependency("docker")
    }

    /// Construct a Firecracker backend manifest.
    pub fn firecracker_manifest() -> SandboxBackendManifest {
        SandboxBackendManifest::new(
            "firecracker",
            SandboxBackendKind::Firecracker,
            "0.1.0",
            "Firecracker microVM backend",
        )
        .with_capabilities(SandboxCapabilities {
            filesystem: true,
            network: true,
            process: true,
            snapshot: true,
            migration: true,
            ..SandboxCapabilities::default()
        })
        .with_health(SandboxHealth::healthy())
        .with_dependency("firecracker")
        .with_dependency("kvm")
    }

    /// Construct a Wasmtime backend manifest.
    pub fn wasmtime_manifest() -> SandboxBackendManifest {
        SandboxBackendManifest::new(
            "wasmtime",
            SandboxBackendKind::Wasmtime,
            "0.1.0",
            "Wasmtime WebAssembly backend",
        )
        .with_capabilities(SandboxCapabilities {
            filesystem: false,
            network: false,
            process: true,
            ..SandboxCapabilities::default()
        })
        .with_health(SandboxHealth::healthy())
        .with_dependency("wasmtime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn docker() -> SandboxBackendManifest {
        manifests::docker_manifest()
    }

    fn native() -> std::sync::Arc<dyn SandboxBackend> {
        std::sync::Arc::new(NativeProcessBackend::new())
    }

    #[test]
    fn kind_names() {
        assert_eq!(SandboxBackendKind::Docker.name(), "Docker");
        assert_eq!(SandboxBackendKind::NativeProcess.name(), "NativeProcess");
        assert_eq!(SandboxBackendKind::Custom("x".to_string()).name(), "x");
    }

    #[test]
    fn kind_all_known_includes_main_kinds() {
        for k in [
            SandboxBackendKind::Docker,
            SandboxBackendKind::Firecracker,
            SandboxBackendKind::Wasmtime,
            SandboxBackendKind::Gvisor,
            SandboxBackendKind::Bubblewrap,
            SandboxBackendKind::NativeProcess,
        ] {
            assert!(SandboxBackendKind::all_known().contains(&k));
        }
    }

    #[test]
    fn manifest_builder() {
        let m = docker();
        assert_eq!(m.name, "docker");
        assert_eq!(m.kind, SandboxBackendKind::Docker);
        assert!(m.capabilities.filesystem);
    }

    #[test]
    fn capabilities_all_set_true() {
        let c = SandboxCapabilities::all();
        assert!(c.filesystem);
        assert!(c.gpu);
        assert!(c.checkpoint);
    }

    #[test]
    fn limits_default() {
        let l = SandboxLimits::default();
        assert_eq!(l.cpu, 1.0);
        assert!(l.memory_bytes > 0);
    }

    #[test]
    fn execution_request_builder() {
        let req = SandboxExecutionRequest::new("sess-1", SandboxProcess::new("ls").with_arg("-l"))
            .with_timeout(5000)
            .with_budget(SandboxBudget {
                tokens: 100,
                wall_ms: 5000,
                operations: 1,
            })
            .with_input("hello");
        assert_eq!(req.session_id, "sess-1");
        assert_eq!(req.process.executable, "ls");
        assert_eq!(req.process.args, vec!["-l"]);
    }

    #[test]
    fn execution_result_success_and_failure() {
        let s = SandboxExecutionResult::success("s", 0, "ok");
        assert_eq!(s.exit_code, 0);
        let f = SandboxExecutionResult::failure("s", "bad");
        assert_eq!(f.exit_code, -1);
        assert!(f.error.is_some());
    }

    #[test]
    fn session_state_lifecycle() {
        let s = SessionState::Initializing;
        assert_eq!(s.as_str(), "INITIALIZING");
        assert_eq!(SessionState::Ready.as_str(), "READY");
    }

    #[test]
    fn validator_accepts_docker() {
        let m = docker();
        assert!(SandboxBackendValidator::new().validate(&m).is_ok());
    }

    #[test]
    fn validator_rejects_empty_name() {
        let mut m = docker();
        m.name = "   ".to_string();
        assert_eq!(
            SandboxBackendValidator::new().validate(&m),
            Err(SandboxBackendValidationError::EmptyName)
        );
    }

    #[test]
    fn validator_rejects_no_capabilities() {
        let mut m = docker();
        m.capabilities = SandboxCapabilities::none();
        assert_eq!(
            SandboxBackendValidator::new().validate(&m),
            Err(SandboxBackendValidationError::NoCapabilities)
        );
    }

    #[test]
    fn registry_register_and_lookup() {
        let r = SandboxBackendRegistry::new();
        r.register(native()).unwrap();
        let found = r.lookup("native-process").unwrap();
        assert_eq!(found.name(), "native-process");
    }

    #[test]
    fn registry_lookup_by_kind() {
        let r = SandboxBackendRegistry::new();
        r.register(native()).unwrap();
        let results = r.lookup_by_kind(&SandboxBackendKind::NativeProcess);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn registry_unregister() {
        let r = SandboxBackendRegistry::new();
        r.register(native()).unwrap();
        assert!(r.unregister("native-process"));
        assert!(!r.unregister("native-process"));
    }

    #[test]
    fn registry_rejects_duplicate() {
        let r = SandboxBackendRegistry::new();
        r.register(native()).unwrap();
        let result = r.register(native());
        assert!(result.is_err());
    }

    #[test]
    fn in_memory_loader_returns_added() {
        let l = InMemorySandboxBackendLoader::new("test")
            .add(docker())
            .add(manifests::firecracker_manifest());
        assert_eq!(l.loader_name(), "test");
        let loaded = l.load();
        assert_eq!(loaded.len(), 2);
    }

    #[tokio::test]
    async fn native_backend_lifecycle() {
        let b = NativeProcessBackend::new();
        b.initialize().await.unwrap();
        let mut s = b.prepare("sess-1").await.unwrap();
        assert_eq!(s.state, SessionState::Ready);
        let req =
            SandboxExecutionRequest::new("sess-1", SandboxProcess::new("echo").with_arg("hi"));
        let r = b.execute(&mut s, &req).await.unwrap();
        assert_eq!(r.exit_code, 0);
        assert!(r.stdout.contains("echo"));
        b.terminate(&mut s).await.unwrap();
        assert_eq!(s.state, SessionState::Terminated);
    }

    #[test]
    fn checkpoint_snapshot_rollback() {
        let cp = SandboxCheckpoint::new("s", "cp-1");
        let snap = SandboxSnapshot::from_checkpoint(cp.clone(), "data");
        assert_eq!(snap.data, "data");
        let rb = SandboxRollback::new("s", "cp-1");
        assert_eq!(rb.to_checkpoint, "cp-1");
    }

    #[test]
    fn migration_repair() {
        let m = SandboxMigration::new("s", "host-2");
        let req = SandboxRepairRequest::new("s", "restart");
        let res = SandboxRepairResult::success("s", "ok");
        assert_eq!(m.target, "host-2");
        assert_eq!(req.strategy, "restart");
        assert!(res.succeeded);
    }

    #[test]
    fn telemetry_and_statistics_default() {
        let t = SandboxTelemetry::default();
        assert_eq!(t.checkpoint_count, 0);
        let s = SandboxStatistics::default();
        assert_eq!(s.sessions_total, 0);
    }

    #[test]
    fn log_diagnostics_artifact() {
        let l = SandboxLog::new("s", "INFO", "hello");
        assert_eq!(l.level, "INFO");
        let d = SandboxDiagnostics::new("s", "E001", "crashed");
        assert_eq!(d.code, "E001");
        let a = SandboxArtifact::new("out.txt", "text/plain", "data");
        assert_eq!(a.content_type, "text/plain");
    }

    #[test]
    fn environment_and_process() {
        let env = SandboxEnvironment::new().set("KEY", "VALUE");
        assert_eq!(env.values.get("KEY"), Some(&"VALUE".to_string()));
        let p = SandboxProcess::new("sh").with_arg("-c").with_arg("ls");
        assert_eq!(p.args, vec!["-c", "ls"]);
    }

    #[test]
    fn isolation_default_none() {
        assert_eq!(SandboxIsolation::default(), SandboxIsolation::None);
    }
}
