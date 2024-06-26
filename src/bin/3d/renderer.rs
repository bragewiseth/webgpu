use wgpu::*;
use wgpu::util::*;
use cgmath::SquareMatrix;
use crate::kaos::*;
use crate::kaos::core::renderer::*;
use crate::kaos::core::texture::*;
use crate::scene::Scene;



pub struct Renderer<'a>
{
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    config: SurfaceConfiguration,
    layout: Layouts,
    buffers: Buffers,
    bindgroups: BindGroups,
    textures: Textures,    
    pipeline0: RenderPipeline,
    pipeline1: RenderPipeline,
    shader0: ShaderModule,    
    shader1: ShaderModule
}
struct Layouts
{    
    camera: BindGroupLayout,
    material: BindGroupLayout,
    vertex0: VertexBufferLayout<'static>,
    vertex1: VertexBufferLayout<'static>,
    instance: VertexBufferLayout<'static>,
    pipeline: PipelineLayout,

}
struct Buffers
{
    vertex: Vec<Buffer>,
    instance: Vec<Buffer>,
    index: Vec<Buffer>,
    camera: Buffer,
    player: Buffer,
    materials: Vec<Buffer>
}
struct BindGroups
{
    camera: BindGroup,
    player: BindGroup,
    materials: Vec<BindGroup>
}
struct Textures
{
    textures: Vec<Texture>,
    sampler0: Sampler,
}




define_vertex_buffer!(
    VertexBuffer0,
    (position, [f32;3], VertexFormat::Float32x3, 0)
);
define_vertex_buffer!(
    VertexBuffer1,
    (position, [f32;3], VertexFormat::Float32x3, 0),
    (uv, [f32;2], VertexFormat::Float32x2, 1),
    (normal, [f32;3], VertexFormat::Float32x3, 2)
);
define_instance_buffer!(
    InstanceBuffer,
    (model, [[f32; 4]; 4], VertexFormat::Float32x4, 3, 4)
);

impl Layouts
{
    pub fn new(device:&Device) -> Self
    {
        let vertex0 = VertexBuffer0::desc();
        let vertex1 = VertexBuffer1::desc();
        let instance = InstanceBuffer::desc();
        let camera = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor
            {
                label: Some("Camera Bind Group Layout"),
                entries: &[uniform_bindgroup_layout_entry(0, ShaderStages::VERTEX)],
            }
        );
        let material = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor
            {
                label: Some("Material Bind Group Layout"),
                entries: &[
                    sampler_bindgroup_layout_entry(0, SamplerBindingType::Filtering, ShaderStages::FRAGMENT),
                    texture_bindgroup_layout_entry(1, ShaderStages::FRAGMENT),
                    uniform_bindgroup_layout_entry(2, ShaderStages::FRAGMENT),
                ],
            }
        );
        let pipeline = device.create_pipeline_layout(
            &PipelineLayoutDescriptor
            {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[&camera, &material],
                push_constant_ranges: &[],
            }
        );

        Self
        {
            camera,
            material,
            vertex0,
            vertex1,
            instance,
            pipeline,
        }
    }
}



impl Buffers
{
    fn new(device: &Device) -> Self
    {
        let vertex = vec![];
        let instance = vec![];
        let index = vec![];
        let materials = vec![];
        let camera_uniform = CameraUniform::new();
        let camera = device.create_buffer_init(
            &BufferInitDescriptor
            {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera_uniform]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            }
        );
        let player_uniform = PlayerUniform{ model : cgmath::Matrix4::identity().into() };
        let player = device.create_buffer_init(
            &BufferInitDescriptor
            {
                label: Some("Player Buffer"),
                contents: bytemuck::cast_slice(&[player_uniform]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            }
        );
        Self
        {
            vertex,
            index,
            instance,
            camera,
            player,
            materials,
        }
    }
}


impl Textures
{
    fn new(device: &Device ) -> Self
    {
        let sampler0 = device.create_sampler(
            &SamplerDescriptor
            {
                label: Some("Sampler 0"),
                address_mode_u: AddressMode::ClampToEdge,
                address_mode_v: AddressMode::ClampToEdge,
                address_mode_w: AddressMode::ClampToEdge,
                mag_filter: FilterMode::Linear,
                min_filter: FilterMode::Linear,
                mipmap_filter: FilterMode::Linear,
                lod_min_clamp: -100.0,
                lod_max_clamp: 100.0,
                compare: None,
                anisotropy_clamp: 1,
                border_color: None,
            }
        );
        let z_buffer =  create_depth_texture(device, Extent3d
            {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            });
        let textures = vec![z_buffer];
        Self
        {
            textures,
            sampler0,
        }
    }
}


