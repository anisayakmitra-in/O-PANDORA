//! Constitutional lifecycle for every Meta Harness.
//!
//! Every Meta Harness in Pandora participates in three
//! phases:
//!
//! 1. **RAHU** (planning) — resolves which source harness,
//!    meta harness, gene, and capabilities the runtime
//!    needs to satisfy a request.
//!
//! 2. **Core Domain** (execution) — does the actual work
//!    (execution, memory, decision, identity, evolution,
//!    ...). The Core phase is owned by the meta harness
//!    itself. The lifecycle is intentionally agnostic
//!    about what the Core phase does.
//!
//! 3. **KETU** (validation) — validates the result of the
//!    Core phase. KETU computes a confidence score, checks
//!    integrity, verifies replay, runs constitutional
//!    checks, and emits a  record.
//!
//! The lifecycle is **constitutional**: every Meta Harness
//! participates in it. RAHU and KETU are not independent
//! Source Harnesses; they are lifecycle stages that wrap
//! the Core phase.
//!
//! ## Design rules
//!
//! - The lifecycle is generic over the three phase types
//!   so any Source Harness can adopt it without code
//!   change to RAHU or KETU.
//! - The Core phase is  so
//!   each Meta Harness can carry its own domain object.
//! - KETU never executes. KETU never plans. KETU only
//!   validates the output of the Core phase.
//! - The lifecycle composes with the existing RAHU
//!    and produces a .

use std::any::Any;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::plan::ExecutionPlan;

/// Confidence score from KETU validation, in 0.0..=1.0.
///
/// A score of 1.0 means KETU verified the Core output
/// without ambiguity. A score of 0.0 means the Core output
/// failed validation entirely. Intermediate values
/// indicate partial verification.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Confidence(f32);

impl Confidence {
    pub const MAX: Confidence = Confidence(1.0);
    pub const MIN: Confidence = Confidence(0.0);
    pub const UNVERIFIED: Confidence = Confidence(0.0);
    pub const FULL: Confidence = Confidence(1.0);

    pub fn new(value: f32) -> Self {
        Confidence(value.clamp(0.0, 1.0))
    }

    pub fn value(self) -> f32 {
        self.0
    }

    /// True if KETU has enough confidence to mark the
    /// Core output as accepted.
    pub fn is_acceptable(self) -> bool {
        self.0 >= 0.5
    }
}

impl Default for Confidence {
    fn default() -> Self {
        Confidence::UNVERIFIED
    }
}

impl fmt::Display for Confidence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// The status of a KETU validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KetuStatus {
    /// KETU verified the Core output. The result is
    /// acceptable.
    Accepted,
    /// KETU verified partially. The result may be used
    /// with caution.
    AcceptedWithCaveats,
    /// KETU could not verify. The result should be
    /// retried or escalated.
    Rejected,
    /// KETU did not run. Placeholder status for the
    /// initial integration.
    Pending,
}

impl KetuStatus {
    pub fn is_accepted(self) -> bool {
        matches!(self, KetuStatus::Accepted | KetuStatus::AcceptedWithCaveats)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            KetuStatus::Accepted => "ACCEPTED",
            KetuStatus::AcceptedWithCaveats => "ACCEPTED_WITH_CAVEATS",
            KetuStatus::Rejected => "REJECTED",
            KetuStatus::Pending => "PENDING",
        }
    }
}

/// The KETU validation record. KETU emits this after
/// validating a Core phase result.
///
/// Initial integration uses placeholder values. The
/// field set is the contract; the implementation comes
/// in subsequent milestones.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KetuValidation {
    pub status: KetuStatus,
    pub confidence: Confidence,
    pub integrity_score: f32,
    pub replay_validated: bool,
    pub benchmark_verified: bool,
    pub constitutional_check: bool,
    pub governance_confidence: Confidence,
    pub capability_validated: bool,
    pub notes: Vec<String>,
}

impl KetuValidation {
    /// Construct a placeholder KETU validation. Real
    /// KETU implementations will replace this with
    /// actual validation logic.
    pub fn placeholder() -> Self {
        KetuValidation {
            status: KetuStatus::Pending,
            confidence: Confidence::UNVERIFIED,
            integrity_score: 0.0,
            replay_validated: false,
            benchmark_verified: false,
            constitutional_check: false,
            governance_confidence: Confidence::UNVERIFIED,
            capability_validated: false,
            notes: vec!["placeholder validation".to_string()],
        }
    }

    pub fn accepted(confidence: Confidence) -> Self {
        KetuValidation {
            status: KetuStatus::Accepted,
            confidence,
            integrity_score: confidence.value(),
            replay_validated: true,
            benchmark_verified: true,
            constitutional_check: true,
            governance_confidence: confidence,
            capability_validated: true,
            notes: vec!["validation passed".to_string()],
        }
    }

