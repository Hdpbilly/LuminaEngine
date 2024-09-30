// world.rs
use crate::ecs_core::component::ComponentManager;
use crate::ecs_core::resource::ResourceManager;
use crate::ecs_core::entity::EntityManager;
use crate::ecs_core::system::System;
use crate::systems::input_system::InputSystem;
use crate::systems::rendering_system::RenderingSystem;
use crate::LuminaEngine;

pub struct World<'a> {
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub resources: ResourceManager<'a>,
    systems: Vec<Box<dyn System>>,
}

impl<'a> World<'a> {
    pub fn new(engine: &'a LuminaEngine) -> Self {
        let systems: Vec<Box<dyn System>> = vec![
            // Box::new(InputSystem::new()),
            // Box::new(InputSystem::new()),
            // Box::new(InputSystem::new()),
        ];
        let mut world = Self {
            entities: EntityManager::new(),
            components: ComponentManager::new(),
            resources: ResourceManager::new(),
            systems,

        };
        // resource initalization 
        world.resources.insert(&engine.webgpu_resource);
        world.resources.insert(&engine.temporal);
        world.resources.insert(&engine.rendering);
        world.resources.insert(&engine.networking);
        world.resources.insert(&engine.inputhandler);
        world.resources.insert(&engine.workers);

        // system initalization
        world.systems.push(Box::new(InputSystem::new()));
        // world.systems.push(Box::new(RenderingSystem::new()));

        world
    }
}