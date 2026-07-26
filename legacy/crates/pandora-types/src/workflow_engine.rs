//! Workflow Engine — defines WHAT gets executed.
//!
//! Produces an `ExecutionGraph` (a DAG of steps) that the
//! `ExecutionController` consumes. Workflow answers: "What must happen?"
//! The controller answers: "How long do we keep improving?"
//! One workflow, many control strategies.

use crate::runtime_context::RuntimeContext;
use serde::Serialize;
use std::collections::HashSet;

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
            Self::Plan => "plan",
            Self::Execute => "execute",
            Self::Verify => "verify",
            Self::Reflect => "reflect",
            Self::Research => "research",
            Self::Analyze => "analyze",
            Self::Generate => "generate",
            Self::Review => "review",
            Self::Benchmark => "benchmark",
            Self::Decision => "decision",
            Self::Custom(s) => s,
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
    pub depends_on: Vec<String>,
    pub parallel: bool,
    pub domain_hint: Option<String>,
    pub provider_hint: Option<String>,
    pub estimated_cost: f64,
    pub estimated_duration_ms: u64,
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
    pub fn ready_steps(&self, completed: &[String]) -> Vec<&WorkflowStep> {
        self.steps
            .iter()
            .filter(|s| {
                !completed.contains(&s.id) && s.depends_on.iter().all(|dep| completed.contains(dep))
            })
            .collect()
    }
    pub fn topological_sort(&self) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut order = Vec::new();
        let mut stack: Vec<String> = self.steps.iter().map(|s| s.id.clone()).collect();
        let mut deferred = HashSet::new();
        while let Some(id) = stack.pop() {
            if visited.contains(&id) {
                continue;
            }
            if let Some(step) = self.steps.iter().find(|s| s.id == id) {
                if step.depends_on.iter().all(|dep| visited.contains(dep)) || deferred.contains(&id)
                {
                    visited.insert(id.clone());
                    order.push(id);
                } else {
                    deferred.insert(id.clone());
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
            version: "0.2.0".into(),
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
    pub fn plan(context: &RuntimeContext, intent: &str) -> ExecutionGraph {
        let mut graph = ExecutionGraph::new("auto-workflow");
        graph.add_step(
            WorkflowStep::new("plan", StepKind::Plan, "Plan execution")
                .with_description(format!("Plan: {intent}")),
        );
        graph.add_step(
            WorkflowStep::new("execute", StepKind::Execute, "Execute")
                .with_description("Execute the plan")
                .depends_on("plan"),
        );
        if context.properties.safety_level as u32 >= 2 {
            graph.add_step(
                WorkflowStep::new("verify", StepKind::Verify, "Verify output")
                    .with_description("Verify execution results")
                    .depends_on("execute"),
            );
        }
        if context.properties.reasoning_depth > 1 {
            graph.add_step(
                WorkflowStep::new("reflect", StepKind::Reflect, "Reflect")
                    .with_description("Reflect on execution")
                    .depends_on("verify"),
            );
        }
        graph.execution_order = graph.topological_sort();
        graph
    }

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
        let mut g = ExecutionGraph::new("test");
        g.add_step(WorkflowStep::new("a", StepKind::Plan, "Plan"));
        g.add_step(WorkflowStep::new("b", StepKind::Execute, "Execute"));
        assert_eq!(g.steps.len(), 2);
    }

    #[test]
    fn topological_sort_respects_dependencies() {
        let mut g = ExecutionGraph::new("deps");
        g.add_step(WorkflowStep::new("b", StepKind::Execute, "B").depends_on("a"));
        g.add_step(WorkflowStep::new("a", StepKind::Plan, "A"));
        let order = g.topological_sort();
        assert!(order.iter().position(|id| id == "a") < order.iter().position(|id| id == "b"));
    }

    #[test]
    fn ready_steps_tracks_completion() {
        let mut g = ExecutionGraph::new("test");
        g.add_step(WorkflowStep::new("b", StepKind::Execute, "B").depends_on("a"));
        g.add_step(WorkflowStep::new("a", StepKind::Plan, "A"));
        assert_eq!(g.ready_steps(&[]).len(), 1);
        assert_eq!(g.ready_steps(&["a".into()])[0].id, "b");
    }

    #[test]
    fn workflow_engine_plans() {
        let ctx = RuntimeContext::new(String::from("test"), String::from("proj"));
        let graph = WorkflowEngine::plan(&ctx, "implement feature");
        assert!(graph.steps.iter().any(|s| s.kind == StepKind::Plan));
    }

    #[test]
    fn step_kind_names() {
        assert_eq!(StepKind::Plan.name(), "plan");
        assert_eq!(StepKind::Custom("hello").name(), "hello");
    }
}
