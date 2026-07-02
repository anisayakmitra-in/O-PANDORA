//! Concrete Phoenix Source Harness.
//!
//! Phoenix owns execution. It is the canonical
//! Execution Source Harness.

use crate::harness::{SourceHarness, SourceHarnessKind};
use crate::runtime::{HealthStatus, LifecycleState, TelemetryReport};
use pandora_types::constitutional::{ConstitutionalManifest, ManifestKind, ManifestVersion};

/// The canonical Phoenix Source Harness.
pub struct PhoenixHarness {
    manifest: ConstitutionalManifest,
}

impl PhoenixHarness {
    pub fn new() -> Self {
        PhoenixHarness {
            manifest: ConstitutionalManifest::new(
                "phoenix",
                ManifestKind::SourceHarness,
                ManifestVersion::new(1, 0, 0),
                "Pandora Execution Source Harness",
            ),
        }
    }

    pub fn version() -> &'static str {
        "1.0.0"
    }
    pub fn canonical_name() -> &'static str {
        "phoenix"
    }
    pub fn health(&self) -> HealthStatus {
        HealthStatus::Healthy
    }
    pub fn lifecycle(&self) -> LifecycleState {
        LifecycleState::Ready
    }

    pub fn capabilities(&self) -> Vec<&'static str> {
        vec![
            "execution",
            "sandbox",
            "checkpoint",
            "rollback",
            "self-healing",
            "telemetry",
            "branch-execution",
            "execution-lineage",
        ]
    }

    pub fn dependencies(&self) -> Vec<SourceHarnessKind> {
        vec![SourceHarnessKind::Anubis, SourceHarnessKind::Provider]
    }

    pub fn telemetry(&self) -> TelemetryReport {
        TelemetryReport::empty()
    }

    // --- Constitutional facets ---

    pub fn health_universal(&self) -> pandora_types::universal::Health {
        pandora_types::universal::Health::Healthy
    }

    pub fn lifecycle_universal(&self) -> pandora_types::universal::Lifecycle {
        pandora_types::universal::Lifecycle::Ready
    }

    pub fn execution_profile(&self) -> pandora_types::universal::ExecutionProfile {
        pandora_types::universal::ExecutionProfile::Stateless
    }

    pub fn evolution_config(&self) -> pandora_types::universal::EvolutionConfig {
        pandora_types::universal::EvolutionConfig::enabled()
    }

    pub fn debug_pipeline(&self) -> Vec<pandora_types::universal::DebugPhase> {
        vec![
            pandora_types::universal::DebugPhase::Trace,
            pandora_types::universal::DebugPhase::Diagnostics,
            pandora_types::universal::DebugPhase::Replay,
            pandora_types::universal::DebugPhase::Repair,
            pandora_types::universal::DebugPhase::Benchmark,
            pandora_types::universal::DebugPhase::Evolution,
            pandora_types::universal::DebugPhase::Publish,
        ]
    }

    pub fn pandora_score(&self) -> pandora_types::universal::PandoraScore {
        pandora_types::universal::PandoraScore::official()
    }

    pub fn owned_engines(&self) -> Vec<&'static str> {
        vec![
            "ExecutionArchaeologyEngine",
            "ExecutionRankingEngine",
            "ExecutionSurvivabilityEngine",
            "ExecutionStateSynthesisEngine",
            "ConstitutionalExecutionLicenseEngine",
        ]
    }

    pub fn owned_meta_harnesses(&self) -> Vec<&'static str> {
        vec![
            "Execution",
            "Sandbox",
            "Recovery",
            "Capability",
            "Telemetry",
        ]
    }
}

impl Default for PhoenixHarness {
    fn default() -> Self {
        PhoenixHarness::new()
    }
}

impl SourceHarness for PhoenixHarness {
    fn kind(&self) -> SourceHarnessKind {
        SourceHarnessKind::Phoenix
    }
    fn manifest(&self) -> &ConstitutionalManifest {
        &self.manifest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn phoenix_manifest() {
        let h = PhoenixHarness::new();
        assert_eq!(h.manifest().identity.name, "phoenix");
        assert_eq!(h.kind(), SourceHarnessKind::Phoenix);
    }
    #[test]
    fn phoenix_health() {
        assert!(PhoenixHarness::new().health().is_dispatchable());
    }
    #[test]
    fn phoenix_universal_health() {
        assert_eq!(
            PhoenixHarness::new().health_universal(),
            pandora_types::universal::Health::Healthy
        );
    }
    #[test]
    fn phoenix_universal_lifecycle() {
        assert_eq!(
            PhoenixHarness::new().lifecycle_universal(),
            pandora_types::universal::Lifecycle::Ready
        );
    }
    #[test]
    fn phoenix_evolution_config() {
        let c = PhoenixHarness::new().evolution_config();
        assert!(c.gepa_enabled);
        assert!(c.dsr_enabled);
    }
    #[test]
    fn phoenix_pandora_score() {
        assert!(PhoenixHarness::new().pandora_score().official_status);
    }
    #[test]
    fn phoenix_owned_engines() {
        assert!(!PhoenixHarness::new().owned_engines().is_empty());
    }
}
