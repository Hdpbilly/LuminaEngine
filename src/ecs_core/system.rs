// system.rs
use crate::engine_core::world::World;

pub trait System {
    fn update(&mut self, world: &mut World);
}