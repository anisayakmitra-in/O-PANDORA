//! Service container — type-based dependency injection.

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct ServiceContainer {
    services: HashMap<TypeId, Box<dyn Any + Send>>,
}

impl ServiceContainer {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    pub fn register<T: Send + 'static>(&mut self, service: T) {
        self.services.insert(TypeId::of::<T>(), Box::new(service));
    }

    pub fn get<T: Send + 'static>(&self) -> Option<&T> {
        self.services
            .get(&TypeId::of::<T>())
            .and_then(|s| s.downcast_ref::<T>())
    }

    pub fn get_mut<T: Send + 'static>(&mut self) -> Option<&mut T> {
        self.services
            .get_mut(&TypeId::of::<T>())
            .and_then(|s| s.downcast_mut::<T>())
    }

    pub fn contains<T: Send + 'static>(&self) -> bool {
        self.services.contains_key(&TypeId::of::<T>())
    }

    pub fn len(&self) -> usize {
        self.services.len()
    }
    pub fn is_empty(&self) -> bool {
        self.services.is_empty()
    }
    pub fn drain(&mut self) {
        self.services.clear();
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}
