// This file contains layouts for gpu input
use crate::core::camera::Camera;
use crate::core::model::Material;
use crate::core::texture;

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



pub trait Vertex 
{
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub trait Uniform 
{
    fn desc( device : &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub trait Texture 
{
    fn desc( device : &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub trait PushConstant 
{
    fn desc() -> wgpu::PushConstantRange;
}


pub trait PipelineLayout
{
    fn new(device : &wgpu::Device, bind_group_layouts : &[&wgpu::BindGroupLayout] ) -> Self;
    fn build_pipeline(&self, device: wgpu::Device, config: &wgpu::SurfaceConfiguration, 
        shader: &wgpu::ShaderModule, depth_stencil: Option<wgpu::DepthStencilState>, buffers : &[wgpu::VertexBufferLayout<'static>]) -> wgpu::RenderPipeline; 
}


pub struct Framebuffer
{
    pub texture: texture::Texture,
    pub bind_group: Option<wgpu::BindGroup>,
    pub depth_texture: Option<texture::Texture>,
}




#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32; 4]; 4],
}



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex 
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}


pub struct Layouts
{
    pub camera: wgpu::BindGroupLayout,
    pub texture: wgpu::BindGroupLayout,
    // pub light: wgpu::BindGroupLayout,
}


impl Uniform for Camera
{
    fn desc( device : &wgpu::Device ) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry 
                {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer 
                    {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("camera_bind_group_layout"),
        })
    }
}



impl Texture for Material
{
    fn desc( device : &wgpu::Device) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                }
            ],
            label: Some("texture_bind_group_layout"),
        })
    }
}








impl Vertex for InstanceRaw {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // We need to switch from using a step mode of Vertex to Instance
            // This means that our shaders will only change to use the next
            // instance when the shader starts processing a new instance
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // A mat4 takes up 4 vertex slots as it is technically 4 vec4s. We need to define a slot
                // for each vec4. We'll have to reassemble the mat4 in the shader.
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}



impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },

            ],
        }
    }
}



impl PipelineLayout for wgpu::PipelineLayout
{

    fn new(device : &wgpu::Device, bind_group_layouts : &[&wgpu::BindGroupLayout] ) -> Self
    {
        device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor 
            {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            }
        )
    }




    fn build_pipeline(
        &self, 
        device: wgpu::Device, 
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule, 
        depth_stencil: Option<wgpu::DepthStencilState>,
        buffers : &[wgpu::VertexBufferLayout<'static>]) -> wgpu::RenderPipeline
    {
        device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor 
            {
                label: Some("Render Pipeline"),
                layout: Some(&self),
                vertex: wgpu::VertexState 
                {
                    module: shader,
                    entry_point: "vs_main",
                    buffers,
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
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil,
                multisample: wgpu::MultisampleState 
                {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None, // 5.
            }
        )
    }
}





impl Layouts
{
    pub fn new(device : &wgpu::Device) -> Self
    {
        let camera = Camera::desc(device);
        let texture = Material::desc(device);
        // let light = Light::desc(device);
        Self { camera, texture }
    }
}





impl Framebuffer
{
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration, texture: texture::Texture) -> Self
    {
        // let size = winit::dpi::PhysicalSize::<u32>::from(config.extent);
        let depth_texture = texture::Texture::create_depth_texture(device, &config, "depth_texture");
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&low_res_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });
        Self { texture, depth_texture, bind_group }
    }
}
