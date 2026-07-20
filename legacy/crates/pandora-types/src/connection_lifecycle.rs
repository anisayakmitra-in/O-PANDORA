//! Connection Lifecycle — heartbeat, stale detection, reconnect, task leasing.
//!
//! Production-grade fleet management: workers heartbeat to the controller,
//! stale connections get swept, tasks are leased to avoid double execution.
//! Inherited from the RuntimeNode abstraction.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Connection state for a fleet worker or runtime node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Connected,
    Disconnected,
    Stale,
    Reconnecting,
}

impl ConnectionState {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Disconnected => "disconnected",
            Self::Stale => "stale",
            Self::Reconnecting => "reconnecting",
        }
    }
}

/// A task lease — prevents double execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskLease {
    pub task_id: String,
    pub worker_id: String,
    pub acquired_at: SystemTime,
    pub expires_at: SystemTime,
    pub renewed_at: Option<SystemTime>,
}

impl TaskLease {
    pub fn new(task_id: &str, worker_id: &str, ttl_secs: u64) -> Self {
        let now = SystemTime::now();
        Self {
            task_id: task_id.into(),
            worker_id: worker_id.into(),
            acquired_at: now,
            expires_at: now + Duration::from_secs(ttl_secs),
            renewed_at: None,
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    pub fn renew(&mut self, ttl_secs: u64) {
        self.expires_at = SystemTime::now() + Duration::from_secs(ttl_secs);
        self.renewed_at = Some(SystemTime::now());
    }
}

/// A connection record for a fleet worker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionRecord {
    pub worker_id: String,
    pub node_id: String,
    pub state: ConnectionState,
    pub connected_at: SystemTime,
    pub last_heartbeat: SystemTime,
    pub address: Option<String>,
    pub capabilities: Vec<String>,
    pub active_leases: Vec<String>,
}

impl ConnectionRecord {
    /// Check if this connection is stale (no heartbeat in N seconds).
    pub fn is_stale(&self, max_age_secs: u64) -> bool {
        SystemTime::now()
            .duration_since(self.last_heartbeat)
            .map(|d| d.as_secs() >= max_age_secs)
            .unwrap_or(true)
    }
}

/// Connection lifecycle manager — for fleet controller.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectionLifecycle {
    connections: HashMap<String, ConnectionRecord>,
    leases: HashMap<String, TaskLease>,
    // Configuration
    pub heartbeat_timeout_secs: u64,
    pub stale_sweep_interval_secs: u64,
    pub lease_ttl_secs: u64,
    pub max_reconnect_attempts: u32,
}

impl ConnectionLifecycle {
    pub fn new() -> Self {
        Self {
            heartbeat_timeout_secs: 45,
            stale_sweep_interval_secs: 5,
            lease_ttl_secs: 300,
            max_reconnect_attempts: 3,
            ..Default::default()
        }
    }

    /// Register a new connection.
    pub fn connect(
        &mut self,
        worker_id: &str,
        node_id: &str,
        address: Option<&str>,
        capabilities: Vec<String>,
    ) {
        let now = SystemTime::now();
        self.connections.insert(
            worker_id.into(),
            ConnectionRecord {
                worker_id: worker_id.into(),
                node_id: node_id.into(),
                state: ConnectionState::Connected,
                connected_at: now,
                last_heartbeat: now,
                address: address.map(|s| s.into()),
                capabilities,
                active_leases: vec![],
            },
        );
    }

    /// Record a heartbeat from a worker.
    pub fn heartbeat(&mut self, worker_id: &str) -> bool {
        if let Some(conn) = self.connections.get_mut(worker_id) {
            conn.last_heartbeat = SystemTime::now();
            conn.state = ConnectionState::Connected;
            true
        } else {
            false
        }
    }

    /// Mark a connection as disconnected.
    pub fn disconnect(&mut self, worker_id: &str) {
        if let Some(conn) = self.connections.get_mut(worker_id) {
            conn.state = ConnectionState::Disconnected;
            // Release all active leases
            for lease_id in &conn.active_leases.clone() {
                self.leases.remove(lease_id);
            }
            conn.active_leases.clear();
        }
    }

