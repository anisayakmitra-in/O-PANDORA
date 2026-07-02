use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// A unique identifier for a constitutional service.
/// Always refer to services by their contract, not by implementation name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServiceId {
    Memory,
    Execution,
    Planning,
    Governance,
    Evolution,
    Identity,
    Security,
    Custom(String),
}

impl ServiceId {
    pub fn as_str(&self) -> &str {
        match self {
            ServiceId::Memory => "memory",
            ServiceId::Execution => "execution",
            ServiceId::Planning => "planning",
            ServiceId::Governance => "governance",
            ServiceId::Evolution => "evolution",
            ServiceId::Identity => "identity",
            ServiceId::Security => "security",
            ServiceId::Custom(s) => s,
        }
    }
}

/// A registered service implementation.
pub struct ServiceEntry {
    pub service_id: ServiceId,
    pub provider_name: String,
    pub instance: Arc<dyn Any + Send + Sync>,
    pub version: String,
}

impl ServiceEntry {
    pub fn new(
        service_id: ServiceId,
        provider_name: impl Into<String>,
        instance: Arc<dyn Any + Send + Sync>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            service_id,
            provider_name: provider_name.into(),
            instance,
            version: version.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceRegistryError {
    #[error("no providers registered for service {0:?}")]
    NoProviders(ServiceId),
    #[error("provider {0} not found for service {1:?}")]
    ProviderNotFound(String, ServiceId),
    #[error("provider {0} is already registered for service {1:?}")]
    AlreadyRegistered(String, ServiceId),
    #[error("service registry is at capacity")]
    AtCapacity,
}

/// The Service Registry - canonical service resolution.
/// Instead of `PhoenixHarness::new()`, resolve by contract:
/// `registry.resolve(ServiceId::Execution)?`
pub struct ServiceRegistry {
    services: HashMap<ServiceId, Vec<ServiceEntry>>,
    capacity: usize,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            capacity: usize::MAX,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            services: HashMap::new(),
            capacity,
        }
    }

    pub fn register(
        &mut self,
        service_id: ServiceId,
        provider_name: impl Into<String>,
        instance: Arc<dyn Any + Send + Sync>,
        version: impl Into<String>,
    ) -> Result<(), ServiceRegistryError> {
        let provider_name = provider_name.into();
        if self.services.len() >= self.capacity {
            return Err(ServiceRegistryError::AtCapacity);
        }
        let entry = ServiceEntry::new(service_id.clone(), &provider_name, instance, version);
        self.services
            .entry(service_id.clone())
            .or_default()
            .push(entry);
        info!(service = %service_id.as_str(), provider = %provider_name, "registered provider");
        Ok(())
    }

    pub fn resolve(&self, service_id: &ServiceId) -> Result<&[ServiceEntry], ServiceRegistryError> {
        self.services
            .get(service_id)
            .filter(|entries| !entries.is_empty())
            .map(|entries| entries.as_slice())
            .ok_or(ServiceRegistryError::NoProviders(service_id.clone()))
    }

    pub fn resolve_named(
        &self,
        service_id: &ServiceId,
        provider_name: &str,
    ) -> Result<&ServiceEntry, ServiceRegistryError> {
        self.services
            .get(service_id)
            .and_then(|entries| entries.iter().find(|e| e.provider_name == provider_name))
            .ok_or_else(|| {
                ServiceRegistryError::ProviderNotFound(
                    provider_name.to_string(),
                    service_id.clone(),
                )
            })
    }

    pub fn has_service(&self, service_id: &ServiceId) -> bool {
        self.services
            .get(service_id)
            .is_some_and(|entries| !entries.is_empty())
    }

    pub fn services(&self) -> Vec<&ServiceId> {
        self.services.keys().collect()
    }

    pub fn providers(&self, service_id: &ServiceId) -> Vec<&str> {
        self.services
            .get(service_id)
            .map(|entries| entries.iter().map(|e| e.provider_name.as_str()).collect())
            .unwrap_or_default()
    }

    pub fn unregister(
        &mut self,
        service_id: &ServiceId,
        provider_name: &str,
    ) -> Result<(), ServiceRegistryError> {
        if let Some(entries) = self.services.get_mut(service_id) {
            let before = entries.len();
            entries.retain(|e| e.provider_name != provider_name);
            if entries.len() < before {
                info!(service = %service_id.as_str(), provider = %provider_name, "unregistered");
                return Ok(());
            }
        }
        Err(ServiceRegistryError::ProviderNotFound(
            provider_name.to_string(),
            service_id.clone(),
        ))
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_resolve() {
        let mut r = ServiceRegistry::new();
        r.register(
            ServiceId::Execution,
            "phoenix",
            Arc::new(String::from("mock")),
            "0.1.0",
        )
        .unwrap();
        let resolved = r.resolve(&ServiceId::Execution).unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].provider_name, "phoenix");
    }

    #[test]
    fn multiple_providers() {
        let mut r = ServiceRegistry::new();
        r.register(
            ServiceId::Memory,
            "anubis",
            Arc::new(String::from("a")),
            "0.1.0",
        )
        .unwrap();
        r.register(
            ServiceId::Memory,
            "enterprise",
            Arc::new(String::from("e")),
            "0.2.0",
        )
        .unwrap();
        assert_eq!(r.resolve(&ServiceId::Memory).unwrap().len(), 2);
    }

    #[test]
    fn missing_service_errors() {
        let r = ServiceRegistry::new();
        assert!(matches!(
            r.resolve(&ServiceId::Security),
            Err(ServiceRegistryError::NoProviders(_))
        ));
    }

    #[test]
    fn unregister_works() {
        let mut r = ServiceRegistry::new();
        r.register(
            ServiceId::Execution,
            "phoenix",
            Arc::new(String::from("p")),
            "1.0",
        )
        .unwrap();
        r.unregister(&ServiceId::Execution, "phoenix").unwrap();
        assert!(!r.has_service(&ServiceId::Execution));
    }

    #[test]
    fn list_providers() {
        let mut r = ServiceRegistry::new();
        r.register(
            ServiceId::Memory,
            "anubis",
            Arc::new(String::from("a")),
            "1.0",
        )
        .unwrap();
        assert_eq!(r.providers(&ServiceId::Memory), vec!["anubis"]);
    }
}
