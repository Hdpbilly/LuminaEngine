// resource.rs
use std::rc::Rc;
use std::any::Any;
use std::collections::HashMap;
use std::any::TypeId;

pub struct ResourceManager<'a> {
    resources: HashMap<TypeId, &'a dyn Any>,
}

impl<'a> ResourceManager<'a> {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn insert<T: 'static + Any>(&mut self, resource: &'a T) {
        self.resources.insert(TypeId::of::<T>(), resource);
    }

    pub fn get<T: 'static + Any>(&self) -> Option<&'a T> {
        self.resources
            .get(&TypeId::of::<T>())
            .and_then(|resource| resource.downcast_ref::<T>())
    }
}