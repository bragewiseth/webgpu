#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;





pub fn toggle_cursor(window : &winit::window::Window) -> bool
{
    static mut MOUSE_LOCKED: bool = false;
    unsafe 
    { 
        if MOUSE_LOCKED == false 
        {
            window.set_cursor_grab(winit::window::CursorGrabMode::Confined).or_else(|_| 
            window.set_cursor_grab(winit::window::CursorGrabMode::Locked)).unwrap();
            window.set_cursor_visible(false);
            MOUSE_LOCKED = true; 
        }
        else
        {
            window.set_cursor_grab(winit::window::CursorGrabMode::None).unwrap();
            window.set_cursor_visible(true);
            MOUSE_LOCKED = false;
        }
    }
    true
}




#[macro_export]
macro_rules! event_loop
{
    (
        window                  => $window:expr,
        windowsize              => $size:expr,
        update_handle           => $update:expr,
        scene                   => $scene:expr
    ) =>
    {
        event_loop.run(
            move |event, _, control_flow| 
            match event 
            {
                /****************************************************************
                 * Window Events
                 ***************************************************************/
                Event::WindowEvent { ref event, window_id, } 
                if window_id == $window.id() =>
                {
                    match event 
                    {
                        WindowEvent::CloseRequested | WindowEvent::KeyboardInput 
                        {
                            input : KeyboardInput 
                            {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => *control_flow = ControlFlow::Exit, 
                        WindowEvent::Resized(physical_size) =>  $window.resize(*physical_size),
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  $window.resize(**new_inner_size),
                        WindowEvent::KeyboardInput 
                        {
                            input: KeyboardInput 
                            {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Tab),
                                ..
                            },
                            ..
                        } => toggle_cursor(&$window),
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(key),
                                    state,
                                    ..
                                },
                            ..
                        } => $scene.key_input(key, state),
                        WindowEvent::MouseWheel { delta, .. } => $scene.mousewheel_input(delta);
                        _ => {}
                    }
                }
                /****************************************************************
                 * Device Events
                 ***************************************************************/ 
                Event::DeviceEvent { ref event, .. } => $scene.device_input(event),
                /****************************************************************
                 * Redraw Requested
                 ***************************************************************/
                Event::RedrawRequested(window_id) if window_id == $window.id() => 
                {
                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    match $update(&$scene,dt);
                    {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost) => $window.resize($size),
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => eprintln!("{:?}", e),
                    }
                }
                /****************************************************************
                 * Main Events Cleared
                 ***************************************************************/
                Event::MainEventsCleared => $window.request_redraw(),
                /****************************************************************
                 * Default
                 ***************************************************************/
                _ => {}
            }
        );
    };
    (
        $key:ident => $value:expr,
        $($rest:tt)*
    ) => 
    {
        event_loop! 
        {
            $($rest)*
            $key => $value,
        }
    };
}




#[macro_export]
macro_rules! rezize
{
    ($new_size:expr) => 
    {
        if $new_size.width > 0 && $new_size.height > 0 
        {
            self.size = $new_size;
            renderer.config.width = $new_size.width;
            renderer.config.height = $new_size.height;
            renderer.renderer.surface.configure(&renderer.device, &renderer.config);
            renderer.camera.projection.resize($new_size.width, $new_size.height);
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

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
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
