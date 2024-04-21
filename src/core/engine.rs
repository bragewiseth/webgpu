#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use crate::renderer::Rendercore;
use crate::new_device;



pub struct GraphicsEngine<'a>
{ 
    renderer: Rendercore<'a>,
    event_loop : winit::event_loop::EventLoop<()>,
    size: winit::dpi::PhysicalSize<u32>,
} 

pub struct ComputeEngine
{ 
    device: wgpu::Device,
    queue: wgpu::Queue,
    event_loop : winit::event_loop::EventLoop<()>,
}






impl<'a> GraphicsEngine<'a>
{
    pub async fn new() -> Self 
    {   
        let (event_loop, window) = new_window("kaos");
        let (device, queue, size, config, surface) = new_device!(window);
        let renderer = Rendercore
        {
            device,
            queue,
            surface,
            config,
        };
        Self
        {
            renderer,
            event_loop,
            size,
        }
    }
}








#[macro_export]
macro_rules! event_loop
{
    ($event_loop:expr, $engine:expr) => 
    {
        $event_loop.run(
            move |event, _, control_flow| 
            match event 
            {
                Event::WindowEvent 
                {
                    ref event,
                    window_id,
                } 
                if window_id == $engine.window.id() && !$engine.window_input(event) =>
                {
                    exit_event!(event, control_flow, $engine);
                } 
                Event::DeviceEvent { ref event, .. } => 
                {
                    $engine.device_input(event);
                }
                Event::RedrawRequested(window_id) if window_id == $engine.window.id() => 
                {
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    $engine.update(dt, last_render_time);
                    match $engine.render()
                    {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => $engine.resize($engine.window_state.size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                Event::MainEventsCleared => 
                {
                    $engine.window.request_redraw();
                }
                _ => {}
            }
        );
    };
}
        


#[macro_export]
macro_rules! rezize
{
    ($engine:expr, $new_size:expr) => 
    {
        if $new_size.width > 0 && $new_size.height > 0 
        {
            $engine.size = $new_size;
            $engine.config.width = $new_size.width;
            $engine.config.height = $new_size.height;
            $engine.renderer.surface.configure(&$engine.device, &$engine.config);
            $engine.camera.projection.resize($new_size.width, $new_size.height);
        }
    };
}



#[macro_export]
macro_rules! exit_event
{
    ($event:expr, $control_flow:expr) => 
    {
        match $event 
        {
            WindowEvent::CloseRequested | 
            WindowEvent::KeyboardInput 
            {
                input : KeyboardInput 
                {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit, 
            _ => {}
        }
    };
    ($event:expr, $control_flow:expr, $engine:expr) => 
    {
        match $event 
        {
            WindowEvent::CloseRequested | 
            WindowEvent::KeyboardInput 
            {
                input : KeyboardInput 
                {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
                ..
            } => *control_flow = ControlFlow::Exit, 
            WindowEvent::Resized(physical_size) =>  $engine.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  $engine.resize(**new_inner_size),
            _ => {}
        }
    };
}




#[macro_export]
macro_rules! new_device 
{
    () => 
    {
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
    };
    ($window:expr) => 
    {{
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        let surface = instance.create_surface($window)
            .expect("Failed to create surface");
        
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                required_features: wgpu::Features::empty(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None,
        ).await.unwrap();
        let size = $window.inner_size();
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
            desired_maximum_frame_latency: 1,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);
        let _modes = &surface_caps.present_modes;
        (device, queue, size, config, surface)
    }};
}




#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn new_window(title: &str) -> (winit::event_loop::EventLoop<()>, winit::window::Window)
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

    let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title(title)
        .build(&event_loop)
        .unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    window.request_inner_size(winit::dpi::PhysicalSize::new(1500, 1500));

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
