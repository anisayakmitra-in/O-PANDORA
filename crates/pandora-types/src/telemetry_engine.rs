//! OpenTelemetry Engine — constitutional tracing across every subsystem.
//!
//! Every execution stage emits spans with timing, metadata, and status.
//! Spans form a tree (parent-child) representing the full execution flow.
//! The TUI renders live span trees.

use std::collections::HashMap;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Severity level for a span event.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpanLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl SpanLevel {
    pub fn name(&self) -> &'static str {
        match self { SpanLevel::Trace => "TRACE", SpanLevel::Debug => "DEBUG", SpanLevel::Info => "INFO", SpanLevel::Warn => "WARN", SpanLevel::Error => "ERROR", SpanLevel::Fatal => "FATAL" }
    }
}

/// A single telemetry event attached to a span.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub timestamp: DateTime<Utc>,
    pub level: SpanLevel,
    pub message: String,
    pub attributes: HashMap<String, String>,
}

impl SpanEvent {
    pub fn new(level: SpanLevel, message: impl Into<String>) -> Self {
        Self { timestamp: Utc::now(), level, message: message.into(), attributes: HashMap::new() }
    }
}

/// Status of a completed span.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpanStatus {
    Ok,
    Error,
    Cancelled,
}

/// A single OpenTelemetry span representing one execution stage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Span {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub trace_id: String,
    pub name: String,
    pub stage: String,
    pub provider: String,
    pub model: String,
    pub status: SpanStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_ms: u64,
    pub events: Vec<SpanEvent>,
    pub attributes: HashMap<String, String>,
}

impl Span {
    pub fn new(trace_id: impl Into<String>, name: impl Into<String>, stage: impl Into<String>) -> Self {
        Self {
            span_id: format!("span-{:x}", 42u64),
            parent_span_id: None,
            trace_id: trace_id.into(),
            name: name.into(),
            stage: stage.into(),
            provider: String::new(),
            model: String::new(),
            status: SpanStatus::Ok,
            start_time: Utc::now(),
            end_time: None,
            duration_ms: 0,
            events: Vec::new(),
            attributes: HashMap::new(),
        }
    }

    pub fn child_of(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_span_id = Some(parent_id.into());
        self
    }
}

/// A complete trace — a tree of spans from a single execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trace {
    pub trace_id: String,
    pub execution_id: String,
    pub task: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub spans: Vec<Span>,
    pub total_duration_ms: u64,
    pub span_count: u64,
}

impl Trace {
    pub fn new(trace_id: impl Into<String>, execution_id: impl Into<String>, task: impl Into<String>) -> Self {
        Self {
            trace_id: trace_id.into(),
            execution_id: execution_id.into(),
            task: task.into(),
            start_time: Utc::now(),
            end_time: None,
            spans: Vec::new(),
            total_duration_ms: 0,
            span_count: 0,
        }
    }

    /// Add a span to this trace.
    pub fn add_span(&mut self, span: Span) {
        self.span_count += 1;
        self.spans.push(span);
    }

    /// Get root spans (no parent).
    pub fn root_spans(&self) -> Vec<&Span> {
        self.spans.iter().filter(|s| s.parent_span_id.is_none()).collect()
    }

    /// Get children of a span.
    pub fn child_spans(&self, parent_id: &str) -> Vec<&Span> {
        self.spans.iter().filter(|s| s.parent_span_id.as_deref() == Some(parent_id)).collect()
    }

    /// Render trace as an indented tree string.
    pub fn format_tree(&self) -> String {
        let mut out = format!("TRACE: {} ({})\n", self.task, self.trace_id);
        for root in self.root_spans() {
            self.format_span(&mut out, root, 0);
        }
        out
    }

    fn format_span(&self, out: &mut String, span: &Span, depth: usize) {
        let indent = "  ".repeat(depth);
        let status = match span.status { SpanStatus::Ok => "✓", SpanStatus::Error => "✗", SpanStatus::Cancelled => "–" };
        let duration = if span.duration_ms > 0 { format!("{}ms", span.duration_ms) } else { "running".to_string() };
        out.push_str(&format!("{}{} [{}] {} ({})\n", indent, status, span.stage, span.name, duration));
        for child in self.child_spans(&span.span_id) {
            self.format_span(out, child, depth + 1);
        }
    }
}

/// The OpenTelemetry Engine — manages traces and spans for all execution.
#[derive(Debug, Clone)]
pub struct TelemetryEngine {
    traces: HashMap<String, Trace>,
    active_trace: Option<String>,
    span_id_counter: u64,
    max_traces: usize,
}

impl TelemetryEngine {
    pub fn new() -> Self {
        Self { traces: HashMap::new(), active_trace: None, span_id_counter: 0, max_traces: 1000 }
    }

    pub fn with_max_traces(max: usize) -> Self { Self { traces: HashMap::new(), active_trace: None, span_id_counter: 0, max_traces: max } }

    /// Begin a new trace for an execution.
    pub fn begin_trace(&mut self, execution_id: impl Into<String>, task: impl Into<String>) -> String {
        let trace_id = format!("trace-{:x}", self.span_id_counter);
        self.span_id_counter += 1;
        let trace = Trace::new(&trace_id, execution_id, task);
        if self.traces.len() >= self.max_traces { self.traces.clear(); }
        self.traces.insert(trace_id.clone(), trace);
        self.active_trace = Some(trace_id.clone());
        trace_id
    }

