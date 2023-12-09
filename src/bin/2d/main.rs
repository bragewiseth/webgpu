
extern crate fstop;
mod app;
mod world;

use fstop::engine::WindowState;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
use winit::
{
    event::*,
    event_loop::ControlFlow,
};



fn main() 
{
    pollster::block_on(run());
}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
async fn run()
{
    // Window setup...
    let (event_loop, window) = fstop::window::new("f✦stop");
    let (device, queue, window_state) = fstop::new_device!(window);
    let mut engine = app::Engine::new(window_state, device, queue).await;
    let mut last_render_time = instant::Instant::now();

    event_loop.run(
        move |event, _, control_flow| 
        match event 
        {
            Event::WindowEvent 
            {
                ref event,
                window_id,
            } 
            if window_id == engine.window().id() && !engine.window_input(event) =>
            {
                match event 
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

                    WindowEvent::Resized(physical_size) =>  engine.resize(*physical_size),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  engine.resize(**new_inner_size),
                    _ => {}
                }
            } 
            Event::DeviceEvent { ref event, .. } => 
            {
                engine.device_input(event);
            }
            Event::RedrawRequested(window_id) if window_id == engine.window().id() => 
            {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                engine.update(dt, last_render_time);
                match engine.render()
                {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => engine.resize(engine.window_state.size),
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
                engine.window().request_redraw();
            }
            _ => {}
        }

    );
}
