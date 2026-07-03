//! Workflow Engine — defines WHAT gets executed.
//!
//! The Workflow Engine produces an Execution Graph (a DAG of steps)
//! that the Loop Engine consumes. The Workflow Engine never iterates.
//! The Loop Engine never defines what steps are.
//!
//! Workflow answers: "What must happen?"
//! Loop answers: "How long do we keep improving?"
//!
//! One workflow, many loop strategies.

use crate::runtime_context::RuntimeContext;
use serde::Serialize;

/// The type of work a workflow step performs.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum StepKind {
    Plan,
    Execute,
    Verify,
    Reflect,
    Research,
    Analyze,
    Generate,
    Review,
    Benchmark,
    Decision,
    Custom(&'static str),
}

impl StepKind {
    pub fn name(&self) -> &str {
        match self {
            StepKind::Plan => "plan",
            StepKind::Execute => "execute",
            StepKind::Verify => "verify",
            StepKind::Reflect => "reflect",
            StepKind::Research => "research",
            StepKind::Analyze => "analyze",
            StepKind::Generate => "generate",
            StepKind::Review => "review",
            StepKind::Benchmark => "benchmark",
            StepKind::Decision => "decision",
            StepKind::Custom(s) => s,
        }
    }
}

/// A single step in an execution workflow.
#[derive(Debug, Clone, Serialize)]
pub struct WorkflowStep {
    pub id: String,
    pub kind: StepKind,
    pub label: String,
    pub description: String,

    /// Dependencies: step IDs that must complete before this one.
    pub depends_on: Vec<String>,

    /// Whether this step runs in parallel with its siblings.
    pub parallel: bool,

    /// Provider/domain hint for capability resolution.
    pub domain_hint: Option<String>,
    pub provider_hint: Option<String>,

    /// Estimated cost and complexity.
    pub estimated_cost: f64,
    pub estimated_duration_ms: u64,

    /// The output artifact key this step produces.
    pub output_key: Option<String>,
}

impl WorkflowStep {
    pub fn new(id: impl Into<String>, kind: StepKind, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            kind,
            label: label.into(),
            description: String::new(),
            depends_on: Vec::new(),
            parallel: false,
            domain_hint: None,
            provider_hint: None,
            estimated_cost: 0.0,
            estimated_duration_ms: 0,
            output_key: None,
        }
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn depends_on(mut self, dep: impl Into<String>) -> Self {
        self.depends_on.push(dep.into());
        self
    }

    pub fn parallel(mut self) -> Self {
        self.parallel = true;
        self
    }
}

/// The execution graph — a DAG of workflow steps.
#[derive(Debug, Clone, Serialize)]
pub struct ExecutionGraph {
    pub workflow_name: String,
    pub steps: Vec<WorkflowStep>,
    /// Topological ordering of step IDs.
    pub execution_order: Vec<String>,
}

impl ExecutionGraph {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            workflow_name: name.into(),
            steps: Vec::new(),
            execution_order: Vec::new(),
        }
    }

    pub fn add_step(&mut self, step: WorkflowStep) {
        let id = step.id.clone();
        self.steps.push(step);
        self.execution_order.push(id);
    }

    pub fn get_step(&self, id: &str) -> Option<&WorkflowStep> {
        self.steps.iter().find(|s| s.id == id)
    }

    /// Get the steps that have no unsatisfied dependencies.
    pub fn ready_steps(&self, completed: &[String]) -> Vec<&WorkflowStep> {
        self.steps
            .iter()
            .filter(|s| {
                !completed.contains(&s.id) && s.depends_on.iter().all(|dep| completed.contains(dep))
            })
            .collect()
    }

    /// Compute a topological ordering respecting dependencies.
    pub fn topological_sort(&self) -> Vec<String> {
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut order: Vec<String> = Vec::new();
        let mut stack: Vec<String> = self.steps.iter().map(|s| s.id.clone()).collect();

        while let Some(id) = stack.pop() {
            if visited.contains(&id) {
                continue;
            }
            if let Some(step) = self.steps.iter().find(|s| s.id == id) {
                let all_deps_visited = step.depends_on.iter().all(|dep| visited.contains(dep));
                if all_deps_visited {
                    visited.insert(id.clone());
                    order.push(id);
                } else {
                    stack.push(id);
                    for dep in &step.depends_on {
                        if !visited.contains(dep) {
                            stack.push(dep.clone());
                        }
                    }
                }
            }
        }
        order
    }

    pub fn parallel_steps(&self) -> Vec<&WorkflowStep> {
        self.steps.iter().filter(|s| s.parallel).collect()
    }

    pub fn sequential_steps(&self) -> Vec<&WorkflowStep> {
        self.steps.iter().filter(|s| !s.parallel).collect()
    }
}

/// A named, reusable workflow definition.
#[derive(Debug, Clone, Serialize)]
pub struct WorkflowDefinition {
    pub name: String,
    pub description: String,
    pub version: String,
    pub tags: Vec<String>,
    pub steps: Vec<WorkflowStep>,
}

impl WorkflowDefinition {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            version: "0.1.0".to_string(),
            tags: Vec::new(),
            steps: Vec::new(),
        }
    }

    pub fn with_step(mut self, step: WorkflowStep) -> Self {
        self.steps.push(step);
        self
    }

    pub fn instantiate(&self, _context: &RuntimeContext) -> ExecutionGraph {
        let mut graph = ExecutionGraph::new(&self.name);
        for step in &self.steps {
            graph.add_step(step.clone());
        }
        graph.execution_order = graph.topological_sort();
        graph
    }
}