    pub fn rejected(reason: impl Into<String>) -> Self {
        KetuValidation {
            status: KetuStatus::Rejected,
            confidence: Confidence::MIN,
            integrity_score: 0.0,
            replay_validated: false,
            benchmark_verified: false,
            constitutional_check: false,
            governance_confidence: Confidence::MIN,
            capability_validated: false,
            notes: vec![reason.into()],
        }
    }
}

/// The RAHU phase result. This is a thin wrapper around
/// the existing . The lifecycle does not
/// reimplement planning — it composes with the existing
/// RAHU .
///
///  carries the plan that the Core phase
/// will execute and that KETU will validate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RahuPhase {
    pub plan: ExecutionPlan,
}

impl RahuPhase {
    pub fn from_plan(plan: ExecutionPlan) -> Self {
        RahuPhase { plan }
    }

    pub fn request_id(&self) -> &str {
        &self.plan.request_id
    }
}

/// The Core Domain phase. The Core phase is what the
/// Meta Harness actually does: execute code, retrieve
/// memory, plan decisions, manage identity, evolve
/// genes. The lifecycle does not dictate what the
/// Core phase is; each Meta Harness supplies its own.
///
/// The Core phase carries a
/// so it can hold any domain object.
#[derive(Debug)]
pub struct CorePhase {
    pub domain: Box<dyn Any + Send + Sync>,
    pub description: String,
}

impl CorePhase {
    pub fn new<D: Any + Send + Sync>(domain: D, description: impl Into<String>) -> Self {
        CorePhase {
            domain: Box::new(domain),
            description: description.into(),
        }
    }

    /// Try to downcast the Core phase to a concrete type.
    /// Returns  if the type does not match.
    pub fn downcast_ref<D: Any>(&self) -> Option<&D> {
        self.domain.downcast_ref::<D>()
    }
}

/// The KETU phase result. KETU produces a
/// after validating the Core phase output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KetuPhase {
    pub validation: KetuValidation,
}

impl KetuPhase {
    pub fn from_validation(validation: KetuValidation) -> Self {
        KetuPhase { validation }
    }

    pub fn placeholder() -> Self {
        KetuPhase {
            validation: KetuValidation::placeholder(),
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.validation.status.is_accepted()
    }
}

/// The full lifecycle outcome. This is what the
/// runtime emits after a Meta Harness completes a
/// full RAHU -> Core -> KETU cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleOutcome {
    pub rahu: RahuPhase,
    pub core: CorePhaseSnapshot,
    pub ketu: KetuPhase,
}

/// A serializable snapshot of the Core phase. The Core
/// phase itself holds an  value that is not
/// serializable; the snapshot captures the description
/// and a serialized representation of the domain
/// object (if any).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorePhaseSnapshot {
    pub description: String,
    pub domain_repr: Option<String>,
}

impl CorePhaseSnapshot {
    pub fn from_core(core: &CorePhase) -> Self {
        CorePhaseSnapshot {
            description: core.description.clone(),
            domain_repr: None,
        }
    }
}

impl LifecycleOutcome {
    pub fn new(rahu: RahuPhase, core: CorePhase, ketu: KetuPhase) -> Self {
        LifecycleOutcome {
            core: CorePhaseSnapshot::from_core(&core),
            rahu,
            ketu,
        }
    }

    pub fn is_accepted(&self) -> bool {
        self.ketu.is_accepted()
    }

    pub fn request_id(&self) -> &str {
        self.rahu.request_id()
    }

    pub fn rahu(&self) -> &RahuPhase {
        &self.rahu
    }

    pub fn core(&self) -> &CorePhaseSnapshot {
        &self.core
    }

    pub fn ketu(&self) -> &KetuPhase {
        &self.ketu
    }
}

/// The lifecycle for a Meta Harness. A lifecycle
/// composes the three phases. The  method runs
/// the full cycle and returns a .
///
/// The lifecycle is generic so any Source Harness can
/// supply its own Core phase. The default
/// implementation produces a placeholder validation;
/// subsequent milestones will replace it with real
/// validation logic.
pub struct MetaHarnessLifecycle {
    /// The RahuPhase is supplied at construction time
    /// from the existing RAHU pipeline.
    rahu: RahuPhase,
    /// The Core phase is supplied by the Meta Harness.
    /// The lifecycle runs it and captures the result.
    core: CorePhase,
    /// The KETU phase is produced by the validation step.
    /// The default implementation produces a placeholder.
    ketu: KetuPhase,
}

