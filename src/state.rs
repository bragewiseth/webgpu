// use crate::components::world::World;
use crate::components::model::Vertex;
use crate::instances::camera;
use crate::components::instance::{Instance, InstanceRaw}; 
// use crate::components::camera;
use crate::components::model;
use crate::components::texture;
use winit::window::Window;
use winit:: event::*;
use wgpu::util::DeviceExt;
use cgmath::prelude::*;


#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompareFunction {
    Undefined = 0,
    Never = 1,
    Less = 2,
    Equal = 3,
    LessEqual = 4,
    Greater = 5,
    NotEqual = 6,
    GreaterEqual = 7,
    Always = 8,
}





// cube
const VERTICES: &[model::ModelVertex] = model::VERTICES;
const INDICES: &[u16] = model::INDICES;





pub struct State
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,// The window must be declared after, the surface contains unsafe references to the window's resources.
    window: Window,
    render_pipeline : wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    camera : camera::FPSCamera,
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
    depth_texture: texture::Texture,
    mouse_pressed: bool,
    // world : World,
}


impl State 
{

    pub async fn new(window: Window) -> Self
    {
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
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

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor 
            {
                features: wgpu::Features::empty(),
                limits: 
                    if cfg!(target_arch = "wasm32") 
                    {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    }
                    else
                    {
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

        let diffuse_bytes = include_bytes!("../assets/img.png");
        // let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        // let diffuse_rgba = diffuse_image.to_rgba8();
        let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "../assets/img.png").unwrap(); // CHANGED!


        let texture_bind_group_layout =
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                ty: wgpu::BindingType::Texture {
                                    multisampled: false,
                                    view_dimension: wgpu::TextureViewDimension::D2,
                                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStages::FRAGMENT,
                                // This should match the filterable field of the
                                // corresponding Texture entry above.
                                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                                count: None,
                            },
                        ],
                        label: Some("texture_bind_group_layout"),
                    });


            let diffuse_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                }
            );



        let _modes = &surface_caps.present_modes;
        let camera = camera::FPSCamera::new(&config, &device);
        // let shader = device.create_shader_module(
        //     wgpu::ShaderModuleDescriptor 
        //     {
        //         label: Some("Shader"),
        //         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        //     }
        // );
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));


        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &texture_bind_group_layout,
                    &camera.camera_bind_group_layout,
                    // &time_bind_group_layout,
                ],
                push_constant_ranges: &[],
            }
        );

        let pipeline1 = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor 
            {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState 
                {
                    module: &shader,
                    entry_point: "vs_main", // 1.
                    buffers: &[model::ModelVertex::desc(), InstanceRaw::desc()], // 2.
                },
                fragment: Some(
                    wgpu::FragmentState 
                    { // 3.
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(
                            wgpu::ColorTargetState 
                            { // 4.
                                format: config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            }
                        )],
                    }
                ),

                primitive: wgpu::PrimitiveState 
                {
                    topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // 2.
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },

                depth_stencil: Some(wgpu::DepthStencilState {
                    format: texture::Texture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less, // 1.
                    stencil: wgpu::StencilState::default(), // 2.
                    bias: wgpu::DepthBiasState::default(),
                }),

                multisample: wgpu::MultisampleState 
                {
                    count: 1, // 2.
                    mask: !0, // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            }
        );



        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        // NEW!
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );

        let num_indices = INDICES.len() as u32;
        let instances = (0..3).flat_map(|z| {
            (0..3).map(move |x| {
                let position = cgmath::Vector3 { x: x as f32 * 2.0, y: 0.0, z: z as f32 * 2.0 } - cgmath::Vector3 { x: 3.0, y: 0.0, z: 3.0 };

                let rotation = if position.is_zero() {
                    // this is needed so an object at (0, 0, 0) won't get scaled to zero
                    // as Quaternions can effect scale if they're not created correctly
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let depth_texture = texture::Texture::create_depth_texture(&device, &config, "depth_texture");


        // let world = World::new(&config, &device);

        Self
        {
            window,
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline : pipeline1,
            vertex_buffer,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_texture,
            camera,
            instances,
            instance_buffer,
            depth_texture,
            mouse_pressed: false,
        }
    }












    pub fn window(&self) -> &Window
    {
        &self.window
    }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>)
    {
        if new_size.width > 0 && new_size.height > 0 
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.camera.projection.resize(new_size.width, new_size.height);
        }
    }



// impl State
    pub fn window_input(&mut self, event: &WindowEvent) -> bool
    {
        match event 
        {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera.camera_controller.process_keyboard(*key, *state),
            WindowEvent::MouseWheel { delta, .. } => {
                self.camera.camera_controller.process_scroll(delta);
                true
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }



    pub fn device_input(&mut self, event : &DeviceEvent) -> bool
    {
        match event
        {
            DeviceEvent::MouseMotion{ delta, } => if self.mouse_pressed
            {
                self.camera.camera_controller.process_mouse(delta.0, delta.1);
                true
            }
            else
            {
                false
            },
            _ => false,
        }
    }







    pub fn update(&mut self, dt: instant::Duration)
    {
        self.camera.camera_controller.update_camera(&mut self.camera.camera, dt);
        self.camera.camera_uniform.update_view_proj(&self.camera.camera, &self.camera.projection);
        self.queue.write_buffer(&self.camera.camera_buffer, 0, bytemuck::cast_slice(&[self.camera.camera_uniform]));
    }


// impl State

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
                    label: Some("Render Pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment 
                        {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations 
                            {
                                load: wgpu::LoadOp::Clear(
                                    wgpu::Color 
                                    {
                                        r: 0.1,
                                        g: 0.1,
                                        b: 0.1,
                                        a: 1.0,
                                    }   
                                    // self.color
                                ),
                                store: wgpu::StoreOp::Store,
                            },
                        }
                    )],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_texture.view,
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]); // NEW!
            render_pass.set_bind_group(1, &self.camera.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // UPDATED!
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
        }        // submit will accept anything that implements IntoIter
        //
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }


}
