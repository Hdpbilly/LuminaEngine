// component.rs
use std::collections::HashMap;
use std::any::{Any, TypeId};
use crate::ecs_core::entity::Entity;

pub trait Component: Any + Sized {}
impl<T: Any + Sized> Component for T {}

pub trait ComponentManagerTrait {
    fn register_component<C: Component>(&mut self);
    fn insert_component<C: Component>(&mut self, entity: Entity, component: C);
    fn remove_component<C: Component>(&mut self, entity: &Entity);
    fn get_component<C: Component>(&self, entity: &Entity) -> Option<&C>;
    fn get_component_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C>;
}

pub struct ComponentManager {
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self { storages: HashMap::new(), }
    }

    /// Registers a new component type by inserting an empty storage for it.
    pub fn register_component<C: Component>(&mut self) {
        let type_id = TypeId::of::<C>();
        self.storages.entry(type_id).or_insert_with(|| Box::new(ComponentStorage::<C>::new()));
    }

     /// Retrieves an immutable reference to the component storage for type `C`.
    pub fn storage<C: Component>(&self) -> Option<&ComponentStorage<C>> {
        self.storages.get(&TypeId::of::<C>())
            .and_then(|boxed_storage| boxed_storage.downcast_ref::<ComponentStorage<C>>())
    }

    /// Retrieves a mutable reference to the component storage for type `C`.
    pub fn storage_mut<C: Component>(&mut self) -> Option<&mut ComponentStorage<C>> {
        self.storages.get_mut(&TypeId::of::<C>())
            .and_then(|boxed_storage| boxed_storage.downcast_mut::<ComponentStorage<C>>())
    }

    /// Inserts a component `C` for the given `Entity`.
    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) {
        self.register_component::<C>();
        if let Some(storage) = self.storage_mut::<C>() {
            storage.insert(entity, component);
        }
    }

    /// Removes a component `C` from the given `Entity`.
    pub fn remove<C: Component>(&mut self, entity: &Entity) {
        if let Some(storage) = self.storage_mut::<C>() {
            storage.remove(entity);
        }
    }

    /// Gets an immutable reference to component `C` of the given `Entity`.
    pub fn get<C: Component>(&self, entity: &Entity) -> Option<&C> {
        self.storage::<C>()?.get(entity)
    }

    /// Gets a mutable reference to component `C` of the given `Entity`.
    pub fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        self.storage_mut::<C>()?.get_mut(entity)
    }
}

impl ComponentManagerTrait for ComponentManager {
    fn register_component<C: Component>(&mut self) {
        self.register_component::<C>();
    }

    fn insert_component<C: Component>(&mut self, entity: Entity, component: C) {
        self.insert(entity, component);
    }

    fn remove_component<C: Component>(&mut self, entity: &Entity) {
        self.remove::<C>(entity);
    }

    fn get_component<C: Component>(&self, entity: &Entity) -> Option<&C> {
        self.get::<C>(entity)
    }

    fn get_component_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        self.get_mut::<C>(entity)
    }
}

pub struct ComponentStorage<C> {
    components: HashMap<Entity, C>,
}

impl<C> ComponentStorage<C> {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, component: C) {
        self.components.insert(entity, component);
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.components.remove(entity);
    }

    pub fn get(&self, entity: &Entity) -> Option<&C> {
        self.components.get(entity)
    }

    pub fn get_mut(&mut self, entity: &Entity) -> Option<&mut C> {
        self.components.get_mut(entity)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Entity, &C)> {
        self.components.iter()
    }
}
