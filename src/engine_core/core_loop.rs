// core_loop.rs
use crate::systems::input_system::InputSystem;
use crate::ecs_core::system::System;
use crate::engine_core::world::World;
use crate::LuminaEngine;

pub struct EngineLoop<'window> {
    core: LuminaEngine<'window>,
    systems: Vec<Box<dyn System>>,
    world: World,
}

impl<'window> EngineLoop<'window> {
    pub fn new(core: LuminaEngine) -> Self {
        let systems: Vec<Box<dyn System>> = vec![
            Box::new(InputSystem::new()),
        ];
    }

    pub fn start(self) {

    }

    async fn run_loop(mut game_loop: Self) {

        }
    }