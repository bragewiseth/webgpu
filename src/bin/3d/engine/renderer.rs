
struct Renderer
{
    core : Rendercore,
    floor_pipeline: wgpu::RenderPipeline,
    final_pipeline: wgpu::RenderPipeline,
    screenquad: Mesh,
    layouts: BindGroupLayouts,
    uniforms: Uniforms,
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
}




    // move render stuff here
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
}




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


    let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));
    let floorshader = device.create_shader_module(wgpu::include_wgsl!("shaders/floor.wgsl"));
    let finalshader = device.create_shader_module(wgpu::include_wgsl!("shaders/final.wgsl"));
    // let rayshader = device.create_shader_module(wgpu::include_wgsl!("shaders/raytrace.wgsl"));
    //
    //
    //


let pixel_pipeline : RenderPipelineWrapper;
let floor_pipeline : RenderPipelineWrapper;
let final_pipeline : RenderPipelineWrapper;

{

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




        let screenquad_buffer = VertexUV::new_vertex_buffer(&device, &SCREENQUAD);
        let screenquad_index_buffer = VertexUV::new_index_buffer(&device, &SCREENQUAD_INDICES);
        let screenquad = Mesh { name: "screenquad".to_string(), vertex_buffer: screenquad_buffer, index_buffer: screenquad_index_buffer, num_elements: 6 };


// let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//     layout,
//     entries: &[
//         wgpu::BindGroupEntry {
//             binding: 0,
//             resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
//         },
//         wgpu::BindGroupEntry {
//             binding: 1,
//             resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
//         },
//         wgpu::BindGroupEntry {
//             binding: 2,
//             resource: wgpu::BindingResource::Buffer(
//                 wgpu::BufferBinding {
//                     buffer: &device.create_buffer_init(
//                         &wgpu::util::BufferInitDescriptor {
//                             label: Some("Material Color Buffer"),
//                             contents: bytemuck::cast_slice(&[diffuse_color]),
//                             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//                         }
//                     ),
//                     offset: 0,
//                     size: None,
//                 }
//             ),
//         },
//     ],
//     label: None,
// });
//
//
//
//            // self.pixelframebuffer.texture = Some(Texture::create_blank_texture(&self.device, 
            //     wgpu::Extent3d 
            //     {
            //         width: state.config.width / PIXEL_SIZE,
            //         height: state.config.height / PIXEL_SIZE,
            //         depth_or_array_layers: 1,
            //     },
            //     "high-res-texture",
            //     wgpu::FilterMode::Nearest));
            //
            // self.pixelframebuffer.depth_texture = Some(Texture::create_depth_texture(&self.device,
            //     wgpu::Extent3d 
            //     {
            //         width: self.window_state.config.width / PIXEL_SIZE,
            //         height: self.window_state.config.height / PIXEL_SIZE,
            //         depth_or_array_layers: 1,
            //     },
            //     "depth_texture",
            //     wgpu::FilterMode::Nearest));
            // 
            // self.pixelframebuffer.bind_group = Some(Framebuffer::make_bind_group(&self.device, &self.layouts, self.pixelframebuffer.texture.as_ref().unwrap(), 
            //     self.pixelframebuffer.depth_texture.as_ref().unwrap()));

// pub fn make_bind_group(device : &wgpu::Device, layout : &BindGroupLayouts, texture: &Texture, depth: &Texture ) -> wgpu::BindGroup
// {
//     device.create_bind_group(&wgpu::BindGroupDescriptor {
//     layout: &layout.framebuffer,
//     entries: &[
//         wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: wgpu::BindingResource::Sampler(&texture.sampler),
//                 },
//         wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::TextureView(&texture.view),
//                 },
//         wgpu::BindGroupEntry {
//                     binding: 2,
//                     resource: wgpu::BindingResource::TextureView(&depth.view),
//                 },
//     ],
//     label: None,
//     })
// }




// let sampler = device.create_sampler(
//     &wgpu::SamplerDescriptor {
//         address_mode_u: wgpu::AddressMode::ClampToEdge,
//         address_mode_v: wgpu::AddressMode::ClampToEdge,
//         address_mode_w: wgpu::AddressMode::ClampToEdge,
//         mag_filter: wgpu::FilterMode::Linear,
//         min_filter: wgpu::FilterMode::Linear, // 1.
//         mipmap_filter: wgpu::FilterMode::Nearest,
//         ..Default::default()
//     }
// );
