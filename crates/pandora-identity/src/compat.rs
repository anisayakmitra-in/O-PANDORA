//! Constitutional compatibility layer.
//!
//! The constitutional types in
//! are the canonical implementations of Manifest, Version,
//! Health, Telemetry, Trust, Provenance, and Lifecycle for
//! every constitutional object in Pandora. The
//!  crate predates that consolidation and
//! carries its own (more detailed) representations.
//!
//! This module does NOT remove or fork the existing
//!  types. It provides  conversions
//! so the identity-system types compose the constitutional
//! types. Existing API is preserved.

use pandora_types::constitutional::{
    ConstitutionalManifest, ManifestHealth, ManifestKind, ManifestLifecycleState,
    ManifestProvenance, ManifestSignature, ManifestTelemetry, ManifestTrust, ManifestVersion,
    TrustLevel,
};

use crate::kind::IdentityKind;
use crate::manifest::{
    IdentityHealth, IdentityLifecycleStage, IdentityManifest, IdentityProvenance,
    IdentityTelemetry, IdentityTrust,
};
use crate::version::IdentityVersion;

impl From<&IdentityVersion> for ManifestVersion {
    fn from(v: &IdentityVersion) -> Self {
        ManifestVersion::new(v.major, v.minor, v.patch)
    }
}

impl From<&IdentityKind> for ManifestKind {
    fn from(k: &IdentityKind) -> Self {
        match k {
            IdentityKind::SourceHarness => ManifestKind::SourceHarness,
            IdentityKind::MetaHarness => ManifestKind::MetaHarness,
            IdentityKind::Gene => ManifestKind::Gene,
            IdentityKind::Loop => ManifestKind::Loop,
            IdentityKind::Provider => ManifestKind::Provider,
            IdentityKind::Tool => ManifestKind::Tool,
            IdentityKind::Capability => ManifestKind::Capability,
            IdentityKind::SandboxBackend => ManifestKind::SandboxBackend,
            IdentityKind::MemoryBackend => ManifestKind::MemoryBackend,
            IdentityKind::ExecutionSession => ManifestKind::Custom("ExecutionSession".to_string()),
            IdentityKind::EngineeringSession => {
                ManifestKind::Custom("EngineeringSession".to_string())
            }
            IdentityKind::Workflow => ManifestKind::Workflow,
            IdentityKind::Agent => ManifestKind::Agent,
            IdentityKind::Plugin => ManifestKind::Plugin,
            IdentityKind::Mcp => ManifestKind::Mcp,
            IdentityKind::Package => ManifestKind::Package,
            IdentityKind::MarketplaceAsset => ManifestKind::Custom("MarketplaceAsset".to_string()),
        }
    }
}

impl From<&IdentityHealth> for ManifestHealth {
    fn from(h: &IdentityHealth) -> Self {
        let status = match h {
            IdentityHealth::Healthy => "healthy",
            IdentityHealth::Degraded => "degraded",
            IdentityHealth::Unhealthy => "unhealthy",
            IdentityHealth::Unknown => "unknown",
        };
        ManifestHealth {
            status: status.to_string(),
        }
    }
}

impl From<&IdentityTrust> for ManifestTrust {
    fn from(t: &IdentityTrust) -> Self {
        let level = if t.verified_by.is_empty() {
            TrustLevel::Unknown
        } else {
            TrustLevel::Community
        };
        let mut trust = ManifestTrust::new(level);
        if !t.notes.is_empty() {
            trust = trust.verified();
        }
        trust
    }
}

impl From<&IdentityTelemetry> for ManifestTelemetry {
    fn from(t: &IdentityTelemetry) -> Self {
        let mut out = ManifestTelemetry::default();
        for (i, kind) in t.emitted_kinds.iter().enumerate() {
            out.counters.insert(format!("emitted.{}", kind), i as u64);
        }
        out
    }
}

impl From<&IdentityProvenance> for ManifestProvenance {
    fn from(p: &IdentityProvenance) -> Self {
        let source = p
            .source_repository
            .clone()
            .or(p.declared_by.clone())
            .unwrap_or_else(|| "unknown".to_string());
        ManifestProvenance::from_source(source)
    }
}

impl From<&IdentityLifecycleStage> for ManifestLifecycleState {
    fn from(s: &IdentityLifecycleStage) -> Self {
        match s {
            IdentityLifecycleStage::Declared => ManifestLifecycleState::Registered,
            IdentityLifecycleStage::Installing => ManifestLifecycleState::Booting,
            IdentityLifecycleStage::Installed => ManifestLifecycleState::Ready,
            IdentityLifecycleStage::Upgrading => ManifestLifecycleState::Booting,
            IdentityLifecycleStage::Uninstalling => ManifestLifecycleState::ShuttingDown,
            IdentityLifecycleStage::Uninstalled => ManifestLifecycleState::Stopped,
        }
    }
}

impl From<&IdentityManifest> for ConstitutionalManifest {
    fn from(m: &IdentityManifest) -> Self {
        let kind: ManifestKind = (&m.kind).into();
        let version: ManifestVersion = (&m.version).into();
        let mut cm =
            ConstitutionalManifest::new(m.name.clone(), kind, version, m.description.clone());
        if let Some(sig) = &m.signature {
            cm.signature = Some(ManifestSignature {
                algorithm: sig.algorithm.clone(),
                signature: sig.value.clone(),
                key_id: sig.key_id.clone().unwrap_or_default(),
            });
        }
        cm
    }
}