impl MetaHarnessLifecycle {
    /// Construct a new lifecycle from a RAHU plan and a
    /// Core phase. KETU is initialized to a placeholder.
    pub fn new(plan: ExecutionPlan, core: CorePhase) -> Self {
        MetaHarnessLifecycle {
            rahu: RahuPhase::from_plan(plan),
            core,
            ketu: KetuPhase::placeholder(),
        }
    }

    /// Run the KETU validation phase. The default
    /// implementation produces a placeholder. Concrete
    /// implementations override this.
    pub fn validate(&mut self) -> &KetuPhase {
        // Placeholder: real KETU logic is implemented
        // in subsequent milestones.
        &self.rahu; // suppress unused warning
        &self.ketu
    }

    /// Run the full RAHU -> Core -> KETU cycle. Returns
    /// the lifecycle outcome.
    pub fn run(mut self) -> LifecycleOutcome {
        let _ = self.validate();
        LifecycleOutcome::new(self.rahu, self.core, self.ketu)
    }

    /// Access the RAHU phase (immutable).
    pub fn rahu(&self) -> &RahuPhase {
        &self.rahu
    }

    /// Access the Core phase (immutable).
    pub fn core(&self) -> &CorePhase {
        &self.core
    }

    /// Access the KETU phase (immutable).
    pub fn ketu(&self) -> &KetuPhase {
        &self.ketu
    }
}

/// Convenience function: build a full lifecycle from a
/// RAHU plan and a Core domain object, then run it and
/// return the outcome. This is the high-level entry
/// point the runtime uses.
pub fn run_lifecycle<D: Any + Send + Sync>(
    plan: ExecutionPlan,
    domain: D,
    core_description: impl Into<String>,
) -> LifecycleOutcome {
    let core = CorePhase::new(domain, core_description);
    MetaHarnessLifecycle::new(plan, core).run()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilityLeaseRequest;
    use crate::harness::GeneKind;
    use crate::harness::SourceHarnessKind;
    use crate::plan::ExecutionMode;
    use crate::selection::{GeneSelection, MetaHarnessSelection, SourceHarnessSelection};

    fn fixture_plan() -> ExecutionPlan {
        use crate::plan::ExecutionRoute;
        let route = ExecutionRoute {
            mode: ExecutionMode::Chain,
            source: SourceHarnessSelection::new(SourceHarnessKind::Phoenix, "phoenix"),
            meta: MetaHarnessSelection::new(SourceHarnessKind::Phoenix, "phoenix-shell"),
            gene: GeneSelection::new(
                SourceHarnessKind::Phoenix,
                GeneKind::Execution,
                "exec-default",
            ),
            lease: CapabilityLeaseRequest::new("lease-1", vec![], 60_000),
        };
        ExecutionPlan::new("req-1", route)
    }

    #[test]
    fn confidence_clamps_and_acceptable() {
        assert_eq!(Confidence::new(2.0).value(), 1.0);
        assert_eq!(Confidence::new(-1.0).value(), 0.0);
        assert!(Confidence::new(0.6).is_acceptable());
        assert!(!Confidence::new(0.4).is_acceptable());
    }

    #[test]
    fn ketu_status_strings() {
        assert_eq!(KetuStatus::Accepted.as_str(), "ACCEPTED");
        assert_eq!(KetuStatus::Pending.as_str(), "PENDING");
        assert!(KetuStatus::Accepted.is_accepted());
        assert!(!KetuStatus::Rejected.is_accepted());
    }

    #[test]
    fn ketu_validation_placeholder_is_pending() {
        let v = KetuValidation::placeholder();
        assert_eq!(v.status, KetuStatus::Pending);
        assert_eq!(v.confidence, Confidence::UNVERIFIED);
        assert!(!v.status.is_accepted());
    }

    #[test]
    fn ketu_validation_accepted_is_acceptable() {
        let v = KetuValidation::accepted(Confidence::new(0.95));
        assert_eq!(v.status, KetuStatus::Accepted);
        assert!(v.status.is_accepted());
    }

    #[test]
    fn core_phase_downcast() {
        let core: CorePhase = CorePhase::new(42u32, "answer");
        assert_eq!(core.downcast_ref::<u32>(), Some(&42));
        assert_eq!(core.downcast_ref::<String>(), None);
    }

    #[test]
    fn rahu_phase_carries_plan() {
        let plan = fixture_plan();
        let p = RahuPhase::from_plan(plan.clone());
        assert_eq!(p.request_id(), "req-1");
        assert_eq!(p.plan, plan);
    }

    #[test]
    fn lifecycle_run_produces_outcome() {
        let plan = fixture_plan();
        let outcome = run_lifecycle(plan, "result-data", "did the thing");
        assert_eq!(outcome.rahu().plan.request_id, "req-1");
        assert_eq!(outcome.core.description, "did the thing");
        assert!(!outcome.is_accepted()); // placeholder KETU
    }
}
