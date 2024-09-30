// input_system.rs
use crate::engine_core::world::World;
use crate::ecs_core::system::System;

pub struct InputSystem {

}

impl InputSystem {
    pub fn new() -> Self {
        Self {
        
        }
    }
}

impl System for InputSystem {
    fn update(&mut self, dt: f32, world: &mut World) {
        let dt = world.time.scaled_delta();
        for (entity, input_component) in world.inputs.iter() {
            
        }
    }
}