// --- Legacy Identity -> Universal compat shims ---

impl From<&IdentityHealth> for pandora_types::universal::Health {
    fn from(h: &IdentityHealth) -> Self {
        match h {
            IdentityHealth::Healthy => pandora_types::universal::Health::Healthy,
            IdentityHealth::Degraded => pandora_types::universal::Health::Degraded,
            IdentityHealth::Unhealthy => pandora_types::universal::Health::Offline,
            IdentityHealth::Unknown => pandora_types::universal::Health::Ready,
        }
    }
}

impl From<&IdentityLifecycleStage> for pandora_types::universal::Lifecycle {
    fn from(l: &IdentityLifecycleStage) -> Self {
        match l {
            IdentityLifecycleStage::Declared => pandora_types::universal::Lifecycle::Created,
            IdentityLifecycleStage::Installing => pandora_types::universal::Lifecycle::Installed,
            IdentityLifecycleStage::Installed => pandora_types::universal::Lifecycle::Ready,
            IdentityLifecycleStage::Upgrading => pandora_types::universal::Lifecycle::Updating,
            IdentityLifecycleStage::Uninstalling => pandora_types::universal::Lifecycle::Stopping,
            IdentityLifecycleStage::Uninstalled => pandora_types::universal::Lifecycle::Stopped,
        }
    }
}

impl From<&IdentityTelemetry> for pandora_types::universal::Telemetry {
    fn from(t: &IdentityTelemetry) -> Self {
        let mut metrics = std::collections::BTreeMap::new();
        metrics.insert(
            "emitted_kinds_count".to_string(),
            t.emitted_kinds.len() as u64,
        );
        pandora_types::universal::Telemetry {
            metrics,
            events: vec![],
            timestamps: pandora_types::universal::TelemetryTimestamps::default(),
            diagnostics: t.emitted_kinds.clone(),
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_conversion() {
        let v = IdentityVersion::new(1, 2, 3);
        let m: ManifestVersion = (&v).into();
        assert_eq!(m.major, 1);
        assert_eq!(m.minor, 2);
        assert_eq!(m.patch, 3);
    }

    #[test]
    fn kind_conversion_source_harness() {
        let k = IdentityKind::SourceHarness;
        let mk: ManifestKind = (&k).into();
        assert_eq!(mk, ManifestKind::SourceHarness);
    }

    #[test]
    fn kind_conversion_marketplace() {
        let k = IdentityKind::MarketplaceAsset;
        let mk: ManifestKind = (&k).into();
        assert_eq!(mk, ManifestKind::Custom("MarketplaceAsset".to_string()));
    }

    #[test]
    fn kind_conversion_execution_session() {
        let k = IdentityKind::ExecutionSession;
        let mk: ManifestKind = (&k).into();
        assert_eq!(mk, ManifestKind::Custom("ExecutionSession".to_string()));
    }

    #[test]
    fn health_conversion_healthy() {
        let h = IdentityHealth::Healthy;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "healthy");
    }

    #[test]
    fn health_conversion_unknown() {
        let h = IdentityHealth::Unknown;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "unknown");
    }

    #[test]
    fn lifecycle_conversion_active() {
        let s = IdentityLifecycleStage::Installed;
        let ms: ManifestLifecycleState = (&s).into();
        assert_eq!(ms, ManifestLifecycleState::Ready);
    }

    #[test]
    fn lifecycle_conversion_failed() {
        let s = IdentityLifecycleStage::Uninstalled;
        let ms: ManifestLifecycleState = (&s).into();
        assert_eq!(ms, ManifestLifecycleState::Stopped);
    }

    #[test]
    fn telemetry_conversion_empty() {
        let t = IdentityTelemetry::new();
        let mt: ManifestTelemetry = (&t).into();
        assert!(mt.counters.is_empty());
    }

    #[test]
    fn telemetry_conversion_with_kinds() {
        let t = IdentityTelemetry::new().emits("loop").emits("tick");
        let mt: ManifestTelemetry = (&t).into();
        assert!(mt.counters.contains_key("emitted.loop"));
        assert!(mt.counters.contains_key("emitted.tick"));
    }

    #[test]
    fn trust_conversion_empty_verifiers() {
        let t = IdentityTrust::new();
        let mt: ManifestTrust = (&t).into();
        assert_eq!(mt.level, TrustLevel::Unknown);
    }

    #[test]
    fn trust_conversion_with_verifiers() {
        let mut t = IdentityTrust::new();
        t.verified_by.push("council".to_string());
        let mt: ManifestTrust = (&t).into();
        assert_eq!(mt.level, TrustLevel::Community);
    }

    #[test]
    fn provenance_conversion_with_repo() {
        let p = IdentityProvenance {
            source_repository: Some("github.com/x/y".to_string()),
            ..Default::default()
        };
        let mp: ManifestProvenance = (&p).into();
        assert_eq!(mp.source, "github.com/x/y");
    }

    #[test]
    fn manifest_conversion_compiles() {
        let k = IdentityKind::SourceHarness;
        let im = IdentityManifest::new("id-1", "phoenix", k, "Arka");
        let cm: ConstitutionalManifest = (&im).into();
        assert_eq!(cm.identity.name, "phoenix");
        assert_eq!(cm.identity.kind, ManifestKind::SourceHarness);
    }
}
