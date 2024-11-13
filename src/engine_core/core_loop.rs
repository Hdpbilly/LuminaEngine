// core_loop.rs
use crate::systems::input_system::InputSystem;
use crate::ecs_core::system::System;
use crate::engine_core::world::World;
use crate::LuminaEngine;

use super::temporal::{self, AdvancedTime};

pub struct EngineLoop {
    core: &'static LuminaEngine,
    world: World,
}

impl EngineLoop {
    pub fn new(core: &'static LuminaEngine) -> Self {
        let world = World::new(core);
        Self { core, world }
    }

    pub fn start(self) {
        // Assuming we're in a WASM environment, we might want to use 
        // wasm_bindgen_futures::spawn_local to start the async loop
        wasm_bindgen_futures::spawn_local(self.run_loop());
    }

    async fn run_loop(mut self) {
        loop {
            // Update game state
            self.core.temporal.update();

            // Render frame

            // Yield to browser to keep things responsive
            // wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(&mut |resolve, _| {
            //     web_sys::window()
            //         .unwrap()
            //         .request_animation_frame(&resolve)
            //         .unwrap();
            // }))
            // .await
            // .unwrap();
        }
    }

}