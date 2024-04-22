use kaos::renderer::CameraUniform;

struct Renderer
{
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
    layouts: Layouts,
    pipelines: Pipelines,
    shaders: Shaders,
    vertex_buffers: VertexBuffers,
    uniforms: Uniforms,
}
struct Layouts
{
    camera: wgpu::BindGroupLayout,
    material: wgpu::BindGroupLayout,
    vertex0: wgpu::VertexBufferLayout<'static>,
    vertex1: wgpu::VertexBufferLayout<'static>,
    instance: wgpu::VertexBufferLayout<'static>,
    pipeline1: wgpu::PipelineLayout,
    pipeline2: wgpu::PipelineLayout,
}
struct Pipelines
{
    floor: wgpu::RenderPipeline,
    final: wgpu::RenderPipeline,
}
struct Uniforms
{
    camera: CameraBuffer,
    materials: Vec<MaterialBuffer>,
}
struct Shaders
{
    shader1: wgpu::ShaderModule,
    shader2: wgpu::ShaderModule,
}
struct VertexBuffers
{
    vertex_buffers: Vec<wgpu::Buffer>,
    instance_buffers: Vec<wgpu::Buffer>,
}
define_vertex_buffer!(
    VertexBuffer0,
    (position, wgpu::VertexFormat::Float32x3, 0),
);
define_vertex_buffer!(
    VertexBuffer1,
    (position, wgpu::VertexFormat::Float32x3, 0),
    (uv, wgpu::VertexFormat::Float32x2, 1),
    (normal, wgpu::VertexFormat::Float32x3, 2)
);
define_instance_buffer!(
    InstanceBuffer,
    (model, [[f32; 4]; 4], wgpu::VertexFormat::Float32x4, 5)
);



impl Renderer
{
    pub fn new(
        device: wgpu::Device, 
        queue: wgpu::Queue, 
        size: winit::dpi::PhysicalSize<u32>, 
        surface: wgpu::Surface,
        config: wgpu::SurfaceConfiguration,
    ) -> Self
    {
        let vertex_buffer_layout0 = VertexBuffer0::desc(device);
        let vertex_buffer_layout1 = VertexBuffer1::desc(device);
        let instance_buffer_layout = InstanceBuffer::desc(device);
        let camera_bind_group_layout = CameraUniform::desc(device);
        let material_bind_group_layout = MaterialBuffer::desc(device);
        let pipeline_layout1 = wgpu::PipelineLayoutDescriptor
        {
            label: Some("Pipeline Layout 1"),
            bind_group_layouts: &[&camera_bind_group_layout, &material_bind_group_layout],
            push_constant_ranges: &[],
        };
        let pipeline_layout2 = wgpu::PipelineLayoutDescriptor
        {
            label: Some("Pipeline Layout 2"),
            bind_group_layouts: &[&camera_bind_group_layout, &material_bind_group_layout],
            push_constant_ranges: &[],
        };
        let pipeline1 = create_render_pipeline!(device, pipeline_layout1, vertex_buffer_layout0, vertex_buffer_layout1, instance_buffer_layout, "shader1");
        let pipeline2 = create_render_pipeline!(device, pipeline_layout2, vertex_buffer_layout0, vertex_buffer_layout1, instance_buffer_layout, "shader2");
        let shader1 = wgpu::ShaderModuleDescriptor
        {
            label: Some("Shader 1"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader1.wgsl").into()),
        };
        let shader2 = wgpu::ShaderModuleDescriptor
        {
            label: Some("Shader 2"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader2.wgsl").into()),
        };

        let layouts = Layouts
        {
            camera: camera_bind_group_layout,
            material: material_bind_group_layout,
            vertex0: vertex_buffer_layout0,
            vertex1: vertex_buffer_layout1,
            instance: instance_buffer_layout,
            pipeline1: pipeline_layout1,
            pipeline2: pipeline_layout2,
        };
        let pipelines = Pipelines
        {
            floor: pipeline1,
            final: pipeline2,
        };
        let shaders = Shaders
        {
            shader1: device.create_shader_module(&shader1),
            shader2: device.create_shader_module(&shader2),
        };
        let vertex_buffers = VertexBuffers
        {
            vertex_buffers: Vec::new(),
            instance_buffers: Vec::new(),
        };
        let uniforms = Uniforms
        {
            camera: CameraBuffer::new(device),
            materials: Vec::new(),
        };

        Self
        {
            device,
            queue,
            surface,
            config,
            layouts,
            pipelines,
            shaders,
            vertex_buffers,
            uniforms,
        }

    }

    pub fn render(&mut self, scene: &Scene) -> Result<(), wgpu::SurfaceError> 
    { 
        self._update_scene(&scene);
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor 
            {
                label: Some("Render Encoder"),
            }
        );
        {
            let mut render_pass = create_render_pass!(encoder, view);
            render_pass.set_pipeline(pipeline);
            render_pass.set_blend_constant(color);
            render_pass.set_vertex_buffer(0, self.world.sphere.vertex_buffer.slice(..));
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


    fn _update_scene(&mut self, scene: &Scene)
    {
        self.uniforms.camera.update(scene.camera);
        self.uniforms.materials.clear();
        for material in &scene.materials
        {
            self.uniforms.materials.push(MaterialBuffer::new(self.device, material));
        }
    }



    fn load_assets()
    {
        let meshes = models.into_iter()
            .map(|m| 
            {
                let pos = (0..m.mesh.positions.len() / 3)
                    .map(|i|
                         [m.mesh.positions[i * 3],
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

                let normals : Vec<[f32; 3]> = if m.mesh.normals.len() > 0 
                {
                    (0..m.mesh.normals.len() / 3)
                        .map(|i| 
                            [m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2]]
                        ).collect()
                } 
                else 
                {
                    (0..m.mesh.positions.len() / 3)
                        .map(|_| [0.0, 0.0, 0.0])
                        .collect()
                };

                let vertices = pos.zip(uv).zip(normals)
                    .map(|((pos, uv), normal)| 
                    {
                        VertexBuffer2
                        {
                            position: pos,
                            uv,
                            normal,
                        }

                    }).collect::<Vec<_>>();

                let indices = m.mesh.indices.clone();
            });
    }
}




