//! Telemetry Engine — constitutional tracing across every subsystem.
//! Every execution stage emits spans with timing, metadata, and status.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Fatal => "FATAL",
        }
    }
}

/// A telemetry span — one stage in the execution trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetrySpan {
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub trace_id: String,
    pub name: String,
    pub level: SpanLevel,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_ms: u64,
    pub status: SpanStatus,
    pub attributes: HashMap<String, String>,
    pub events: Vec<SpanEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpanStatus {
    Ok,
    Error(String),
    Unset,
}

/// An event within a span.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanEvent {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub attributes: HashMap<String, String>,
}

/// A complete execution trace — tree of spans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionTrace {
    pub trace_id: String,
    pub root_span_id: Option<String>,
    pub spans: Vec<TelemetrySpan>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub total_duration_ms: u64,
}

impl TelemetrySpan {
    pub fn new(trace_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            span_id: format!("span-{:016x}", rand::random::<u64>()),
            parent_span_id: None,
            trace_id: trace_id.into(),
            name: name.into(),
            level: SpanLevel::Info,
            started_at: Utc::now(),
            ended_at: None,
            duration_ms: 0,
            status: SpanStatus::Unset,
            attributes: HashMap::new(),
            events: Vec::new(),
        }
    }
    pub fn child_of(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_span_id = Some(parent_id.into());
        self
    }
    pub fn add_event(&mut self, name: impl Into<String>) {
        self.events.push(SpanEvent {
            name: name.into(),
            timestamp: Utc::now(),
            attributes: HashMap::new(),
        });
    }
    pub fn finish(&mut self) {
        self.ended_at = Some(Utc::now());
        if let (Some(start), Some(end)) = (Some(self.started_at), self.ended_at) {
            self.duration_ms = end.signed_duration_since(start).num_milliseconds().max(0) as u64;
        }
    }
}

/// The Telemetry Engine — manages span hierarchies and traces.
#[derive(Debug, Clone)]
pub struct TelemetryEngine {
    traces: Vec<ExecutionTrace>,
    current_trace_id: Option<String>,
}

impl TelemetryEngine {
    pub fn new() -> Self {
        Self {
            traces: Vec::new(),
            current_trace_id: None,
        }
    }

    pub fn begin_trace(&mut self, trace_id: impl Into<String>, _task: impl Into<String>) -> String {
        let tid = trace_id.into();
        self.traces.push(ExecutionTrace {
            trace_id: tid.clone(),
            root_span_id: None,
            spans: Vec::new(),
            started_at: Utc::now(),
            ended_at: None,
            total_duration_ms: 0,
        });
        self.current_trace_id = Some(tid.clone());
        tid
    }

    pub fn begin_span(
        &mut self,
        _trace_id: impl Into<String>,
        name: impl Into<String>,
        _kind: impl Into<String>,
    ) -> String {
        let tid = self.current_trace_id.clone().unwrap_or_else(|| {
            self.begin_trace(format!("trace-{:016x}", rand::random::<u64>()), "auto")
        });
        if let Some(trace) = self.traces.iter_mut().find(|t| t.trace_id == tid) {
            let span = TelemetrySpan::new(&tid, name);
            let id = span.span_id.clone();
            if trace.root_span_id.is_none() {
                trace.root_span_id = Some(id.clone());
            }
            trace.spans.push(span);
            id
        } else {
            String::new()
        }
    }

    pub fn add_span(&mut self, span: TelemetrySpan) {
        let tid = span.trace_id.clone();
        if let Some(trace) = self.traces.iter_mut().find(|t| t.trace_id == tid) {
            trace.spans.push(span);
        }
    }

    pub fn end_trace(&mut self, trace_id: &str) {
        if let Some(trace) = self.traces.iter_mut().find(|t| t.trace_id == trace_id) {
            trace.ended_at = Some(Utc::now());
        }
    }

    pub fn trace_count(&self) -> usize {
        self.traces.len()
    }
    pub fn span_count(&self) -> usize {
        self.traces.iter().map(|t| t.spans.len()).sum()
    }
    pub fn get_trace(&self, id: &str) -> Option<&ExecutionTrace> {
        self.traces.iter().find(|t| t.trace_id == id)
    }
    pub fn traces(&self) -> &[ExecutionTrace] {
        &self.traces
    }
    pub fn clear(&mut self) {
        self.traces.clear();
    }
}

impl Default for TelemetryEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate telemetry metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TelemetryMetrics {
    pub total_executions: u64,
    pub total_errors: u64,
    pub avg_duration_ms: f64,
    pub max_duration_ms: u64,
    pub total_spans: u64,
    pub spans_by_level: HashMap<String, u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn span_creation() {
        let s = TelemetrySpan::new("trace-1", "test");
        assert_eq!(s.name, "test");
        assert_eq!(s.trace_id, "trace-1");
    }

    #[test]
    fn begin_trace_and_span() {
        let mut engine = TelemetryEngine::new();
        engine.begin_trace("trace-1", "test");
        let sid = engine.begin_span("trace-1", "plan", "test");
        assert!(!sid.is_empty());
        assert_eq!(engine.trace_count(), 1);
    }

    #[test]
    fn span_finish_sets_duration() {
        let mut s = TelemetrySpan::new("t1", "test");
        s.finish();
        assert!(s.ended_at.is_some());
    }

    #[test]
    fn trace_metrics() {
        let mut engine = TelemetryEngine::new();
        engine.begin_trace("t1", "test");
        engine.begin_span("t1", "a", "test");
        engine.begin_span("t1", "b", "test");
        assert_eq!(engine.span_count(), 2);
    }
}
