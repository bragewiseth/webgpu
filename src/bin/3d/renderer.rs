struct Renderer
{
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    config: wgpu::SurfaceConfiguration,
}

struct Pipelines
{
    floor: wgpu::RenderPipeline,
    final: wgpu::RenderPipeline,
}

struct BindGroupLayouts
{
    camera: wgpu::BindGroupLayout,
    material: wgpu::BindGroupLayout,
}

struct BindGroups
{
    camera: wgpu::BindGroup,
    material: wgpu::BindGroup,
}

struct Shaders
{
    shader1: wgpu::ShaderModule,
    shader2: wgpu::ShaderModule,
}

struct Buffers
{
    buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}


struct BufferDescriptors
{

}



impl Renderer
{
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, size: winit::dpi::PhysicalSize<u32>, surface: wgpu::Surface) -> Self
    {
        
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

        define_uniform_buffer!(
            UniformBuffer,
            (view_proj, [[f32; 4]; 4], wgpu::VertexFormat::Float32x4, 0)
        );





        Self { device, queue, surface, config}
    }



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
            }
        }
}




