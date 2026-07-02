//! Constitutional Meta Harness contract.
//!
//! Every Meta Harness in Pandora participates in a
//! three-phase lifecycle: RAHU, Core Domain, KETU.
//!
//! This module defines the **contract** that every
//! Meta Harness implements. It is the constitution of
//! the Meta Harness layer. No business logic lives here:
//! each Meta Harness supplies its own RAHU routing, Core
//! domain, and KETU validation. The contract is the
//! shape they all share.
//!
//! ## Architecture
//!
//! MetaHarness (existing, in )
//!     |
//!     v
//! ConstitutionalHarness (this module, the contract)
//!     |
//!     +-- rahu_phase()    -> RahuContext
//!     +-- core_phase()    -> CoreContext
//!     +-- ketu_phase()    -> KetuContext
//!     |
//!     v
//! MetaHarnessExecutor (executes the contract)
//!
//! ## Design rules
//!
//! - The contract is a trait. Every Meta Harness
//!   implements it. The implementation is whatever the
//!   Meta Harness needs to do (Phoenix: execution;
//!   ANUBIS: memory; etc.).
//! - RAHU and KETU are not standalone Meta Harnesses.
//!   They are lifecycle phases implemented inside every
//!   Meta Harness.
//! - The contract is Send + Sync so the runtime can
//!   dispatch it on any executor.
//! - A Meta Harness that needs richer RAHU/KETU logic
//!   extends the default. A Meta Harness that just needs
//!   the canonical defaults implements the trait with the
//!   no-op implementations.

use std::fmt;

use serde::{Deserialize, Serialize};

use crate::harness::MetaHarness;

/// The RAHU phase context. This is what the RAHU
/// phase of a Meta Harness produces: the routing,
/// capability requests, and execution plan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RahuContext {
    pub phase_name: String,
    pub plan_summary: String,
    pub capabilities_requested: Vec<String>,
    pub sources_resolved: Vec<String>,
    pub notes: Vec<String>,
}

impl RahuContext {
    pub fn new(phase_name: impl Into<String>) -> Self {
        RahuContext {
            phase_name: phase_name.into(),
            plan_summary: String::new(),
            capabilities_requested: Vec::new(),
            sources_resolved: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_plan_summary(mut self, s: impl Into<String>) -> Self {
        self.plan_summary = s.into();
        self
    }

    pub fn with_capability(mut self, c: impl Into<String>) -> Self {
        self.capabilities_requested.push(c.into());
        self
    }

    pub fn with_source(mut self, s: impl Into<String>) -> Self {
        self.sources_resolved.push(s.into());
        self
    }

    pub fn with_note(mut self, n: impl Into<String>) -> Self {
        self.notes.push(n.into());
        self
    }
}

/// The Core Domain phase context. The Core phase is
/// the work the Meta Harness actually does. The
/// context is intentionally generic: each Meta
/// Harness supplies its own domain object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreContext {
    pub phase_name: String,
    pub description: String,
    pub domain_repr: Option<String>,
}

impl CoreContext {
    pub fn new(phase_name: impl Into<String>, description: impl Into<String>) -> Self {
        CoreContext {
            phase_name: phase_name.into(),
            description: description.into(),
            domain_repr: None,
        }
    }

    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain_repr = Some(domain.into());
        self
    }
}

/// A validation finding. KETU accumulates findings
/// during the validation phase. Each finding has a
/// severity and a description.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: FindingSeverity,
    pub description: String,
    pub source: String,
}

impl ValidationFinding {
    pub fn new(
        severity: FindingSeverity,
        description: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        ValidationFinding {
            severity,
            description: description.into(),
            source: source.into(),
        }
    }

    pub fn is_blocking(&self) -> bool {
        matches!(self.severity, FindingSeverity::Critical)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl FindingSeverity {
    pub fn as_str(self) -> &'static str {
        match self {
            FindingSeverity::Info => "INFO",
            FindingSeverity::Warning => "WARNING",
            FindingSeverity::Error => "ERROR",
            FindingSeverity::Critical => "CRITICAL",
        }
    }
}

impl fmt::Display for FindingSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The KETU phase context. KETU produces a
///  after validating the Core phase
/// output. This is the contract every Meta Harness
/// implements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KetuContext {
    pub phase_name: String,
    pub report: ValidationReport,
}

impl KetuContext {
    pub fn from_report(phase_name: impl Into<String>, report: ValidationReport) -> Self {
        KetuContext {
            phase_name: phase_name.into(),
            report,
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.report.is_accepted()
    }
}

/// A validation report. KETU produces this. The report
/// captures:
///
/// - the overall confidence
/// - the integrity score
/// - which checks were performed
/// - any findings (warnings, errors, critical issues)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    pub accepted: bool,
    pub confidence: f32,
    pub integrity_score: f32,
    pub replay_validated: bool,
    pub benchmark_verified: bool,
    pub constitutional_check: bool,
    pub governance_validated: bool,
    pub capability_validated: bool,
    pub findings: Vec<ValidationFinding>,
    pub notes: Vec<String>,
}

