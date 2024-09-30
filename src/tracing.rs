// tracing.rs
use tracing_wasm;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(
       tracing_wasm::WASMLayerConfigBuilder::default()
            .set_max_level(tracing::Level::TRACE)
            .build()
    );
}

#[wasm_bindgen] // cna likely get rid of this whhole thing 
pub fn init_tracing() {
    start();
}