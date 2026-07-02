use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

/// A unique lease identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeaseId(String);

impl Default for LeaseId {
    fn default() -> Self {
        Self::new()
    }
}

impl LeaseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LeaseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The current state of a lease.
#[derive(Debug, Clone, PartialEq)]
pub enum LeaseState {
    Active,
    Expired,
    Revoked,
    Released,
}

/// A lease represents temporary, revocable ownership of a resource.
///
/// In Pandora's constitutional architecture, resources are **never**
/// permanently owned. They are leased. When a lease expires or is
/// revoked, the resource must be returned gracefully.
///
/// This applies to: providers, models, memory, sandboxes, capabilities.
#[derive(Debug, Clone)]
pub struct Lease {
    pub lease_id: LeaseId,
    pub resource_id: String,
    pub holder: String,
    pub state: LeaseState,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_renewals: u32,
    pub renewal_count: u32,
}

impl Lease {
    pub fn new(
        resource_id: impl Into<String>,
        holder: impl Into<String>,
        ttl_seconds: Option<u64>,
    ) -> Self {
        let expires_at =
            ttl_seconds.map(|secs| Utc::now() + chrono::Duration::seconds(secs as i64));
        Self {
            lease_id: LeaseId::new(),
            resource_id: resource_id.into(),
            holder: holder.into(),
            state: LeaseState::Active,
            issued_at: Utc::now(),
            expires_at,
            max_renewals: 3,
            renewal_count: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expiry) => Utc::now() > expiry,
            None => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LeaseManagerError {
    #[error("resource {0} is not currently leased")]
    NotLeased(String),
    #[error("lease {0} not found")]
    LeaseNotFound(LeaseId),
    #[error("lease {0} has expired")]
    LeaseExpired(LeaseId),
    #[error("lease {0} has been revoked")]
    LeaseRevoked(LeaseId),
    #[error("max renewals ({0}) reached for lease")]
    MaxRenewalsReached(u32),
    #[error("resource {0} is already leased")]
    AlreadyLeased(String),
}

/// The Lease Manager governs temporary ownership of all constitutional resources.
///
/// Every resource in Pandora is leased — never permanently owned.
/// When a lease expires, the resource must be returned or the lease renewed.
pub struct LeaseManager {
    leases: HashMap<LeaseId, Lease>,
    resource_leases: HashMap<String, LeaseId>,
}

impl LeaseManager {
    pub fn new() -> Self {
        Self {
            leases: HashMap::new(),
            resource_leases: HashMap::new(),
        }
    }

    /// Acquire a lease on a resource. Returns an error if already leased.
    pub fn acquire(
        &mut self,
        resource_id: impl Into<String>,
        holder: impl Into<String>,
        ttl_seconds: Option<u64>,
    ) -> Result<Lease, LeaseManagerError> {
        let resource_id = resource_id.into();
        if self.resource_leases.contains_key(&resource_id) {
            return Err(LeaseManagerError::AlreadyLeased(resource_id));
        }
        let lease = Lease::new(&resource_id, holder, ttl_seconds);
        let lease_id = lease.lease_id.clone();
        self.leases.insert(lease_id.clone(), lease.clone());
        self.resource_leases.insert(resource_id, lease_id);
        Ok(lease)
    }

    /// Release a lease by ID. Returns the resource to the pool.
    pub fn release(&mut self, lease_id: &LeaseId) -> Result<Lease, LeaseManagerError> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or(LeaseManagerError::LeaseNotFound(lease_id.clone()))?;
        lease.state = LeaseState::Released;
        let resource_id = lease.resource_id.clone();
        self.resource_leases.remove(&resource_id);
        Ok(lease.clone())
    }

    /// Renew a lease. Extends the TTL and increments renewal counter.
    pub fn renew(
        &mut self,
        lease_id: &LeaseId,
        additional_seconds: u64,
    ) -> Result<Lease, LeaseManagerError> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or(LeaseManagerError::LeaseNotFound(lease_id.clone()))?;
        if lease.state != LeaseState::Active {
            return Err(LeaseManagerError::LeaseRevoked(lease_id.clone()));
        }
        if lease.renewal_count >= lease.max_renewals {
            return Err(LeaseManagerError::MaxRenewalsReached(lease.max_renewals));
        }
        let new_expiry = Utc::now() + chrono::Duration::seconds(additional_seconds as i64);
        lease.expires_at = Some(new_expiry);
        lease.renewal_count += 1;
        Ok(lease.clone())
    }

    /// Revoke a lease immediately. Forceful release.
    pub fn revoke(&mut self, lease_id: &LeaseId) -> Result<Lease, LeaseManagerError> {
        let lease = self
            .leases
            .get_mut(lease_id)
            .ok_or(LeaseManagerError::LeaseNotFound(lease_id.clone()))?;
        lease.state = LeaseState::Revoked;
        let resource_id = lease.resource_id.clone();
        self.resource_leases.remove(&resource_id);
        Ok(lease.clone())
    }

    /// Get the current lease for a resource, if any.
    pub fn get_lease_for_resource(&self, resource_id: &str) -> Option<&Lease> {
        self.resource_leases
            .get(resource_id)
            .and_then(|lease_id| self.leases.get(lease_id))
    }

    /// Check if a resource is currently leased.
    pub fn is_leased(&self, resource_id: &str) -> bool {
        self.resource_leases.contains_key(resource_id)
    }

    /// List all active leases.
    pub fn active_leases(&self) -> Vec<&Lease> {
        self.leases
            .values()
            .filter(|l| l.state == LeaseState::Active)
            .collect()
    }
}

impl Default for LeaseManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acquire_and_release_lease() {
        let mut lm = LeaseManager::new();
        let lease = lm.acquire("ollama", "phoenix", Some(60)).unwrap();
        assert_eq!(lease.state, LeaseState::Active);
        assert!(lm.is_leased("ollama"));

        lm.release(&lease.lease_id).unwrap();
        assert!(!lm.is_leased("ollama"));
    }

    #[test]
    fn cannot_acquire_twice() {
        let mut lm = LeaseManager::new();
        lm.acquire("gpu-0", "moira", Some(300)).unwrap();
        let result = lm.acquire("gpu-0", "shani", Some(300));
        assert!(matches!(result, Err(LeaseManagerError::AlreadyLeased(_))));
    }

    #[test]
    fn renew_lease() {
        let mut lm = LeaseManager::new();
        let lease = lm
            .acquire("model-qwen", "capability-resolver", Some(60))
            .unwrap();
        let renewed = lm.renew(&lease.lease_id, 120).unwrap();
        assert_eq!(renewed.renewal_count, 1);
    }

    #[test]
    fn renew_past_max_fails() {
        let mut lm = LeaseManager::new();
        let lease = lm.acquire("test-resource", "test", Some(10)).unwrap();
        for _ in 0..3 {
            let _ = lm.renew(&lease.lease_id, 10);
        }
        let result = lm.renew(&lease.lease_id, 10);
        assert!(matches!(
            result,
            Err(LeaseManagerError::MaxRenewalsReached(_))
        ));
    }

    #[test]
    fn revoke_lease() {
        let mut lm = LeaseManager::new();
        let lease = lm.acquire("ollama", "test", Some(60)).unwrap();
        let revoked = lm.revoke(&lease.lease_id).unwrap();
        assert_eq!(revoked.state, LeaseState::Revoked);
        assert!(!lm.is_leased("ollama"));
    }
}