impl ValidationReport {
    pub fn placeholder() -> Self {
        ValidationReport {
            accepted: false,
            confidence: 0.0,
            integrity_score: 0.0,
            replay_validated: false,
            benchmark_verified: false,
            constitutional_check: false,
            governance_validated: false,
            capability_validated: false,
            findings: Vec::new(),
            notes: vec!["placeholder validation".to_string()],
        }
    }

    pub fn accepted(confidence: f32) -> Self {
        ValidationReport {
            accepted: true,
            confidence: confidence.clamp(0.0, 1.0),
            integrity_score: confidence.clamp(0.0, 1.0),
            replay_validated: true,
            benchmark_verified: true,
            constitutional_check: true,
            governance_validated: true,
            capability_validated: true,
            findings: Vec::new(),
            notes: vec!["all checks passed".to_string()],
        }
    }

    pub fn rejected(reason: impl Into<String>) -> Self {
        let mut report = ValidationReport::placeholder();
        report.findings.push(ValidationFinding::new(
            FindingSeverity::Critical,
            reason.into(),
            "ketu",
        ));
        report
    }

    pub fn with_finding(mut self, f: ValidationFinding) -> Self {
        self.findings.push(f);
        self
    }

    pub fn is_accepted(&self) -> bool {
        self.accepted
    }

    pub fn has_blocking(&self) -> bool {
        self.findings.iter().any(|f| f.is_blocking())
    }
}

/// The full lifecycle result. Every Meta Harness
/// produces a  after running its
/// three-phase cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleResult {
    pub meta_harness_name: String,
    pub rahu: RahuContext,
    pub core: CoreContext,
    pub ketu: KetuContext,
}

impl LifecycleResult {
    pub fn new(
        meta_harness_name: impl Into<String>,
        rahu: RahuContext,
        core: CoreContext,
        ketu: KetuContext,
    ) -> Self {
        LifecycleResult {
            meta_harness_name: meta_harness_name.into(),
            rahu,
            core,
            ketu,
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.ketu.is_accepted()
    }

    pub fn confidence(&self) -> f32 {
        self.ketu.report.confidence
    }
}

/// The contract every Meta Harness implements. This
/// trait extends the existing  trait so
/// existing registries continue to work. New Meta
/// Harnesses can implement either or both.
pub trait ConstitutionalHarness: MetaHarness {
    /// Run the RAHU phase. The implementation is
    /// domain-specific: Phoenix plans execution,
    /// ANUBIS plans memory access, etc. The default
    /// implementation returns an empty RAHU context.
    fn rahu_phase(&self) -> RahuContext {
        RahuContext::new(format!("{}::rahu", self.name()))
    }

    /// Run the Core Domain phase. The implementation
    /// is the actual domain work. The default
    /// implementation returns an empty Core context.
    fn core_phase(&self) -> CoreContext {
        CoreContext::new(
            format!("{}::core", self.name()),
            "no core phase implementation",
        )
    }

    /// Run the KETU phase. The implementation validates
    /// the Core phase output. The default implementation
    /// returns a placeholder validation report.
    fn ketu_phase(&self, _core: &CoreContext) -> KetuContext {
        KetuContext::from_report(
            format!("{}::ketu", self.name()),
            ValidationReport::placeholder(),
        )
    }

