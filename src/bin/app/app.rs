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
use fstop::core::renderer::{ 
    Framebuffer, 
    Draw,
    BindGroupLayouts,
    RenderPipelineWrapper,
    PipelineResources,
    PipelineBuffers,
    Resource, VertexUV,
    SCREENQUAD,
    SCREENQUAD_INDICES,
    VertexBuffer,
};

use winit::window::Window;
use winit::event::*;
use winit::window::CursorGrabMode;


const PIXEL_SIZE : u32 = 1;


pub struct Engine
{ 
    pub window_state: WindowState,
    device: wgpu::Device,
    queue: wgpu::Queue,
    camera: Camera,
    layouts: BindGroupLayouts,
    pixel_pipeline : RenderPipelineWrapper,
    floor_pipeline : RenderPipelineWrapper,
    final_pipeline : RenderPipelineWrapper,
    pixelframebuffer : Framebuffer,
    world : world::World,
    screenquad : Mesh,
    mouse_locked: bool
} 



impl Engine
{
    pub async fn new(window_state:WindowState, device:wgpu::Device, queue:wgpu::Queue) -> Self 
    { 
        let config = &window_state.config; 

        let layouts = BindGroupLayouts
        {
            camera: Camera::desc(&device),
            material: Material::desc(&device),
            framebuffer: Framebuffer::desc(&device),
        };


        let pixelframebuffer: Framebuffer;
        {
            let size = wgpu::Extent3d 
            {
                width: config.width / PIXEL_SIZE,
                height: config.height / PIXEL_SIZE,
                depth_or_array_layers: 1,
            };
            let texture = Texture::create_blank_texture(&device, size,"low-res-texture", wgpu::FilterMode::Nearest);
            let depth_texture = Texture::create_depth_texture(&device, size, "depth_texture", wgpu::FilterMode::Nearest);
            let bind_group = Some(Framebuffer::make_bind_group(&device, &layouts,&texture,  &depth_texture));
            pixelframebuffer = Framebuffer
            {
                texture: Some(texture),
                depth_texture: Some(depth_texture),
                bind_group,
            }
        };
        let pixel_pipeline : RenderPipelineWrapper;
        let floor_pipeline : RenderPipelineWrapper;
        let final_pipeline : RenderPipelineWrapper;

        {
            let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
            let floorshader = device.create_shader_module(wgpu::include_wgsl!("shaders/floor.wgsl"));
            let finalshader = device.create_shader_module(wgpu::include_wgsl!("shaders/final.wgsl"));
            // let rayshader = device.create_shader_module(wgpu::include_wgsl!("shaders/raytrace.wgsl"));
            pixel_pipeline = RenderPipelineWrapper::new(
                &device, 
                &config,
                &shader,
                // &rayshader,
                true,
                vec![PipelineResources::Camera , PipelineResources::Material],
                vec![PipelineBuffers::Model, PipelineBuffers::Instance ],
                // vec![PipelineBuffers::VertexUV],
                &layouts,
                Some("pixel_pipeline_layout"));
            floor_pipeline = RenderPipelineWrapper::new(
                &device, 
                &config,
                &floorshader,
                false,
                vec![PipelineResources::Camera],
                vec![PipelineBuffers::Model],
                &layouts,
                Some("floor_pipeline_layout"));
            final_pipeline = RenderPipelineWrapper::new(
                &device, 
                &config,
                &finalshader,
                false,
                vec![PipelineResources::Framebuffer],
                vec![PipelineBuffers::VertexUV],
                &layouts,
                Some("final_pipeline_layout"));
        }



        let world = world::World::new(&device, &queue, &layouts ).await;
        let camera = Camera::new(
            cgmath::Point3::new(0.0, -10.0, 0.0),
            cgmath::Deg(0.0),
            cgmath::Deg(0.0),
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
            CameraController::new(10.0, 0.2),
            &device,
            &layouts.camera,

        );

        let screenquad_buffer = VertexUV::new_vertex_buffer(&device, &SCREENQUAD);
        let screenquad_index_buffer = VertexUV::new_index_buffer(&device, &SCREENQUAD_INDICES);
        let screenquad = Mesh { name: "screenquad".to_string(), vertex_buffer: screenquad_buffer, index_buffer: screenquad_index_buffer, num_elements: 6 };

        Self
        {
            window_state,
            device,
            queue,
            camera,
            layouts,
            pixel_pipeline,
            floor_pipeline,
            final_pipeline,
            pixelframebuffer,
            world,
            screenquad,
            mouse_locked: false,
        }
    }


