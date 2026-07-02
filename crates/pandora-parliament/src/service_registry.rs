use pandora_types::services::{Service, ServiceId};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// A registered service implementation.
pub struct ServiceEntry {
    pub service_id: ServiceId,
    pub provider_name: String,
    pub instance: Arc<dyn Service>,
    pub version: String,
}

impl ServiceEntry {
    pub fn new(instance: Arc<dyn Service>) -> Self {
        let sid = instance.service_id();
        let name = instance.provider_name().to_string();
        let ver = instance.version().to_string();
        Self {
            service_id: sid,
            provider_name: name,
            instance,
            version: ver,
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

    pub fn register(&mut self, instance: Arc<dyn Service>) -> Result<(), ServiceRegistryError> {
        if self.services.len() >= self.capacity {
            return Err(ServiceRegistryError::AtCapacity);
        }
        let sid = instance.service_id();
        let name = instance.provider_name().to_string();
        let entry = ServiceEntry::new(instance);
        self.services.entry(sid.clone()).or_default().push(entry);
        info!(service = %sid, provider = %name, "registered");
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
                info!(service = %service_id, provider = %provider_name, "unregistered");
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

    #[derive(Debug)]
    struct MockService;
    impl Service for MockService {
        fn service_id(&self) -> ServiceId {
            ServiceId::Execution
        }
        fn provider_name(&self) -> &str {
            "mock-phoenix"
        }
        fn version(&self) -> &str {
            "0.1.0"
        }
    }

    #[test]
    fn register_and_resolve() {
        let mut r = ServiceRegistry::new();
        r.register(Arc::new(MockService)).unwrap();
        let resolved = r.resolve(&ServiceId::Execution).unwrap();
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].provider_name, "mock-phoenix");
    }

    #[test]
    fn multiple_providers() {
        let mut r = ServiceRegistry::new();
        r.register(Arc::new(MockService)).unwrap();

        #[derive(Debug)]
        struct MockService2;
        impl Service for MockService2 {
            fn service_id(&self) -> ServiceId {
                ServiceId::Execution
            }
            fn provider_name(&self) -> &str {
                "mock-phoenix-2"
            }
            fn version(&self) -> &str {
                "0.2.0"
            }
        }
        r.register(Arc::new(MockService2)).unwrap();
        assert_eq!(r.resolve(&ServiceId::Execution).unwrap().len(), 2);
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
        r.register(Arc::new(MockService)).unwrap();
        r.unregister(&ServiceId::Execution, "mock-phoenix").unwrap();
        assert!(!r.has_service(&ServiceId::Execution));
    }
}
