// resources.rs
// non ECS based resources needed for the core of the engine... eg webGPUResources struct.

use wasm_bindgen::JsValue;
use web_sys::HtmlCanvasElement;
use wgpu::{self, Backends, Surface};
use tracing::{error};

pub trait EngineResources {
    fn get_instance(&self) -> &wgpu::Instance;
    fn get_device(&self) -> &wgpu::Device;
    fn get_queue(&self) -> &wgpu::Queue;
    fn get_adapter(&self) -> &wgpu::Adapter;
    fn get_surface(&self) -> &wgpu::Surface;
    fn get_config(&self) -> &wgpu::SurfaceConfiguration;
    fn resize(&mut self, width: u32, height: u32);
}

pub struct WebGPUResources {
    canvas: HtmlCanvasElement,
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
}

impl WebGPUResources {
    pub async fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::PRIMARY,
            dx12_shader_compiler: Default::default(),
            flags: Default::default(),
            gles_minor_version: Default::default()
        });

        let surface_target = wgpu::SurfaceTarget::Canvas(canvas.clone());
        let surface = instance.create_surface(surface_target)
        .map_err(|e| {
            error!("Failed to create surface: {}", e);
            JsValue::from_str(&format!("Could not create surface: {}", e)) 
        })?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,    
            compatible_surface: Some(&surface),
        })
        .await
        .ok_or_else(|| { 
            let err_msg = "No Suitable GPU adapter found";
            error!("{}", err_msg);
            JsValue::from_str(err_msg)
        })?;

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            },
        None,
        ).await
        .map_err(|e| { 
            error!("Failed to create device: {}", e);
            JsValue::from_str(&format!("Failed to create device: {}", e))
        })?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: canvas.width(),
            height: canvas.height(),
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);



        Ok(Self {
            canvas: canvas.clone(), 
            instance, 
            // surface, 
            adapter, 
            device, 
            queue, 
            config })
    }
}

impl EngineResources for WebGPUResources {
    fn get_instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    fn get_surface(&self) -> &wgpu::Surface {
        let surface_target = wgpu::SurfaceTarget::Canvas(self.canvas.clone());
        let surface = self.instance.create_surface(surface_target)
            .expect("Failed to create surface");
        let surface_box = Box::new(surface);
        Box::leak(surface_box)
    }
    
    fn get_queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.get_surface().configure(&self.device, &self. config);
    }
}