// resource.rs
use std::any::Any;
use std::collections::HashMap;
use std::any::TypeId;

pub struct ResourceManager {
    resources: HashMap<TypeId, &'static dyn Any>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: 'static>(&mut self, resource: &'static T) {
        self.resources.insert(TypeId::of::<T>(), resource);
    }

    pub fn get<T: 'static>(&self) -> Option<&'static T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|&r| r.downcast_ref::<T>())
    }
}