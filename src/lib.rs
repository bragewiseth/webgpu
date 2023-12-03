pub mod app;
pub mod core;
// mod shaders;
// use winit::window::Window;







#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;


use winit::
{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};









#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run()
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
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_inner_size(winit::dpi::LogicalSize::new(1000, 750));


    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(1200, 1000));

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

    // Window setup...

    let mut state = app::engine::Engine::new(window).await;
    let mut last_render_time = instant::Instant::now();  // NEW!

    // Event loop...

    event_loop.run(
        move |event, _, control_flow| 
        match event 
        {
            Event::WindowEvent 
            {
                ref event,
                window_id,
            } 
            if window_id == state.window().id() && !state.window_input(event) =>
            {
                match event 
                {
                    // #[cfg(target_arch = "wasm32")]
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

                    WindowEvent::Resized(physical_size) =>  state.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  state.resize(**new_inner_size),
                    _ => {}
                }
            } 
            Event::DeviceEvent { ref event, .. } => 
            {
                state.device_input(event);
            }
            Event::RedrawRequested(window_id) if window_id == state.window().id() => 
            {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                state.update(dt, last_render_time);
                match state.render() 
                {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => 
            {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }

    );
}
