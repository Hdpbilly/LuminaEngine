// world.rs
use crate::ecs_core::component::ComponentManager;
use crate::ecs_core::resource::ResourceManager;
use crate::ecs_core::entity::EntityManager;
use crate::ecs_core::system::System;
use crate::systems::input_system::InputSystem;
use crate::LuminaEngine;

pub struct World {
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new(engine: &'static LuminaEngine) -> Self {
        let mut world = Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
            systems: Vec::new(),
        };

        // System initialization
        world.systems.push(Box::new(InputSystem::new()));
        // world.systems.push(Box::new(RenderingSystem::new()));

        world
    }
}