    /// End the active trace with final timing.
    pub fn end_trace(&mut self, trace_id: &str) {
        if let Some(trace) = self.traces.get_mut(trace_id) {
            trace.end_time = Some(Utc::now());
            trace.total_duration_ms = trace.spans.iter().map(|s| s.duration_ms).sum();
        }
        self.active_trace = None;
    }

    /// Begin a new span within the active trace.
    pub fn begin_span(&mut self, trace_id: &str, name: impl Into<String>, stage: impl Into<String>) -> String {
        let span_id = format!("span-{:x}", self.span_id_counter);
        self.span_id_counter += 1;
        let mut span = Span::new(trace_id, name, stage);
        span.span_id = span_id.clone();
        if let Some(trace) = self.traces.get_mut(trace_id) {
            trace.add_span(span);
        }
        span_id
    }

    /// End a span with final status and duration.
    pub fn end_span(&mut self, trace_id: &str, span_id: &str, status: SpanStatus, duration_ms: u64) {
        if let Some(trace) = self.traces.get_mut(trace_id) {
            if let Some(span) = trace.spans.iter_mut().find(|s| s.span_id == span_id) {
                span.end_time = Some(Utc::now());
                span.status = status;
                span.duration_ms = duration_ms;
            }
        }
    }

    /// Add an event to a span.
    pub fn add_event(&mut self, trace_id: &str, span_id: &str, level: SpanLevel, message: impl Into<String>) {
        if let Some(trace) = self.traces.get_mut(trace_id) {
            if let Some(span) = trace.spans.iter_mut().find(|s| s.span_id == span_id) {
                span.events.push(SpanEvent::new(level, message));
            }
        }
    }

    /// Get a trace by ID.
    pub fn get_trace(&self, trace_id: &str) -> Option<&Trace> {
        self.traces.get(trace_id)
    }

    /// List all traces, most recent first.
    pub fn list_traces(&self) -> Vec<&Trace> {
        let mut v: Vec<&Trace> = self.traces.values().collect();
        v.sort_by(|a, b| b.start_time.cmp(&a.start_time));
        v
    }

    /// Find traces with errors.
    pub fn failed_traces(&self) -> Vec<&Trace> {
        self.traces.values().filter(|t| t.spans.iter().any(|s| s.status == SpanStatus::Error)).collect()
    }

    pub fn trace_count(&self) -> usize { self.traces.len() }
}

impl Default for TelemetryEngine {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begin_and_end_trace() {
        let mut engine = TelemetryEngine::new();
        let tid = engine.begin_trace("exec-1", "Implement feature");
        engine.end_trace(&tid);
        assert_eq!(engine.trace_count(), 1);
        assert!(engine.get_trace(&tid).is_some());
    }

    #[test]
    fn span_lifecycle() {
        let mut engine = TelemetryEngine::new();
        let tid = engine.begin_trace("exec-1", "Coding task");
        let sid = engine.begin_span(&tid, "Plan architecture", "planning");
        engine.end_span(&tid, &sid, SpanStatus::Ok, 1500);
        let trace = engine.get_trace(&tid).unwrap();
        assert_eq!(trace.spans.len(), 1);
        assert_eq!(trace.spans[0].duration_ms, 1500);
    }

    #[test]
    fn span_events() {
        let mut engine = TelemetryEngine::new();
        let tid = engine.begin_trace("exec-1", "Research");
        let sid = engine.begin_span(&tid, "Search papers", "research");
        engine.add_event(&tid, &sid, SpanLevel::Info, "Found 12 papers");
        engine.add_event(&tid, &sid, SpanLevel::Warn, "Rate limited");
        let trace = engine.get_trace(&tid).unwrap();
        assert_eq!(trace.spans[0].events.len(), 2);
    }

    #[test]
    fn trace_tree_format() {
        let mut engine = TelemetryEngine::new();
        let tid = engine.begin_trace("exec-1", "Test");
        let s1 = engine.begin_span(&tid, "Root step", "plan");
        let s2 = engine.begin_span(&tid, "Child step", "execute");
        engine.end_span(&tid, &s2, SpanStatus::Ok, 500);
        engine.end_span(&tid, &s1, SpanStatus::Ok, 1000);
        let trace = engine.get_trace(&tid).unwrap();
        let tree = trace.format_tree();
        assert!(tree.contains("Test"));
        assert!(tree.contains("plan"));
        assert!(tree.contains("execute"));
    }

    #[test]
    fn failed_traces_filter() {
        let mut engine = TelemetryEngine::new();
        let tid_ok = engine.begin_trace("exec-ok", "Good");
        engine.end_trace(&tid_ok);
        let tid_fail = engine.begin_trace("exec-fail", "Bad");
        let sid = engine.begin_span(&tid_fail, "Failing step", "execute");
        engine.end_span(&tid_fail, &sid, SpanStatus::Error, 3000);
        engine.end_trace(&tid_fail);
        let failed = engine.failed_traces();
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].execution_id, "exec-fail");
    }

    #[test]
    fn span_status_names() {
        assert_eq!(SpanStatus::Ok as u8, 0);
        assert_eq!(SpanStatus::Error as u8, 1);
    }
}
