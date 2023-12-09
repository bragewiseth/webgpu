use winit::window::Window;
use winit::event::WindowEvent;

pub trait Engine
{
    fn new() -> Self;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn window_input(&mut self, event: &WindowEvent) -> bool;
    fn device_input(&mut self, event: &winit::event::DeviceEvent) -> bool;
    fn window(&self) -> Option<&WindowState>;
    fn update(&mut self);
    fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}


pub struct WindowState
{
    pub size: winit::dpi::PhysicalSize<u32>,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
    pub window: Window,
}

#[macro_export]
macro_rules! new_device {
    () => {{
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();
        (device, queue)
    }};



    ($window:expr) => {{
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        let surface = unsafe { instance.create_surface(&$window) }.unwrap();
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let size = $window.inner_size();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration 
        {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let _modes = &surface_caps.present_modes;
        let window_state = WindowState { size, config, surface, window:$window };
        (device, queue, window_state)
    }};
}
