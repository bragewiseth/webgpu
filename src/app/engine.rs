use crate::app::world;
use crate::core::model::{
    Material,
    Instance,
};
use crate::core::texture::Texture;
use crate::core::camera::{ 
    Camera, 
    CameraController,
    Projection
};
use crate::core::renderer::{ 
    Framebuffer, 
    Draw,
    BindGroupLayouts,
    RenderPipeline,
    PipelineResources,
    PipelineBuffers,
    Resource
};

use winit::window::Window;
use winit:: event::*;
use winit::window::CursorGrabMode;




pub struct Engine
{ 
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    camera: Camera,
    layouts: BindGroupLayouts,
    pixel_pipeline : RenderPipeline,
    floor_pipeline : RenderPipeline,
    final_pipeline : RenderPipeline,
    pixelframebuffer : Framebuffer,
    world : world::World,
    mouse_locked: bool
} 



impl Engine
{
    // new {{{
    pub async fn new(window: Window) -> Self 
    { 

        // window setup {{{
        let size = window.inner_size();
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor 
            {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
                ..Default::default()
            }
        );
        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions 
            {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }
        ).await.unwrap();

        println!("Adapter: {:?}", adapter.get_info().name);
        println!("Backend: {:?}", adapter.get_info().backend);

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
            None, // Trace path
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

        // end window setup }}}

        let layouts = BindGroupLayouts
        {
            camera: Camera::desc(&device),
            material: Material::desc(&device),
        };


        let pixelframebuffer: Framebuffer;
        {
            let size = wgpu::Extent3d 
            {
                width: config.width / 4,
                height: config.height / 4,
                depth_or_array_layers: 1,
            };
            let texture = Texture::create_blank_texture(&device, size,"low-res-texture", wgpu::FilterMode::Nearest);
            let bind_group = Some(Framebuffer::make_bind_group(&device, &layouts, &texture));
            pixelframebuffer = Framebuffer
            {
                texture: Some(texture),
                depth_texture: Some(Texture::create_depth_texture(&device, 
                wgpu::Extent3d 
                {
                    width: config.width /4 ,
                    height: config.height / 4,
                    depth_or_array_layers: 1,
                },
                "depth_texture",
                wgpu::FilterMode::Nearest)),
                bind_group,
            }
        };
        let pixel_pipeline : RenderPipeline;
        let floor_pipeline : RenderPipeline;
        let final_pipeline : RenderPipeline;

        {
            let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
            let floorshader = device.create_shader_module(wgpu::include_wgsl!("shaders/floor.wgsl"));
            let finalshader = device.create_shader_module(wgpu::include_wgsl!("shaders/final.wgsl"));
            pixel_pipeline = RenderPipeline::new(
                &device, 
                &config,
                &shader,
                true,
                vec![PipelineResources::Camera , PipelineResources::Material],
                vec![PipelineBuffers::Model, PipelineBuffers::Instance ],
                &layouts,
                Some("pixel_pipeline_layout"));
            floor_pipeline = RenderPipeline::new(
                &device, 
                &config,
                &floorshader,
                false,
                vec![PipelineResources::Camera],
                vec![PipelineBuffers::VertexOnly],
                &layouts,
                Some("floor_pipeline_layout"));
            final_pipeline = RenderPipeline::new(
                &device, 
                &config,
                &finalshader,
                false,
                vec![ PipelineResources::Material],
                vec![PipelineBuffers::Model],
                &layouts,
                Some("final_pipeline_layout"));
        }



        let world = world::World::new(&device, &queue, &layouts ).await;
        let camera = Camera::new(
            cgmath::Point3::new(0.0, 0.0, 3.0),
            cgmath::Deg(0.0),
            cgmath::Deg(0.0),
            Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
            CameraController::new(5.0, 0.4),
            &device,
            &layouts.camera,

        );


        Self
        {
            surface,
            device,
            queue,
            config,
            size,
            window,
            camera,
            layouts,
            pixel_pipeline,
            floor_pipeline,
            final_pipeline,
            pixelframebuffer,
            world,
            mouse_locked: false,
        }
    } // end new }}}


