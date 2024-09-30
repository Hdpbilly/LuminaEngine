import init, { initialize_client } from './pkg/luminaengine.js';

async function start() {
   // console.log("Starting WebGPU initialization...");
    try {
        await init();
        //console.log("WASM module initialized");
        
        // Add a small delay to ensure the canvas is rendered
        await new Promise(resolve => setTimeout(resolve, 100));
        
        //console.log("Attempting to run WebGPU...");
        await initialize_client('canvas');
        //console.log("WebGPU run successful");
    } catch (error) {
        console.error('Failed to run WebGPU:', error);
    }
}

start();