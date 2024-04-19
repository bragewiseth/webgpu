/**
 this module is responsible for rendering the scene
 it contains all the necessary bindings and structures that the graphics pipeline needs
 having all the gpu related stuff in one place makes it easier to see what the pipeline is doing and its dependencies
 if i were to build a render pass i need to know what resources it needs and what buffers it needs to bind therefore
 having all the necessary bindings in one place makes it easier to see what the render pass needs
 and what it needs to bind to the gpu
*/

use crate::core::camera::Camera;
use cgmath::prelude::*;
use crate::core::texture::Texture;
use crate::core::model::{ Material,  Instances, Mesh };

use std::ops::Range;
use wgpu::util::DeviceExt;


pub const QUADMESH: [Vertex; 4] = [
    Vertex { position: [-1.0, -1.0, 0.0], uv: [0.0, 1.0], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [-1.0,  1.0, 0.0], uv: [0.0, 0.0], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ 1.0, -1.0, 0.0], uv: [1.0, 1.0], normal: [0.0, 0.0, 0.0] },
    Vertex { position: [ 1.0,  1.0, 0.0], uv: [1.0, 0.0], normal: [0.0, 0.0, 0.0] },
];
pub const QUADMESH_INDICES: &[u32] = &[2, 1, 0, 3, 1, 2];





#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform 
{
    view: [[f32; 4]; 4],// can't use cgmath with bytemuck directly so we'll have
    proj: [[f32; 4]; 4],// to convert the Matrix4 into a 4x4 f32 array
    view_position: [f32; 4],
}
impl CameraUniform 
{
    pub fn new() -> Self
    {
        Self 
        {
            view: cgmath::Matrix4::identity().into(),
            proj: cgmath::Matrix4::identity().into(),
            view_position: [0.0; 4]
        }
    }
}
pub fn update_view_proj(camera: &mut Camera, uniform: &mut CameraUniform) 
{
    uniform.view_position = camera.position.to_homogeneous().into();
    uniform.proj = camera.projection.calc_matrix().into();
    uniform.view = camera.calc_matrix().into();
}





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



pub trait VertexBuffer
{
    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        use std::mem;
        wgpu::VertexBufferLayout 
        {
            array_stride: mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute 
                {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
    fn new_vertex_buffer (device : &wgpu::Device, vertices : &[Self]) -> wgpu::Buffer
    where Self : Sized + bytemuck::Pod
    {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor 
            {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        )
    }
    fn new_index_buffer (device : &wgpu::Device, indices : &[u32]) -> wgpu::Buffer
    where Self : Sized + bytemuck::Pod
    {
        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor
            {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        )
    }
}

// let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Vertex Buffer", file_name)),
//     contents: bytemuck::cast_slice(&vertices),
//     usage: wgpu::BufferUsages::VERTEX,
// });
// let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Index Buffer", file_name)),
//     contents: bytemuck::cast_slice(&m.mesh.indices),
//     usage: wgpu::BufferUsages::INDEX,
// });




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





#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceArray
{
    pub model: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexArray
{
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}




impl VertexBuffer for InstanceArray {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceArray>() as wgpu::BufferAddress,
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



impl VertexBuffer for ModelVertex {
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
            ],
        }
    }
}


pub trait Resource
{
    fn desc(device : &wgpu::Device) -> wgpu::BindGroupLayout;
}


pub trait PushConstant 
{
    fn desc() -> wgpu::PushConstantRange;
}



impl Resource for Camera
{
    fn desc( device : &wgpu::Device ) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor 
            {
                entries: &[
                    wgpu::BindGroupLayoutEntry 
                    {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX ,
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
            }
        )
    }
}



impl Resource for Material
{
    fn desc( device : &wgpu::Device ) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
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

                wgpu::BindGroupLayoutEntry 
                {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer 
                    {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("material_bind_group_layout"),
        })
    }
}



impl Resource for Framebuffer
{
    fn desc( device : &wgpu::Device ) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
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
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Depth,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            label: Some("framebuffer_bind_group_layout"),
        })
    }
}



// enum PipelineType
// {
//     Forward,
//     Deferred,
//     Shadow,
//     PostProcess,
// }
//
pub struct BindGroupLayouts
{
    pub camera: wgpu::BindGroupLayout,
    pub material: wgpu::BindGroupLayout,
    pub framebuffer: wgpu::BindGroupLayout,
}

pub enum PipelineResources
{
    Camera,
    Material,
    Framebuffer,
}

pub enum PipelineBuffers
{
    Model,
    Instance,
    VertexOnly,
    VertexUV
}


pub struct RenderPipelineWrapper
{
    pub pipeline: wgpu::RenderPipeline,
    pub resources: Vec<PipelineResources>,
    pub vertex_buffers: Vec<PipelineBuffers>,
}




impl RenderPipelineWrapper
{
    pub fn new(
        device : &wgpu::Device, 
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
        depth_stencil: bool,
        resources : Vec<PipelineResources>,
        vertex_buffers : Vec<PipelineBuffers>,
        layouts : &BindGroupLayouts,
        label: Option<&str>) -> Self
    {

        let bind_group_layouts : Vec<&wgpu::BindGroupLayout> = resources.iter().map(|x| 
        {
            match x
            {
                PipelineResources::Camera =>        { &layouts.camera },
                PipelineResources::Material =>      { &layouts.material },
                PipelineResources::Framebuffer =>   { &layouts.framebuffer },
            }
        }).collect();
        let buffers : Vec<wgpu::VertexBufferLayout<'static>> = vertex_buffers.iter().map(|x| 
        {
            match x
            {
                PipelineBuffers::Model =>       { ModelVertex::desc() },
                PipelineBuffers::Instance =>    { InstanceRaw::desc() },
                PipelineBuffers::VertexOnly =>  { VertexOnly::desc() },
                PipelineBuffers::VertexUV =>    { VertexUV::desc() },
            }
        }).collect();