impl BindGroups
{
    fn new(device: &Device, buffers: &Buffers, textures: &Textures, layouts:&Layouts) -> Self
    {
        let camera = device.create_bind_group(
            &BindGroupDescriptor
            {
                layout: &layouts.camera,
                entries: &[uniform_bindgroup_entry(0, buffers.camera.as_entire_binding())],
                label: Some("Camera Bind Group"),
            }
        );
        let player = device.create_bind_group(
            &BindGroupDescriptor
            {
                layout: &layouts.camera,
                entries: &[uniform_bindgroup_entry(0, buffers.player.as_entire_binding())],
                label: Some("Player Bind Group"),
            }
        );
        let materials = vec![];
        Self
        {
            camera,
            player,
            materials,
        }
    }
}





impl Renderer<'_>
{
    pub fn new(
        device: Device, 
        queue: Queue, 
        size: winit::dpi::PhysicalSize<u32>, 
        surface: Surface,
        config: SurfaceConfiguration,
    ) -> Self
    {        
        let layout = Layouts::new(&device);
        let shader0 = device.create_shader_module(wgpu::include_wgsl!("shaders/floor.wgsl"));
        let shader1 = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        let pipeline0 = device.create_render_pipeline(
            &RenderPipelineDescriptor
            {
                label: Some("Render Pipeline 1"),
                layout: Some(&layout.pipeline),
                vertex: VertexState
                {
                    module: &shader0,
                    entry_point: "vs_main",
                    buffers: &[layout.vertex0, layout.instance],
                },
                fragment: Some(FragmentState
                {
                    module: &shader0,
                    entry_point: "fs_main",
                    targets: &[Some(alphablend_color_target_state(config.format))]
                }),
                primitive: primitive_state(PrimitiveTopology::TriangleList, PolygonMode::Fill),
                depth_stencil: Some(depth_stencil_state(TextureFormat::Depth24Plus, true, CompareFunction::Less)),
                multisample: multisample_state(1),
                multiview: None,
            }
        ); 
        let pipeline1 = device.create_render_pipeline(
            &RenderPipelineDescriptor
            {
                label: Some("Render Pipeline 2"),
                layout: Some(&layout.pipeline),
                vertex: VertexState
                {
                    module: &shader1,
                    entry_point: "vs_main",
                    buffers: &[layout.vertex1, layout.instance],
                },
                fragment: Some(FragmentState
                {
                    module: &shader1,
                    entry_point: "fs_main",
                    targets: &[Some(alphablend_color_target_state(config.format))]
                }),
                primitive: primitive_state(PrimitiveTopology::TriangleList, PolygonMode::Fill),
                depth_stencil: Some(depth_stencil_state(TextureFormat::Depth24Plus, true, CompareFunction::Less)),
                multisample: multisample_state(1),
                multiview: None,
            }
        );
        let buffers = Buffers::new(&device);
        let textures = Textures::new(&device);
        let bindgroups = BindGroups::new(&device, &buffers, &textures, &layout);
        Self
        {
            device,
            queue,
            surface,
            config,
            layout,
            buffers,
            bindgroups,
            textures,
            pipeline0,
            pipeline1,
            shader0,
            shader1
        }
    }
    /**************************************************************************************************
     * RENDER FUNCTION
     * ***********************************************************************************************/
    pub fn render(&mut self, scene: &Scene, dt: f32) -> Result<(), SurfaceError> 
    { 
        self._update_scene(&scene , dt);
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let z_buffer = self.textures.textures[0].create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &CommandEncoderDescriptor 
            {
                label: Some("Render Encoder"),
            }
        );
        {
            let mut render_pass = create_render_pass!(encoder, view, z_buffer);
            render_pass.set_pipeline(&self.pipeline0);
            render_pass.set_vertex_buffer(0, self.buffers.vertex[0].slice(..));
            render_pass.set_vertex_buffer(1, self.buffers.instance[0].slice(..));
            render_pass.set_index_buffer(self.buffers.index[0].slice(..), IndexFormat::Uint32);
            render_pass.set_bind_group(0, &self.bindgroups.camera, &[]);
            render_pass.set_bind_group(1, &self.bindgroups.materials[0], &[]);
            render_pass.draw_indexed(0..6, 0, 0..1);

            render_pass.set_pipeline(&self.pipeline1);
            render_pass.set_vertex_buffer(0, self.buffers.vertex[1].slice(..));
            render_pass.set_vertex_buffer(1, self.buffers.instance[1].slice(..));
            render_pass.set_index_buffer(self.buffers.index[1].slice(..), IndexFormat::Uint32);
            render_pass.set_bind_group(0, &self.bindgroups.camera, &[]);
            render_pass.set_bind_group(1, &self.bindgroups.materials[1], &[]);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
    /**************************************************************************************************
     * UPDATE SCENE FUNCTION
     * ***********************************************************************************************/
    fn _update_scene(&mut self, scene: &Scene, dt: f32)
    {
        let camera_uniform = CameraUniform::new_with(&scene.camera);
        self.queue.write_buffer(&self.buffers.camera, 0, bytemuck::cast_slice(&[camera_uniform]));
        self.queue.write_buffer(&self.buffers.player, 0, bytemuck::cast_slice(&[scene.player.calc_matrix()]));
    }
    /**************************************************************************************************
     * RESIZE FUNCTION
     * ***********************************************************************************************/
    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>)
    {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.textures.textures[0] = create_depth_texture(&self.device, Extent3d
            {
                width: size.width,
                height: size.height,
                depth_or_array_layers: 1,
            });
    }
    /**************************************************************************************************
     * LOAD ASSETS FUNCTION
     * ***********************************************************************************************/
    pub async fn load_assets(&mut self, scene: &Scene)
    {
        let (models, materials) = &scene.resources;
        for m in materials
        {
            let diffuse_texture = load_texture(&m.diffuse_texture, &self.device, &self.queue).await.unwrap();
            let material_uniform = MaterialUniform::new_with(m.diffuse, m.dissolve, m.shininess);
            let material_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Material Buffer"),
                contents: bytemuck::cast_slice(&[material_uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
            let texture_view = diffuse_texture.create_view(&TextureViewDescriptor::default());
            let bindgroup = self.device.create_bind_group(
                &BindGroupDescriptor
                {
                    layout: &self.layout.material,
                    entries: &[
                        sampler_bindgroup_entry(0, &self.textures.sampler0),
                        texture_bindgroup_entry(1, &texture_view),
                        uniform_bindgroup_entry(2, material_buffer.as_entire_binding()),
                    ],
                    label: Some("Material Bind Group"),
                }
            );
            self.textures.textures.push(diffuse_texture);
            self.buffers.materials.push(material_buffer);
            self.bindgroups.materials.push(bindgroup);
        }
        models.into_iter()
            .map(|m| 
            {
                let pos = (0..m.mesh.positions.len() / 3)
                    .map(|i|
                         [ m.mesh.positions[i * 3],
                           m.mesh.positions[i * 3 + 1],
                           m.mesh.positions[i * 3 + 2]]
                    );
                let uv : Vec<[f32; 2]> = if m.mesh.texcoords.len() > 0 
                {
                    (0..m.mesh.texcoords.len() / 2)
                        .map(|i| [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]])
                        .collect()
                } 
                else 
                {
                    (0..m.mesh.positions.len() / 3)
                        .map(|_| [0.0, 0.0])
                        .collect()
                }; 
                let normals : Vec<[f32; 3]> = if m.mesh.normals.len() > 0 {
                    (0..m.mesh.normals.len() / 3)
                        .map(|i| [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ])
                        .collect()
                }
                else
                {
                    (0..m.mesh.positions.len() / 3)
                        .map(|_| [0.0, 0.0, 0.0])
                        .collect()
                };
                let vertices = pos.zip(uv).zip(normals)
                    .map(|((pos, uv), normal)| 
                        VertexBuffer1
                        {
                            position: pos,
                            uv,
                            normal,
                        }).collect::<Vec<_>>();
                let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });
                self.buffers.vertex.push(vertex_buffer);
                self.buffers.index.push(index_buffer);

                
            });
        let instances = scene.instances;
        for i in instances
        {
            let instance_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&[i.calc_matrix()]),
                usage: wgpu::BufferUsages::VERTEX,
            });
            self.buffers.instance.push(instance_buffer);
        }
    }
}

