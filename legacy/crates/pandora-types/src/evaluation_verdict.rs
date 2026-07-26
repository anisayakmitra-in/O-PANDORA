//! EvaluationVerdict — structured evaluator result with score and diagnostics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Structured result from an evaluator gene.
///
/// Contains a confidence score, per-criterion results, diagnostics, and
/// arbitrary metadata. The `ExecutionController` uses the score and
/// diagnostics to decide whether to retry, escalate, or complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationVerdict {
    /// Confidence score (0.0–1.0), higher means more confident.
    pub score: f32,
    /// Per-criterion evaluation results.
    pub criteria: Vec<Criterion>,
    /// Machine-actionable diagnostics (errors, warnings, info).
    pub diagnostics: Vec<Diagnostic>,
    /// Arbitrary key-value metadata for extensibility.
    pub metadata: HashMap<String, String>,
}

impl EvaluationVerdict {
    /// Create a new verdict with the given score and no criteria or diagnostics.
    pub fn new(score: f32) -> Self {
        Self {
            score,
            criteria: Vec::new(),
            diagnostics: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Convenience constructor: evaluator passed with the given confidence.
    pub fn pass(score: f32) -> Self {
        Self::new(score)
    }

    /// Convenience constructor: evaluator failed.
    /// Adds an error diagnostic with the given reason.
    pub fn fail(score: f32, reason: &str) -> Self {
        let mut verdict = Self::new(score);
        verdict
            .diagnostics
            .push(Diagnostic::error("evaluator", reason));
        verdict
    }
}

/// A single evaluation criterion with pass/fail and score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criterion {
    /// Criterion name, e.g. "compiles", "tests-pass".
    pub name: String,
    /// Whether this criterion was met.
    pub passed: bool,
    /// Score for this criterion (0.0–1.0).
    pub score: f32,
    /// Human-readable detail about this criterion's evaluation.
    pub detail: String,
}

/// A machine-actionable diagnostic message from an evaluator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Source of the diagnostic, e.g. "cargo", "pytest".
    pub source: String,
    /// Severity level.
    pub severity: Severity,
    /// Machine-readable error code, e.g. "E0432".
    pub code: String,
    /// Human-readable message.
    pub message: String,
    /// Suggested fix or next step.
    pub recommendation: String,
}

impl Diagnostic {
    /// Create an error-level diagnostic.
    pub fn error(source: &str, message: &str) -> Self {
        Self {
            source: source.into(),
            severity: Severity::Error,
            code: String::new(),
            message: message.into(),
            recommendation: String::new(),
        }
    }

    /// Create a warning-level diagnostic.
    pub fn warn(source: &str, message: &str) -> Self {
        Self {
            source: source.into(),
            severity: Severity::Warning,
            code: String::new(),
            message: message.into(),
            recommendation: String::new(),
        }
    }
}

#[non_exhaustive]
/// Severity level for diagnostics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}