/// The Workflow Engine — produces execution graphs from intents.
pub struct WorkflowEngine;

impl WorkflowEngine {
    /// Build an execution graph for a given intent and context.
    pub fn plan(context: &RuntimeContext, intent: &str) -> ExecutionGraph {
        // Simplified planning: creates a standard workflow based on context
        let mut graph = ExecutionGraph::new("auto-workflow");

        // Plan step
        let plan = WorkflowStep::new("plan", StepKind::Plan, "Plan execution")
            .with_description(format!("Plan: {}", intent));
        graph.add_step(plan);

        // Execute step (depends on plan)
        let execute = WorkflowStep::new("execute", StepKind::Execute, "Execute")
            .with_description("Execute the plan")
            .depends_on("plan");
        graph.add_step(execute);

        // Verify step (depends on execute)
        if context.properties.safety_level as u32 >= 2 {
            let verify = WorkflowStep::new("verify", StepKind::Verify, "Verify output")
                .with_description("Verify execution results")
                .depends_on("execute");
            graph.add_step(verify);
        }

        // Reflect step (optional)
        if context.properties.reasoning_depth > 1 {
            let reflect = WorkflowStep::new("reflect", StepKind::Reflect, "Reflect")
                .with_description("Reflect on execution")
                .depends_on("verify");
            graph.add_step(reflect);
        }

        graph.execution_order = graph.topological_sort();
        graph
    }

    /// Build a custom execution graph from explicit steps.
    pub fn build(steps: Vec<WorkflowStep>) -> ExecutionGraph {
        let mut graph = ExecutionGraph::new("custom-workflow");
        for step in steps {
            graph.add_step(step);
        }
        graph.execution_order = graph.topological_sort();
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_step_creation() {
        let step = WorkflowStep::new("step-1", StepKind::Plan, "Initial plan")
            .with_description("Plan the implementation");
        assert_eq!(step.id, "step-1");
        assert_eq!(step.kind, StepKind::Plan);
    }

    #[test]
    fn execution_graph_add_steps() {
        let mut graph = ExecutionGraph::new("test");
        graph.add_step(WorkflowStep::new("a", StepKind::Plan, "Plan"));
        graph.add_step(WorkflowStep::new("b", StepKind::Execute, "Execute"));
        assert_eq!(graph.steps.len(), 2);
    }

    #[test]
    fn topological_sort_respects_dependencies() {
        let mut graph = ExecutionGraph::new("deps");
        graph.add_step(WorkflowStep::new("b", StepKind::Execute, "Step B").depends_on("a"));
        graph.add_step(WorkflowStep::new("a", StepKind::Plan, "Step A"));
        let order = graph.topological_sort();
        assert_eq!(order.len(), 2);
        // A should come before B
        assert!(order.iter().position(|id| id == "a") < order.iter().position(|id| id == "b"));
    }

    #[test]
    fn ready_steps_tracks_completion() {
        let mut graph = ExecutionGraph::new("test");
        graph.add_step(WorkflowStep::new("b", StepKind::Execute, "B").depends_on("a"));
        graph.add_step(WorkflowStep::new("a", StepKind::Plan, "A"));

        // Before any completion: only A (no deps) is ready
        let ready = graph.ready_steps(&[]);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "a");

        // After A completes: B is ready
        let ready = graph.ready_steps(&["a".to_string()]);
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].id, "b");
    }

    #[test]
    fn workflow_definition_instantiate() {
        let def = WorkflowDefinition::new("code-review")
            .with_step(WorkflowStep::new("read", StepKind::Review, "Read code"))
            .with_step(
                WorkflowStep::new("analyze", StepKind::Analyze, "Analyze").depends_on("read"),
            );

        let ctx = RuntimeContext::new("test".to_string(), "proj".to_string());
        let graph = def.instantiate(&ctx);
        assert_eq!(graph.steps.len(), 2);
        assert_eq!(graph.workflow_name, "code-review");
    }

    #[test]
    fn workflow_engine_plans_from_context() {
        let ctx = RuntimeContext::new("test".to_string(), "proj".to_string());
        let graph = WorkflowEngine::plan(&ctx, "implement feature");
        assert!(!graph.steps.is_empty());
        assert!(graph.steps.iter().any(|s| s.kind == StepKind::Plan));
        assert!(graph.steps.iter().any(|s| s.kind == StepKind::Execute));
    }

    #[test]
    fn step_kind_names() {
        assert_eq!(StepKind::Plan.name(), "plan");
        assert_eq!(StepKind::Execute.name(), "execute");
        assert_eq!(StepKind::Verify.name(), "verify");
        assert_eq!(StepKind::Custom("hello").name(), "hello");
    }

    #[test]
    fn parallel_vs_sequential() {
        let mut graph = ExecutionGraph::new("test");
        graph.add_step(WorkflowStep::new("a", StepKind::Execute, "A").parallel());
        graph.add_step(WorkflowStep::new("b", StepKind::Execute, "B"));
        assert_eq!(graph.parallel_steps().len(), 1);
        assert_eq!(graph.sequential_steps().len(), 1);
    }
}
