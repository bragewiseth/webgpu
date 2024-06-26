extern crate kaos;
use kaos::engine::toggle_cursor;
use winit::event::*;
use winit::event_loop::ControlFlow;
use winit::keyboard::*;
use winit::keyboard::KeyCode;
use winit::event::WindowEvent::*;
mod scene;
mod renderer;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;






fn main() 
{
    pollster::block_on(run());
}



#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
async fn run()
{
    let (event_loop, window) = kaos::engine::new_window("floating"); // name it floating since i3wm makes windows with that title float
    let (device, queue, windowsize, config, surface) = kaos::new_device!(window);
    let scene = scene::Scene::new(&config).await;
    let gpu = renderer::Renderer::new(device, queue, windowsize, surface, config);
    gpu.load_assets(&scene).await;

    let last_render_time = instant::Instant::now();
    event_loop.run(
        move |event, control_flow| 
        match event 
        {
            /****************************************************************
             * Window Events
             ***************************************************************/
            Event::WindowEvent { ref event, window_id, } 
            if window_id == window.id() =>
            {
                match event 
                {
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput 
                    {
                        event: KeyEvent
                        {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(Escape),
                            ..
                        },
                        ..
                    } => *control_flow = ControlFlow::Exit, 
                    WindowEvent::Resized(physical_size) =>  
                    {
                        window.resize(*physical_size),
                        gpu.resize(*physical_size);
                    }

                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>  
                    {
                        window.resize(**new_inner_size),
                        gpu.resize(**new_inner_size);
                    }
                    WindowEvent::KeyboardInput 
                    {
                        event: KeyEvent
                        {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyTab),
                            ..
                        },
                        ..
                    } => toggle_cursor(&window),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state,
                                ..
                            },
                        ..
                    } => scene.key_input(key, state),
                    WindowEvent::MouseWheel { delta, .. } => scene.mousewheel_input(delta);
                    _ => {}
                }
            }
            /****************************************************************
             * Device Events
             ***************************************************************/ 
            Event::DeviceEvent { ref event, .. } => scene.device_input(event),
            /****************************************************************
             * Redraw Requested
             ***************************************************************/
            Event::RedrawRequested(window_id) if window_id == window.id() => 
            {
                let now = instant::Instant::now();
                let dt = now - last_render_time;
                last_render_time = now;
                match gpu.update(&scene,dt)
                {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => window.resize(size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            /****************************************************************
             * Main Events Cleared
             ***************************************************************/
            Event::MainEventsCleared => window.request_redraw(),
            /****************************************************************
             * Default
             ***************************************************************/
            _ => {}
        }
    );
}