    /// Mark as reconnecting.
    pub fn reconnecting(&mut self, worker_id: &str) -> bool {
        if let Some(conn) = self.connections.get_mut(worker_id) {
            conn.state = ConnectionState::Reconnecting;
            true
        } else {
            false
        }
    }

    /// Sweep stale connections. Returns list of stale worker IDs.
    pub fn sweep_stale(&mut self) -> Vec<String> {
        let mut stale = vec![];
        for conn in self.connections.values_mut() {
            if conn.state == ConnectionState::Connected
                && conn.is_stale(self.heartbeat_timeout_secs)
            {
                conn.state = ConnectionState::Stale;
                stale.push(conn.worker_id.clone());
            }
        }
        stale
    }

    /// Remove stale connections entirely.
    pub fn purge_stale(&mut self) -> usize {
        let before = self.connections.len();
        self.connections
            .retain(|_, c| !c.is_stale(self.heartbeat_timeout_secs * 3));
        before - self.connections.len()
    }

    /// Acquire a lease for a task. Returns None if already leased.
    pub fn acquire_lease(&mut self, task_id: &str, worker_id: &str) -> Option<TaskLease> {
        // Check if already leased
        if let Some(existing) = self.leases.get(task_id) {
            if !existing.is_expired() {
                return None; // Still held by someone
            }
        }
        let lease = TaskLease::new(task_id, worker_id, self.lease_ttl_secs);
        self.leases.insert(task_id.into(), lease.clone());
        if let Some(conn) = self.connections.get_mut(worker_id) {
            conn.active_leases.push(task_id.into());
        }
        Some(lease)
    }

    /// Release a lease.
    pub fn release_lease(&mut self, task_id: &str, worker_id: &str) {
        self.leases.remove(task_id);
        if let Some(conn) = self.connections.get_mut(worker_id) {
            conn.active_leases.retain(|l| l != task_id);
        }
    }

    /// Renew a lease.
    pub fn renew_lease(&mut self, task_id: &str) -> bool {
        if let Some(lease) = self.leases.get_mut(task_id) {
            if !lease.is_expired() {
                lease.renew(self.lease_ttl_secs);
                return true;
            }
        }
        false
    }

    /// Get a worker's connection record.
    pub fn worker(&self, worker_id: &str) -> Option<&ConnectionRecord> {
        self.connections.get(worker_id)
    }

    /// List all healthy (Connected) workers.
    pub fn healthy_workers(&self) -> Vec<&ConnectionRecord> {
        self.connections
            .values()
            .filter(|c| c.state == ConnectionState::Connected)
            .collect()
    }

    /// Count total connections.
    pub fn count(&self) -> usize {
        self.connections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_and_heartbeat() {
        let mut cl = ConnectionLifecycle::new();
        cl.connect("w1", "node1", None, vec!["shell".into()]);
        assert_eq!(cl.count(), 1);
        assert!(cl.heartbeat("w1"));
        assert!(cl.worker("w1").is_some());
    }

    #[test]
    fn stale_detection() {
        let mut cl = ConnectionLifecycle::new();
        cl.heartbeat_timeout_secs = 0; // Immediately stale
        cl.connect("w1", "node1", None, vec![]);
        let stale = cl.sweep_stale();
        assert_eq!(stale.len(), 1);
        assert_eq!(cl.worker("w1").unwrap().state, ConnectionState::Stale);
    }

    #[test]
    fn task_lease_acquire_and_release() {
        let mut cl = ConnectionLifecycle::new();
        cl.connect("w1", "node1", None, vec![]);
        let lease = cl.acquire_lease("task-1", "w1");
        assert!(lease.is_some());
        // Cannot acquire same lease again
        assert!(cl.acquire_lease("task-1", "w2").is_none());
        cl.release_lease("task-1", "w1");
        // Now it can be acquired again
        assert!(cl.acquire_lease("task-1", "w2").is_some());
    }
}
