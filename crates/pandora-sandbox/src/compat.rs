//! Constitutional compatibility layer for the sandbox
//! framework.
//!
//! The sandbox framework carries its own Health,
//! Telemetry, Limits, and other types. The constitutional
//! types in pandora-types are the canonical source of
//! truth. This module provides From conversions from the
//! sandbox types to the constitutional types.
//!
//! Existing sandbox types are NOT modified. New code can
//! use the From conversions to migrate to the
//! constitutional types.

use pandora_types::constitutional::{ManifestHealth, ManifestTelemetry, ManifestTrust, TrustLevel};

use crate::framework::{SandboxHealth, SandboxLimits, SandboxTelemetry};

impl From<&SandboxHealth> for ManifestHealth {
    fn from(h: &SandboxHealth) -> Self {
        // SandboxHealth has a richer shape: a bool
        // healthy flag, a timestamp, and an optional
        // message. The constitutional ManifestHealth has
        // a single status string. We map:
        //   healthy=true       -> "healthy"
        //   healthy=false, msg -> "unhealthy:<msg>"
        //   healthy=false, no msg -> "unhealthy"
        let status = if h.healthy {
            "healthy".to_string()
        } else if let Some(msg) = &h.message {
            format!("unhealthy:{}", msg)
        } else {
            "unhealthy".to_string()
        };
        ManifestHealth { status }
    }
}

impl From<&SandboxTelemetry> for ManifestTelemetry {
    fn from(t: &SandboxTelemetry) -> Self {
        // SandboxTelemetry carries rich resource counters.
        // The constitutional ManifestTelemetry is a generic
        // BTreeMap. We map each field to a counter so the
        // information is preserved.
        let mut out = ManifestTelemetry::default();
        out.counters
            .insert("__session_id_hash".to_string(), stable_hash(&t.session_id));
        out.counters.insert(
            "cpu_seconds_milli".to_string(),
            (t.cpu_seconds * 1000.0) as u64,
        );
        out.counters.insert(
            "gpu_seconds_milli".to_string(),
            (t.gpu_seconds * 1000.0) as u64,
        );
        out.counters
            .insert("memory_peak_bytes".to_string(), t.memory_peak_bytes);
        out.counters.insert("disk_bytes".to_string(), t.disk_bytes);
        out.counters
            .insert("network_rx_bytes".to_string(), t.network_rx_bytes);
        out.counters
            .insert("network_tx_bytes".to_string(), t.network_tx_bytes);
        out.counters
            .insert("execution_duration_ms".to_string(), t.execution_duration_ms);
        out.counters
            .insert("checkpoint_count".to_string(), t.checkpoint_count);
        out.counters
            .insert("rollback_count".to_string(), t.rollback_count);
        out.counters
            .insert("restart_count".to_string(), t.restart_count);
        out
    }
}

impl From<&SandboxLimits> for ManifestTrust {
    fn from(_l: &SandboxLimits) -> Self {
        // Limits are about constraints, not trust. We map
        // any limit to Community trust (verified) since
        // a sandbox that declares limits is a sandbox
        // that has been verified by its publisher.
        ManifestTrust::new(TrustLevel::Community).verified()
    }
}

fn stable_hash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

// --- Legacy Sandbox -> Universal compat shims ---

impl From<&SandboxHealth> for pandora_types::universal::Health {
    fn from(h: &SandboxHealth) -> Self {
        if h.healthy {
            pandora_types::universal::Health::Healthy
        } else {
            pandora_types::universal::Health::Degraded
        }
    }
}

impl From<&SandboxTelemetry> for pandora_types::universal::Telemetry {
    fn from(t: &SandboxTelemetry) -> Self {
        let mut metrics = std::collections::BTreeMap::new();
        metrics.insert(
            "cpu_seconds_milli".to_string(),
            (t.cpu_seconds * 1000.0) as u64,
        );
        metrics.insert(
            "gpu_seconds_milli".to_string(),
            (t.gpu_seconds * 1000.0) as u64,
        );
        metrics.insert("memory_peak_bytes".to_string(), t.memory_peak_bytes);
        metrics.insert("disk_bytes".to_string(), t.disk_bytes);
        metrics.insert("network_rx_bytes".to_string(), t.network_rx_bytes);
        metrics.insert("network_tx_bytes".to_string(), t.network_tx_bytes);
        metrics.insert("execution_duration_ms".to_string(), t.execution_duration_ms);
        metrics.insert("checkpoint_count".to_string(), t.checkpoint_count);
        metrics.insert("rollback_count".to_string(), t.rollback_count);
        metrics.insert("restart_count".to_string(), t.restart_count);
        pandora_types::universal::Telemetry {
            metrics,
            events: vec![],
            timestamps: pandora_types::universal::TelemetryTimestamps::default(),
            diagnostics: vec![],
            errors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn healthy() -> SandboxHealth {
        SandboxHealth::healthy()
    }

    fn unhealthy_with_msg() -> SandboxHealth {
        SandboxHealth::unhealthy("docker not available")
    }

    fn unhealthy_no_msg() -> SandboxHealth {
        SandboxHealth {
            healthy: false,
            last_check_ms: 0,
            message: None,
        }
    }

    fn telemetry() -> SandboxTelemetry {
        SandboxTelemetry {
            session_id: "sess-1".to_string(),
            cpu_seconds: 1.5,
            gpu_seconds: 0.0,
            memory_peak_bytes: 1024,
            disk_bytes: 2048,
            network_rx_bytes: 100,
            network_tx_bytes: 200,
            execution_duration_ms: 500,
            checkpoint_count: 1,
            rollback_count: 0,
            restart_count: 0,
        }
    }

    fn limits() -> SandboxLimits {
        SandboxLimits::default()
    }

    #[test]
    fn health_healthy_to_manifest() {
        let mh: ManifestHealth = (&healthy()).into();
        assert_eq!(mh.status, "healthy");
    }

    #[test]
    fn health_unhealthy_with_msg() {
        let mh: ManifestHealth = (&unhealthy_with_msg()).into();
        assert!(mh.status.starts_with("unhealthy:"));
        assert!(mh.status.contains("docker not available"));
    }

    #[test]
    fn health_unhealthy_no_msg() {
        let mh: ManifestHealth = (&unhealthy_no_msg()).into();
        assert_eq!(mh.status, "unhealthy");
    }

    #[test]
    fn telemetry_counters_present() {
        let mt: ManifestTelemetry = (&telemetry()).into();
        assert_eq!(mt.counters.get("memory_peak_bytes"), Some(&1024));
        assert_eq!(mt.counters.get("disk_bytes"), Some(&2048));
        assert_eq!(mt.counters.get("network_rx_bytes"), Some(&100));
        assert_eq!(mt.counters.get("network_tx_bytes"), Some(&200));
        assert_eq!(mt.counters.get("execution_duration_ms"), Some(&500));
        assert_eq!(mt.counters.get("checkpoint_count"), Some(&1));
        assert_eq!(mt.counters.get("rollback_count"), Some(&0));
    }

    #[test]
    fn telemetry_cpu_milli() {
        let mt: ManifestTelemetry = (&telemetry()).into();
        // 1.5 seconds -> 1500 milli
        assert_eq!(mt.counters.get("cpu_seconds_milli"), Some(&1500));
    }

    #[test]
    fn limits_to_trust() {
        let mt: ManifestTrust = (&limits()).into();
        assert_eq!(mt.level, TrustLevel::Community);
        assert!(mt.verified);
    }
}
