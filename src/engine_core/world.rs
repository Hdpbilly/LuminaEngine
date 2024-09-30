// world.rs
use crate::ecs_core::component::ComponentManager;
use crate::ecs_core::resource::ResourceManager;
use crate::ecs_core::entity::EntityManager;
use crate::LuminaEngine;
use std::rc::Rc;

pub struct World<'a> {
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub resources: ResourceManager<'a>,
    systems: Vec<Box<dyn System>>,
}

impl<'a> World<'a> {
    pub fn new(engine: &'a LuminaEngine) -> Self {
        let mut world = Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
            resources: ResourceManager::new(),
        };

        world.resources.insert(&engine.webgpu_resource);
        world.resources.insert(&engine.temporal);
        world.resources.insert(&engine.rendering);
        world.resources.insert(&engine.networking);
        world.resources.insert(&engine.inputhandler);
        world.resources.insert(&engine.workers);

        world
    }
}