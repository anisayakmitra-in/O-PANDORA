//! Constitutional compatibility layer for RAHU.
//!
//! The runtime layer of RAHU carries its own VersionSpec,
//! HealthStatus, and TelemetryReport. The constitutional
//! types in pandora-types are the canonical source of
//! truth. This module provides From conversions from
//! the runtime types to the constitutional types.
//!
//! Existing RAHU runtime types are NOT modified. New code
//! can use the From conversions to migrate to the
//! constitutional types.

use pandora_types::constitutional::{ManifestHealth, ManifestTelemetry, ManifestVersion};

use crate::runtime::{HealthStatus, LifecycleState, TelemetryReport, VersionSpec};

impl From<&VersionSpec> for ManifestVersion {
    fn from(v: &VersionSpec) -> Self {
        // The runtime VersionSpec carries a
        //  field which the constitutional
        // ManifestVersion does not have. The compat_minor
        // is lost in this conversion; callers that need
        // the compat range should use the runtime
        // VersionSpec directly.
        ManifestVersion::new(v.major, v.minor, v.patch)
    }
}

impl From<&HealthStatus> for ManifestHealth {
    fn from(h: &HealthStatus) -> Self {
        let status = match h {
            HealthStatus::Healthy => "healthy",
            HealthStatus::Degraded => "degraded",
            HealthStatus::Unhealthy => "unhealthy",
            HealthStatus::Unknown => "unknown",
        };
        ManifestHealth {
            status: status.to_string(),
        }
    }
}

impl From<&TelemetryReport> for ManifestTelemetry {
    fn from(t: &TelemetryReport) -> Self {
        // The runtime TelemetryReport carries
        //  and . The
        // constitutional ManifestTelemetry carries only
        // . The activity and uptime fields are
        // preserved as counters so callers can still
        // access them.
        let mut counters = t.counters.clone();
        counters.insert("__last_activity_ms".to_string(), t.last_activity_ms);
        counters.insert("__uptime_ms".to_string(), t.uptime_ms);

        ManifestTelemetry { counters }
    }
}

// --- Legacy -> Universal compat shims ---

impl From<HealthStatus> for pandora_types::universal::Health {
    fn from(h: HealthStatus) -> Self {
        match h {
            HealthStatus::Healthy => pandora_types::universal::Health::Healthy,
            HealthStatus::Degraded => pandora_types::universal::Health::Degraded,
            HealthStatus::Unhealthy => pandora_types::universal::Health::Offline,
            HealthStatus::Unknown => pandora_types::universal::Health::Ready,
        }
    }
}

impl From<LifecycleState> for pandora_types::universal::Lifecycle {
    fn from(l: LifecycleState) -> Self {
        match l {
            LifecycleState::Registered => pandora_types::universal::Lifecycle::Created,
            LifecycleState::Booting => pandora_types::universal::Lifecycle::Installed,
            LifecycleState::Ready => pandora_types::universal::Lifecycle::Ready,
            LifecycleState::ShuttingDown => pandora_types::universal::Lifecycle::Stopping,
            LifecycleState::Stopped => pandora_types::universal::Lifecycle::Stopped,
            LifecycleState::Failed => pandora_types::universal::Lifecycle::Recovering,
        }
    }
}

impl From<&TelemetryReport> for pandora_types::universal::Telemetry {
    fn from(t: &TelemetryReport) -> Self {
        let timestamps = pandora_types::universal::TelemetryTimestamps {
            last_activity_ms: t.last_activity_ms,
            ..Default::default()
        };
        let mut metrics = t.counters.clone();
        metrics.insert("uptime_ms".to_string(), t.uptime_ms);
        pandora_types::universal::Telemetry {
            metrics,
            events: vec![],
            timestamps,
            diagnostics: vec![],
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::LifecycleState;
    use pandora_types::constitutional::ConstitutionalManifest;

    #[test]
    fn version_spec_to_manifest_version() {
        let v = VersionSpec::new(2, 3, 4).with_compat_minor(1);
        let m: ManifestVersion = (&v).into();
        assert_eq!(m.major, 2);
        assert_eq!(m.minor, 3);
        assert_eq!(m.patch, 4);
    }

    #[test]
    fn health_status_healthy() {
        let h = HealthStatus::Healthy;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "healthy");
    }

    #[test]
    fn health_status_degraded() {
        let h = HealthStatus::Degraded;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "degraded");
    }

    #[test]
    fn health_status_unhealthy() {
        let h = HealthStatus::Unhealthy;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "unhealthy");
    }

    #[test]
    fn health_status_unknown() {
        let h = HealthStatus::Unknown;
        let mh: ManifestHealth = (&h).into();
        assert_eq!(mh.status, "unknown");
    }

    #[test]
    fn telemetry_report_counters_preserved() {
        let t = TelemetryReport::empty()
            .with_counter("requests", 5)
            .with_counter("errors", 2);
        let mt: ManifestTelemetry = (&t).into();
        assert_eq!(mt.counters.get("requests"), Some(&5));
        assert_eq!(mt.counters.get("errors"), Some(&2));
    }

    #[test]
    fn telemetry_report_activity_preserved() {
        let t = TelemetryReport::empty()
            .with_activity(1000)
            .with_uptime(5000);
        let mt: ManifestTelemetry = (&t).into();
        assert_eq!(mt.counters.get("__last_activity_ms"), Some(&1000));
        assert_eq!(mt.counters.get("__uptime_ms"), Some(&5000));
    }

    #[test]
    fn lifecycle_state_serializes() {
        let s = LifecycleState::Ready;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"Ready\"");
    }

    #[test]
    fn source_harness_manifest_serde() {
        use pandora_types::constitutional::{ManifestKind, ManifestVersion};
        let m = ConstitutionalManifest::new(
            "phoenix",
            ManifestKind::SourceHarness,
            ManifestVersion::new(1, 0, 0),
            "test",
        );
        let json = serde_json::to_string(&m).unwrap();
        assert!(json.contains("phoenix"));
        assert!(json.contains("major")); // ManifestVersion serializes as {major,minor,patch}
    }
}