    /// Run the full RAHU -> Core -> KETU cycle. The
    /// default implementation composes the three
    /// phases. Meta Harnesses rarely need to override
    /// this; they override the individual phase
    /// methods instead.
    fn run_lifecycle(&self) -> LifecycleResult {
        let rahu = self.rahu_phase();
        let core = self.core_phase();
        let ketu = self.ketu_phase(&core);
        LifecycleResult::new(self.name().to_string(), rahu, core, ketu)
    }
}

/// Helper: build a  from a
///  reference. Convenience
/// wrapper around .
pub fn run(harness: &dyn ConstitutionalHarness) -> LifecycleResult {
    harness.run_lifecycle()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::harness::{MetaHarnessKind, SourceHarnessKind};
    use pandora_types::constitutional::{ConstitutionalManifest, ManifestKind, ManifestVersion};
    use std::sync::OnceLock;

    struct StubMeta {
        manifest: ConstitutionalManifest,
    }
    impl MetaHarness for StubMeta {
        fn meta_kind(&self) -> MetaHarnessKind {
            MetaHarnessKind::General
        }
        fn parent(&self) -> SourceHarnessKind {
            SourceHarnessKind::Phoenix
        }
        fn manifest(&self) -> &ConstitutionalManifest {
            // Use OnceLock to satisfy 'static requirement
            // without making the manifest mutable.
            static M: OnceLock<ConstitutionalManifest> = OnceLock::new();
            M.get_or_init(|| self.manifest.clone())
        }
    }

    fn stub() -> StubMeta {
        StubMeta {
            manifest: ConstitutionalManifest::new(
                "phoenix-default",
                ManifestKind::MetaHarness,
                ManifestVersion::new(0, 1, 0),
                "Phoenix default meta harness",
            ),
        }
    }

    impl ConstitutionalHarness for StubMeta {}

    #[test]
    fn rahu_context_builder() {
        let r = RahuContext::new("phoenix::rahu")
            .with_plan_summary("plan")
            .with_capability("fs")
            .with_source("phoenix")
            .with_note("n");
        assert_eq!(r.phase_name, "phoenix::rahu");
        assert_eq!(r.capabilities_requested, vec!["fs"]);
        assert_eq!(r.sources_resolved, vec!["phoenix"]);
    }

    #[test]
    fn core_context_builder() {
        let c = CoreContext::new("phoenix::core", "exec").with_domain("data");
        assert_eq!(c.domain_repr, Some("data".to_string()));
    }

    #[test]
    fn validation_report_placeholder_rejected() {
        let r = ValidationReport::placeholder();
        assert!(!r.is_accepted());
        assert!(!r.has_blocking());
    }

    #[test]
    fn validation_report_accepted_passed() {
        let r = ValidationReport::accepted(0.9);
        assert!(r.is_accepted());
        assert!(!r.has_blocking());
        assert_eq!(r.confidence, 0.9);
    }

    #[test]
    fn validation_report_rejected_has_blocking_finding() {
        let r = ValidationReport::rejected("bad input");
        assert!(!r.is_accepted());
        assert!(r.has_blocking());
    }

    #[test]
    fn validation_finding_severity() {
        let f = ValidationFinding::new(FindingSeverity::Critical, "x", "y");
        assert!(f.is_blocking());
        let g = ValidationFinding::new(FindingSeverity::Warning, "x", "y");
        assert!(!g.is_blocking());
    }

    #[test]
    fn finding_severity_ordering() {
        assert!(FindingSeverity::Critical > FindingSeverity::Error);
        assert!(FindingSeverity::Error > FindingSeverity::Warning);
        assert!(FindingSeverity::Warning > FindingSeverity::Info);
    }

    #[test]
    fn default_lifecycle_runs_three_phases() {
        let s = stub();
        let result = s.run_lifecycle();
        assert_eq!(result.rahu.phase_name, "phoenix-default::rahu");
        assert_eq!(result.core.phase_name, "phoenix-default::core");
        assert_eq!(result.ketu.phase_name, "phoenix-default::ketu");
        // Default is placeholder (rejected).
        assert!(!result.is_accepted());
    }

    #[test]
    fn override_via_newtype() {
        // A newtype around a MetaHarness can
        // implement ConstitutionalHarness with
        // custom phases. This is the recommended
        // pattern for community Meta Harnesses.
        struct AnubisHarness;
        impl MetaHarness for AnubisHarness {
            fn meta_kind(&self) -> MetaHarnessKind {
                MetaHarnessKind::Memory
            }
            fn parent(&self) -> SourceHarnessKind {
                SourceHarnessKind::Anubis
            }
            fn manifest(&self) -> &ConstitutionalManifest {
                static M: OnceLock<ConstitutionalManifest> = OnceLock::new();
                M.get_or_init(|| {
                    ConstitutionalManifest::new(
                        "anubis-default",
                        ManifestKind::MetaHarness,
                        ManifestVersion::new(0, 1, 0),
                        "Anubis default meta harness",
                    )
                })
            }
        }
        impl ConstitutionalHarness for AnubisHarness {
            fn rahu_phase(&self) -> RahuContext {
                RahuContext::new("anubis::rahu")
                    .with_plan_summary("memory plan")
                    .with_capability("memory")
            }
            fn ketu_phase(&self, _: &CoreContext) -> KetuContext {
                KetuContext::from_report("anubis::ketu", ValidationReport::accepted(0.95))
            }
        }
        let result = AnubisHarness.run_lifecycle();
        assert!(result.is_accepted());
        assert_eq!(result.confidence(), 0.95);
        assert_eq!(result.rahu.phase_name, "anubis::rahu");
        assert!(result.rahu.plan_summary.contains("memory"));
    }

    #[test]
    fn run_helper_works() {
        let s = stub();
        let result = run(&s);
        assert_eq!(result.meta_harness_name, "phoenix-default");
    }
}