    pub fn window(&self) -> &Window
    { 
        &self.window_state.window
    }


    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    { 
        if new_size.width > 0 && new_size.height > 0 
        {
            let state = &mut self.window_state;
            state.size = new_size;
            state.config.width = new_size.width;
            state.config.height = new_size.height;
            state.surface.configure(&self.device, &state.config);
            self.camera.projection.resize(new_size.width, new_size.height);
            self.pixelframebuffer.texture = Some(Texture::create_blank_texture(&self.device, 
                wgpu::Extent3d 
                {
                    width: state.config.width / PIXEL_SIZE,
                    height: state.config.height / PIXEL_SIZE,
                    depth_or_array_layers: 1,
                },
                "high-res-texture",
                wgpu::FilterMode::Nearest));

            self.pixelframebuffer.depth_texture = Some(Texture::create_depth_texture(&self.device,
                wgpu::Extent3d 
                {
                    width: self.window_state.config.width / PIXEL_SIZE,
                    height: self.window_state.config.height / PIXEL_SIZE,
                    depth_or_array_layers: 1,
                },
                "depth_texture",
                wgpu::FilterMode::Nearest));
            
            self.pixelframebuffer.bind_group = Some(Framebuffer::make_bind_group(&self.device, &self.layouts, self.pixelframebuffer.texture.as_ref().unwrap(), 
                self.pixelframebuffer.depth_texture.as_ref().unwrap()));
        }
    }


    pub fn window_input(&mut self, event: &WindowEvent) -> bool
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
                // self.window.set_title(&format!("{:?}", self.camera.position));
                true
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.controller.process_scroll(delta);
                // self.window.set_title(&format!("{:?}", self.camera.position));
                true
            }
            _ => false,
        }
    }


    pub fn device_input(&mut self, event : &DeviceEvent) -> bool
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


    pub fn update(&mut self, dt: instant::Duration, time: instant::Instant)
    { 
        self.camera.update_orbit(dt);
        // self.camera.update_fps(dt);
        self.camera.update_view_proj();
        self.queue.write_buffer(&self.camera.buffer, 0, bytemuck::cast_slice(&[self.camera.uniform]));

        // self.world.sphere_instances.instances[0].position = cgmath::Vector3::new(0.0, 0.0, t * 1.0); 
        // let instance_data = self.world.sphere_instances.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        // self.queue.write_buffer(&self.world.sphere_instances.buffer, 0, bytemuck::cast_slice(&instance_data));
    }


    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> 
    { 
        let output = self.window_state.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor 
            {
                label: Some("Render Encoder"),
            }
        );
        {
            // let mut render_pass = create_render_pass!(encoder, &self.pixelframebuffer);
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor 
                {
                    label: Some("Pixel Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment 
                        {
                            view: &self.pixelframebuffer.texture.as_ref().unwrap().view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear( wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0, }   ),
                                // load : wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        }
                    )],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.pixelframebuffer.depth_texture.as_ref().unwrap().view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );
            // render_pass.draw_pipeline_instanced(&self.pixel_pipeline, &self.world.cube, &self.world.mats, &self.world.cube_instances, 0..9, &self.camera.bind_group );
            // render_pass.draw_model_instanced(&self.world.sphere, &self.world.sphere_instances, 0..3);
            render_pass.set_pipeline_and_bindgroups(&self.pixel_pipeline, &self.world.mats[0],&self.camera.bind_group );
            render_pass.draw_mesh_instanced( &self.world.sphere.meshes[0], &self.world.sphere_instances, 0..3,  )
            // render_pass.draw_pipeline_instanced(&self.pixel_pipeline, &self.world.plane, &self.world.plane_instances, 0..1, &self.camera.bind_group );
            // render_pass.set_pipeline(&self.pixel_pipeline.pipeline);
            // render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            // render_pass.set_bind_group(1, &self.world.mats[0].bind_group, &[]);
            // render_pass.draw_mesh(&self.screenquad);
        }
        {
            let mut render_pass = create_render_pass!(encoder, view);
            render_pass.set_pipeline(&self.floor_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            render_pass.draw_mesh(&self.world.floor);
            render_pass.set_pipeline(&self.final_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.pixelframebuffer.bind_group.as_ref().unwrap(), &[]);
            render_pass.draw_mesh(&self.screenquad);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
