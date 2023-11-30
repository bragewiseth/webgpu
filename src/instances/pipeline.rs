use crate::components::entity_instancing::InstanceRaw;
use crate::components::texture;
use crate::components::model;
use crate::components::model::Vertex;


pub fn make_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, shader: &wgpu::ShaderModule, bind_group_layouts: &[&wgpu::BindGroupLayout]) -> wgpu::RenderPipeline
{
    let render_pipeline_layout = device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts,
                    push_constant_ranges: &[],
                }
            );

    device.create_render_pipeline(
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
    )
}



pub fn make_pipeline_final(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, shader: &wgpu::ShaderModule, bind_group_layouts: &[&wgpu::BindGroupLayout]) -> wgpu::RenderPipeline
{
    let render_pipeline_layout = device.create_pipeline_layout(
                &wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts,
                    push_constant_ranges: &[],
                }
            );

    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor 
        {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState 
            {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[model::ModelVertex::desc()]
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

            depth_stencil: None,

            multisample: wgpu::MultisampleState 
            {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        }
    )
}
