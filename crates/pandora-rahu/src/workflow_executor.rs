//! Workflow Executor.
//!
//! Workflows become executable graphs. This executor
//! runs workflow steps in order, tracks checkpoints,
//! handles retries and failures.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::universal::{Health, Lifecycle, WorkflowManifest};

/// Status of a workflow execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WorkflowRunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// A running workflow instance.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowRun {
    pub run_id: String,
    pub manifest: WorkflowManifest,
    pub status: WorkflowRunStatus,
    pub current_step: usize,
    pub step_results: Vec<StepResult>,
    pub health: Health,
    pub lifecycle: Lifecycle,
}

/// Result of a single workflow step.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StepResult {
    pub step_name: String,
    pub success: bool,
    pub output: Option<String>,
    pub duration_ms: u64,
    pub retries: u32,
}

/// Executes workflow graphs.
pub struct WorkflowExecutor {
    runs: Arc<Mutex<BTreeMap<String, WorkflowRun>>>,
    next_id: Arc<Mutex<u64>>,
}

impl WorkflowExecutor {
    pub fn new() -> Self {
        WorkflowExecutor {
            runs: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Start a workflow run from a manifest.
    pub fn start(&self, manifest: WorkflowManifest) -> WorkflowRun {
        let id = self.next_id();
        let run = WorkflowRun {
            run_id: format!("wf-{}", id),
            manifest,
            status: WorkflowRunStatus::Running,
            current_step: 0,
            step_results: vec![],
            health: Health::Healthy,
            lifecycle: Lifecycle::Running,
        };
        self.runs
            .lock()
            .unwrap()
            .insert(run.run_id.clone(), run.clone());
        run
    }

    /// Complete a workflow run.
    pub fn complete(&self, run_id: &str, success: bool) -> bool {
        let mut runs = self.runs.lock().unwrap();
        if let Some(run) = runs.get_mut(run_id) {
            run.status = if success {
                WorkflowRunStatus::Completed
            } else {
                WorkflowRunStatus::Failed
            };
            run.lifecycle = Lifecycle::Stopped;
            true
        } else {
            false
        }
    }

    /// Get a run by ID.
    pub fn get_run(&self, run_id: &str) -> Option<WorkflowRun> {
        self.runs.lock().unwrap().get(run_id).cloned()
    }

    /// List all running workflows.
    pub fn running(&self) -> Vec<WorkflowRun> {
        self.runs
            .lock()
            .unwrap()
            .values()
            .filter(|r| r.status == WorkflowRunStatus::Running)
            .cloned()
            .collect()
    }

    fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new()
    }
}
