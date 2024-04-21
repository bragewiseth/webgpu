use crate::world;
use fstop::core::engine::WindowState;
use fstop::create_render_pass;
use fstop::core::model::{
    Material,
    Mesh
};
use fstop::core::texture::Texture;
use fstop::core::camera::{ 
    Camera, 
    CameraController,
    Projection
};


use winit::window::Window;
use winit::event::*;
use winit::window::CursorGrabMode;


const PIXEL_SIZE : u32 = 1;


pub struct Engine
{ 
    renderer: Renderer,
    event_loop : winit::EventsLoop,
    window : winit::Window,
    size: winit::dpi::PhysicalSize<u32>,
    config: winit::window::WindowAttributes,
    mouse_locked: bool
} 





impl EngineTrait for Engine
{
    async fn new(window_state:WindowState, device:wgpu::Device, queue:wgpu::Queue) -> Self 
    {   
        let (event_loop, window) = kaos::window::new("f✦stop");
        let (device, queue, size, config, surface) = kaos::new_device!(window);
        let mut last_render_time = instant::Instant::now();
        let renderer = Renderer::new(device, queue, surface);
        Self
        {
            renderer,
            event_loop,
            window,
            size,
            config,
            mouse_locked: false,
        }
    }



    fn run()
    {
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
                exit_event!(event, control_flow);
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
                    Err(wgpu::SurfaceError::Lost) => engine.resize(engine.window_state.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => 
            {
                engine.window().request_redraw();
            }
            _ => {}
        }

    );
    }

    fn window_input(&mut self, event: &WindowEvent) -> bool
    { 
        match event 
        {
            WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Tab),
                        ..
                    },
                    ..
            } => 
            {
                if self.mouse_locked == false
                {
                    self.window_state.window.set_cursor_grab(CursorGrabMode::Confined).or_else(|_| 
                    self.window_state.window.set_cursor_grab(CursorGrabMode::Locked)).unwrap();
                    self.window_state.window.set_cursor_visible(false);
                    self.mouse_locked = true; 
                }
                else
                {
                    self.window_state.window.set_cursor_grab(CursorGrabMode::None).unwrap();
                    self.window_state.window.set_cursor_visible(true);
                    self.mouse_locked = false;
                }
                true
            }
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => 
            { 
                self.camera.controller.process_keyboard(*key, *state);
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.controller.process_scroll(delta);
                true
            }
            _ => false,
        }
    }
    fn device_input(&mut self, event : &DeviceEvent) -> bool
    { 
        match event
        {
            DeviceEvent::MouseMotion{ delta, } if self.mouse_locked == true => 
            {
                self.camera.controller.process_mouse(delta.0, delta.1);
                true
            }
            _ => false,
        }
    }


    fn update(&mut self, dt: instant::Duration, time: instant::Instant)
    { 
        self.camera.update_orbit(dt);
        // self.camera.update_fps(dt);
        self.camera.update_view_proj();
    }
}




// step 1 define the buffers

// define_vertex_buffer!(
//     VertexBuffer0,
//     (position, wgpu::VertexFormat::Float32x3, 0)
// );
//
// define_vertex_buffer!(
//     VertexBuffer1,
//     (position, wgpu::VertexFormat::Float32x3, 0),
//     (uv, wgpu::VertexFormat::Float32x2, 1)
// );
//
// define_vertex_buffer!(
//     VertexBuffer2,
//     (position, wgpu::VertexFormat::Float32x3, 0),
//     (uv, wgpu::VertexFormat::Float32x2, 1),
//     (normal, wgpu::VertexFormat::Float32x3, 2)
// );
//
// define_instance_buffer!(
//     InstanceBuffer,
//     (model, [[f32; 4]; 4], wgpu::VertexFormat::Float32x4, 5)
// );


// step 2 make buffers from obj files or whatever data
// let vertices = VertexBuffer2 {}

// let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Vertex Buffer", file_name)),
    // contents: bytemuck::cast_slice(&vertices),
//     usage: wgpu::BufferUsages::VERTEX,
// });
// let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Index Buffer", file_name)),
//     contents: bytemuck::cast_slice(&m.mesh.indices),
//     usage: wgpu::BufferUsages::INDEX,
// });