// window {{{
    pub fn window(&self) -> &Window
    { 
        &self.window
    } // end window }}}


// resize {{{
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    { 
        if new_size.width > 0 && new_size.height > 0 
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.projection.resize(new_size.width, new_size.height);
            self.pixelframebuffer.texture = Some(Texture::create_blank_texture(&self.device, 
                wgpu::Extent3d 
                {
                    width: self.config.width / 4,
                    height: self.config.height / 4,
                    depth_or_array_layers: 1,
                },
                "high-res-texture",
                wgpu::FilterMode::Nearest));

            self.pixelframebuffer.depth_texture = Some(Texture::create_depth_texture(&self.device,
                wgpu::Extent3d 
                {
                    width: self.config.width / 4,
                    height: self.config.height / 4,
                    depth_or_array_layers: 1,
                },
                "depth_texture",
                wgpu::FilterMode::Nearest));
            
            self.pixelframebuffer.bind_group = Some(Framebuffer::make_bind_group(&self.device, &self.layouts, self.pixelframebuffer.texture.as_ref().unwrap()));
        }
    } // end resize }}}


// window input {{{
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
                    self.window.set_cursor_grab(CursorGrabMode::Confined).or_else(|_| 
                        self.window.set_cursor_grab(CursorGrabMode::Locked)).unwrap();
                    self.window.set_cursor_visible(false);
                    self.mouse_locked = true; 
                }
                else
                {
                    self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
                    self.window.set_cursor_visible(true);
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
    } // end window input }}}


// device input {{{
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
    } // end device input }}}


// update {{{
    pub fn update(&mut self, dt: instant::Duration, time: instant::Instant)
    { 
        let t = time.elapsed().as_secs_f32();
        self.camera.update_camera(dt);
        self.camera.update_view_proj();
        self.queue.write_buffer(&self.camera.buffer, 0, bytemuck::cast_slice(&[self.camera.uniform]));

        self.world.sphere_instances.instances[0].position = cgmath::Vector3::new(0.0, 0.0, t * 1.0); 
        let instance_data = self.world.sphere_instances.instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        self.queue.write_buffer(&self.world.sphere_instances.buffer, 0, bytemuck::cast_slice(&instance_data));
        println!("{:?}", self.world.sphere_instances.instances[0].position);
        println!("{:?}", t);
    } // end update }}}


// render {{{
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> 
    { 

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor 
            {
                label: Some("Render Encoder"),
            }
        );
        {
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
                                load: wgpu::LoadOp::Clear( wgpu::Color { r: 1.0, g: 1.0, b: 1.0, a: 0.0, }   ),
                                // load : wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        }
                    )],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.pixelframebuffer.depth_texture.as_ref().unwrap().view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0), // 1.
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );
            render_pass.draw_pipeline_instanced(&self.pixel_pipeline, &self.world.cube, &self.world.cube_instances, 0..9, &self.camera.bind_group );
            render_pass.draw_model_instanced(&self.world.sphere, &self.world.sphere_instances, 0..3);

        }
        {
            let mut render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor 
                {
                    label: Some("Final Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment 
                        {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations 
                            {
                                // load: wgpu::LoadOp::Load,
                                load: wgpu::LoadOp::Clear( wgpu::Color { r: 0.8, g: 0.7, b: 0.9, a: 1.0, }   ),
                                store: wgpu::StoreOp::Store,
                            },
                        }
                    )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                }
            );
            render_pass.set_pipeline(&self.floor_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.camera.bind_group, &[]);
            render_pass.draw_mesh(&self.world.floor);
            render_pass.set_pipeline(&self.final_pipeline.pipeline);
            render_pass.set_bind_group(0, &self.pixelframebuffer.bind_group.as_ref().unwrap(), &[]);
            render_pass.draw_mesh(&self.world.plane);            
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    } // end render }}}
}
