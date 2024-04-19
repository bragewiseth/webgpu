/*
 Holds window state and handle to wgpu surface and device
*/


#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

use winit::
{
    event_loop::EventLoop,
    window::WindowBuilder,
    window::Window,
    event::WindowEvent
};


pub trait EngineTrait
{
    fn new() -> Self;
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>);
    fn window_input(&mut self, event: &WindowEvent) -> bool;
    fn device_input(&mut self, event: &winit::event::DeviceEvent) -> bool;
    fn update(&mut self);
    // fn render(&mut self) -> Result<(), wgpu::SurfaceError>;
}


pub struct Engine<'a>
{
    pub size: winit::dpi::PhysicalSize<u32>,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
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




#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn new_window(title: &str) -> (EventLoop<()>, Window)
{
    cfg_if::cfg_if! 
    {
        if #[cfg(target_arch = "wasm32")] 
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } 
        else { env_logger::init(); }
    }

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    window.set_inner_size(winit::dpi::LogicalSize::new(1400, 800));


    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;

        window.set_inner_size(PhysicalSize::new(1500, 1500));
        

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| 
            {
                let dst = doc.get_element_by_id("f_stop")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body.");
    }
    (event_loop, window)
}
