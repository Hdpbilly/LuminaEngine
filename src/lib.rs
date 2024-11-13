// lib.rs
mod components;
mod engine_core;
mod ecs_core;
mod systems;
mod tracing;
use engine_core::core_loop::EngineLoop;
use engine_core::wgpures::WebGPUResources;
use engine_core::temporal::{self, AdvancedTime};
use engine_core::networking::NetworkResources;
use engine_core::rendering::RenderSystem;
use engine_core::inputhandler::InputHandler;
use engine_core::webworker::WebWorker;
use engine_core::world::World;
pub use tracing::init_tracing;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use std::rc::Rc;

pub struct EngineResources {
    pub webgpu_resource: WebGPUResources,
    pub temporal: AdvancedTime,
    pub rendering: RenderSystem,
    pub networking: NetworkResources,
    pub inputhandler: InputHandler,
    pub workers: WebWorker,
}

pub struct LuminaEngine {
    resources: EngineResources,
    coreloop: EngineLoop,
    world: World,
}

impl LuminaEngine {
    pub async fn new(canvas: HtmlCanvasElement) -> Self {
        let webgpu_resource = WebGPUResources::new(canvas).await.unwrap();
        let temporal = AdvancedTime::new(10, 10);
        let rendering = RenderSystem::new();
        let networking = NetworkResources::new();
        let inputhandler = InputHandler::new();
        let workers = WebWorker::new();

    

        Self {

        }
    }

    pub fn test(& mut self) {
        self.resources.temporal.update();
        self.world.components.
    }
}


#[wasm_bindgen]
pub async fn initalize_client(canvas: HtmlCanvasElement) {
    let engine = LuminaEngine::new(canvas).await; 
    let world = World::new(&engine);
}