        let layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor 
            {
                label,
                bind_group_layouts: &bind_group_layouts,
                push_constant_ranges: &[],
            }
        );
        let ds : Option<wgpu::DepthStencilState>;
        if depth_stencil == true
        { 
            ds =  Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            })
        }
        else
        {
            ds = None;
        }

        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor 
            {
                label,
                layout: Some(&layout),
                vertex: wgpu::VertexState 
                {
                    module: shader,
                    entry_point: "vs_main",
                    buffers: &buffers,
                },
                fragment: Some(
                    wgpu::FragmentState 
                    {
                        module: &shader,
                        entry_point: "fs_main",
                        targets: &[Some(
                            wgpu::ColorTargetState 
                            {
                                format: config.format,
                                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                                write_mask: wgpu::ColorWrites::ALL
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
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: ds,
                multisample: wgpu::MultisampleState 
                {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );
        Self { pipeline, resources, vertex_buffers }
    }
}


#[macro_export]
macro_rules! create_render_pass {
    // Plain render pass with no parameters
    ($encoder:expr, $view:expr ) => {
        $encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Simple Pass"),
                color_attachments: &[Some(
                    wgpu::RenderPassColorAttachment 
                    {
                        view: &$view,
                        resolve_target: None,
                        ops: wgpu::Operations 
                        {
                            load: wgpu::LoadOp::Clear( wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, }   ),
                            store: wgpu::StoreOp::Store,
                        },
                    }
                )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
        })
    };
    // Render pass with framebuffer parameter
    // ($encoder:expr, $framebuffer:expr) => {
    //     let depth_stencil_attachment = match &$framebuffer.depth_texture {
    //         Some(texture) => Some(wgpu::RenderPassDepthStencilAttachment {
    //             view: &texture.view,
    //             depth_ops: Some(wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(1.0),
    //                 store: wgpu::StoreOp::Store,
    //             }),
    //             stencil_ops: None,
    //         }),
    //         None => None,
    //     };
    //     $encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Buffer Pass"),
    //         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
    //             view: &$framebuffer.texture.as_ref().unwrap().view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0, }),
    //                 store: wgpu::StoreOp::Store,
    //             },
    //         })],
    //         depth_stencil_attachment: depth_stencil_attachment,
    //         timestamp_writes: None,
    //         occlusion_query_set: None,
    //     })
    // };

    ($encoder:expr, $view:expr, $z_buffer:expr) => {
        $encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Simple Pass"),
            color_attachments: &[Some(
                wgpu::RenderPassColorAttachment 
                {
                    view: &$view,
                    resolve_target: None,
                    ops: wgpu::Operations 
                    {
                        load: wgpu::LoadOp::Clear( wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, }   ),
                        store: wgpu::StoreOp::Store,
                    },
                }
                )],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &$z_buffer.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
        })
    };
}




pub struct Framebuffer
{
    pub texture: Option<Texture>,
    pub depth_texture: Option<Texture>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Framebuffer
{
    pub fn make_bind_group(device : &wgpu::Device, layout : &BindGroupLayouts, texture: &Texture, depth: &Texture ) -> wgpu::BindGroup
    {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout.framebuffer,
        entries: &[
            wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
            wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
            wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::TextureView(&depth.view),
                    },
        ],
        label: None,
        })
    }
}



pub trait Draw<'a>
{

    fn set_pipeline_and_bindgroups(
        &mut self, 
        pipeline: &'a RenderPipelineWrapper, 
        materials : &'a Material,
        camera: &'a wgpu::BindGroup) -> ();

    fn draw_mesh(

        &mut self,
        mesh: &'a Mesh,
    );

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        instances: &'a Instances,
        num_instances: Range<u32>,
    );
}


impl<'a, 'b> Draw<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn set_pipeline_and_bindgroups(
            &mut self, 
            pipeline: &'b RenderPipelineWrapper, 
            material : &'b Material,
            camera: &'b wgpu::BindGroup) -> ()
    {
        self.set_pipeline(&pipeline.pipeline);
        for (i,resource) in pipeline.resources.iter().enumerate()
        {
            match resource
            {
                PipelineResources::Camera => { self.set_bind_group(i as u32, camera, &[]); },
                PipelineResources::Material => { self.set_bind_group(i as u32, &material.bind_group, &[]); },
                PipelineResources::Framebuffer => { self.set_bind_group(i as u32, &material.bind_group, &[]); },
            }
        }
    }


    fn draw_mesh(
        &mut self,
        mesh: &'b Mesh,) 
    {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_elements, 0, 0..1);
    }

    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        instances: &'b Instances,
        num_instances: Range<u32>,) 
    {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instances.buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..mesh.num_elements, 0, num_instances);
    }
}

// end RENDERPASS }}}






// STUFF THAT COULD POTEINTIALLY BE USED LATER {{{



// #[repr(C)]
// #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// pub enum CompareFunction 
// {
//     Undefined = 0,
//     Never = 1,
//     Less = 2,
//     Equal = 3,
//     LessEqual = 4,
//     Greater = 5,
//     NotEqual = 6,
//     GreaterEqual = 7,
//     Always = 8,
// }
// }}}
