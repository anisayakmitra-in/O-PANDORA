//! Capability Lease Manager.
//!
//! RAHU manages capability leases. Every capability
//! is leased, never directly invoked. This manager
//! handles the lease lifecycle: request → resolve →
//! grant → track → release.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use pandora_types::capability_leasing::{
    CapabilityLease, CapabilityRequest, CapabilitySession, CapabilityStatistics, LeaseStatus,
};

/// Manages capability leases for an execution session.
pub struct CapabilityLeaseManager {
    leases: Arc<Mutex<BTreeMap<String, CapabilityLease>>>,
    #[allow(dead_code)]
    sessions: Arc<Mutex<BTreeMap<String, CapabilitySession>>>,
    next_id: Arc<Mutex<u64>>,
}

impl CapabilityLeaseManager {
    pub fn new() -> Self {
        CapabilityLeaseManager {
            leases: Arc::new(Mutex::new(BTreeMap::new())),
            sessions: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Request a capability lease.
    pub fn request(&self, req: &CapabilityRequest) -> CapabilityLease {
        let id = self.next_id();
        let now = Self::now_ms();
        let lease = CapabilityLease {
            lease_id: format!("lease-{}", id),
            capability: req.capability.clone(),
            provider: req
                .preferred_provider
                .clone()
                .unwrap_or_else(|| "default".to_string()),
            session_id: String::new(),
            priority: req.priority,
            granted_at_ms: now,
            expires_at_ms: now + req.timeout_ms,
            status: LeaseStatus::Active,
        };
        self.leases
            .lock()
            .unwrap()
            .insert(lease.lease_id.clone(), lease.clone());
        lease
    }

    /// Release a lease.
    pub fn release(&self, lease_id: &str) -> bool {
        let mut leases = self.leases.lock().unwrap();
        if let Some(lease) = leases.get_mut(lease_id) {
            lease.status = LeaseStatus::Released;
            true
        } else {
            false
        }
    }

    /// Revoke a lease.
    pub fn revoke(&self, lease_id: &str) -> bool {
        let mut leases = self.leases.lock().unwrap();
        if let Some(lease) = leases.get_mut(lease_id) {
            lease.status = LeaseStatus::Revoked;
            true
        } else {
            false
        }
    }

    /// Get a lease by ID.
    pub fn get(&self, lease_id: &str) -> Option<CapabilityLease> {
        self.leases.lock().unwrap().get(lease_id).cloned()
    }

    /// List all active leases.
    pub fn active_leases(&self) -> Vec<CapabilityLease> {
        self.leases
            .lock()
            .unwrap()
            .values()
            .filter(|l| l.status == LeaseStatus::Active)
            .cloned()
            .collect()
    }

    /// Statistics.
    pub fn statistics(&self) -> CapabilityStatistics {
        let leases = self.leases.lock().unwrap();
        let total = leases.len() as u64;
        let active = leases
            .values()
            .filter(|l| l.status == LeaseStatus::Active)
            .count() as u64;
        let failed = leases
            .values()
            .filter(|l| l.status == LeaseStatus::Failed)
            .count() as u64;
        CapabilityStatistics {
            total_leases: total,
            active_leases: active,
            failed_leases: failed,
            total_cost_cents: 0,
            avg_duration_ms: 0,
        }
    }

    fn next_id(&self) -> u64 {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        current
    }

    fn now_ms() -> u64 {
        0 // placeholder: real impl uses SystemTime
    }
}

impl Default for CapabilityLeaseManager {
    fn default() -> Self {
        Self::new()
    }
}
