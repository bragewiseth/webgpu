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


pub const QUADVERTS: &[[f32; 3]; 4] = &[ [-1.0, -1.0, 0.0], [-1.0,  1.0, 0.0], [ 1.0, -1.0, 0.0], [ 1.0,  1.0, 0.0], ];
pub const QUADUV: &[[f32; 2]; 4] = &[[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]];
pub const QUADNORMALS: &[[f32; 3]; 4] = &[[0.0, 0.0, 0.0]; 4];
pub const QUADMESH_INDICES: &[u32] = &[2, 1, 0, 3, 1, 2];


/****************************************************************************************
 * UNIFORMS
 ****************************************************************************************/
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform 
{
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
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



/****************************************************************************************
 * VERTEX STUFF
 ****************************************************************************************/
// step 1 define the buffers

// define_vertex_buffer!(
//     VertexBuffer0,
//     (position, wgpu::VertexFormat::Float32x3, 0)
// );
//
// define_vertex_buffer!(
//     VertexBuffer1,
//     (position, wgpu::VertexFormat::Float32x3, 0),
//     (uv, wgpu::VertexFormat::Float32x2, 1)
// );
//
// define_vertex_buffer!(
//     VertexBuffer2,
//     (position, wgpu::VertexFormat::Float32x3, 0),
//     (uv, wgpu::VertexFormat::Float32x2, 1),
//     (normal, wgpu::VertexFormat::Float32x3, 2)
// );
//
// define_instance_buffer!(
//     InstanceBuffer,
//     (model, [[f32; 4]; 4], wgpu::VertexFormat::Float32x4, 5)
// );


// step 2 make buffers from obj files or whatever data
// let vertices = VertexBuffer2 {}

// let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Vertex Buffer", file_name)),
    // contents: bytemuck::cast_slice(&vertices),
//     usage: wgpu::BufferUsages::VERTEX,
// });
// let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//     label: Some(&format!("{:?} Index Buffer", file_name)),
//     contents: bytemuck::cast_slice(&m.mesh.indices),
//     usage: wgpu::BufferUsages::INDEX,
// });

// let meshes = models
//     .into_iter()
//     .map(|m| 
//         {
//             let pos = (0..m.mesh.positions.len() / 3)
//                 .map(|i| [
//                         m.mesh.positions[i * 3],
//                         m.mesh.positions[i * 3 + 1],
//                         m.mesh.positions[i * 3 + 2],
//                     ]
//                 );
//
//             let uv : Vec<[f32; 2]> = if m.mesh.texcoords.len() > 0 {
//                 (0..m.mesh.texcoords.len() / 2)
//                     .map(|i| [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]])
//                     .collect()
//             } else {
//                 (0..m.mesh.positions.len() / 3)
//                     .map(|_| [0.0, 0.0])
//                     .collect()
//             }; 
//
//             let normals : Vec<[f32; 3]> = if m.mesh.normals.len() > 0 {
//                 (0..m.mesh.normals.len() / 3)
//                     .map(|i| [
//                         m.mesh.normals[i * 3],
//                         m.mesh.normals[i * 3 + 1],
//                         m.mesh.normals[i * 3 + 2],
//                     ])
//                     .collect()
//             } else {
//                 (0..m.mesh.positions.len() / 3)
//                     .map(|_| [0.0, 0.0, 0.0])
//                     .collect()
//             };
//
//             let vertices = pos.zip(uv).zip(normals).map(|((pos, uv), normal)| 
//             {
//                 VertexBuffer2
//                 {
//                     position: pos,
//                     uv,
//                     normal,
//                 }
//
//             }).collect::<Vec<_>>();
//
//             let indices = m.mesh.indices.clone();

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceBuffer
{
    pub modelmatrix : [[f32;4];4]
}

pub trait VertexBufferTrait
{
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

pub fn update_instance_buffer(instance: &Instances, buffer: &mut wgpu::Buffer)
{
    let data = instance
        .instances
        .iter()
        .map(|i| i.to_buffer())
        .collect::<Vec<_>>();
    let size = (std::mem::size_of::<[[f32; 4]; 4]] * data.len()) as wgpu::BufferAddress;
    let slice = bytemuck::cast_slice(&data);
    let mut encoder = instance.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(slice, 0, buffer, 0, size);
    instance.queue.submit(std::iter::once(encoder.finish()));
}


#[macro_export]
macro_rules! define_instance_buffer {
    ($name:ident, $(($field:ident, $size:expr, $format:expr, $location:expr)),*) => {
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct $name {
            $(
                pub $field: $size,
            )*
        }

        impl VertexBufferTrait for $name {
            fn desc() -> wgpu::VertexBufferLayout<'static> {
                use std::mem;
                let mut attributes = Vec::new();
                let mut offset = 0;

                $(
                    let field_size = mem::size_of::<$size>() as wgpu::BufferAddress;
                    for i in 0..$size.len() {
                        attributes.push(wgpu::VertexAttribute {
                            offset,
                            shader_location: $location + i as u32,
                            format: $format,
                        });
                        offset += mem::size_of::<f32>() as wgpu::BufferAddress * $size[i].len();
                    }
                )*

                wgpu::VertexBufferLayout {
                    array_stride: offset,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &attributes,
                }
            }
        }
    };
}


#[macro_export]
macro_rules! define_vertex_buffer {
    ($name:ident, $(($field:ident, $size:expr, $format:expr, $location:expr)),*) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct $name {
            $(
                pub $field: $size,
            )*
        }

        impl VertexBufferTrait for $name {
            fn desc() -> wgpu::VertexBufferLayout<'static> {
                use std::mem;
                let mut attributes = Vec::new();
                let mut offset = 0;

                $(
                    let field_size = mem::size_of::<$size>() as wgpu::BufferAddress;
                    attributes.push(wgpu::VertexAttribute {
                        offset,
                        shader_location: $location,
                        format: $format,
                    });
                    offset += field_size;
                )*

                wgpu::VertexBufferLayout {
                    array_stride: offset,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &attributes,
                }
            }
        }
    };
}





/****************************************************************************************
 * BIND GROUP STUFF
 ****************************************************************************************/



pub trait BindGroupTrait
{
    fn desc(device : &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub trait PushConstantTrait
{
    fn desc() -> wgpu::PushConstantRange;
}



#[macro_export]
macro_rules! define_bind_group {
    ($name:ident, $(($binding:expr, $visibility:expr, $ty:expr, $count:expr)),*) => {
        pub struct $name;

        impl BindGroupTrait for $name {
            fn desc(device: &wgpu::Device) -> wgpu::BindGroupLayout {
                device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        $(
                            wgpu::BindGroupLayoutEntry {
                                binding: $binding,
                                visibility: $visibility,
                                ty: $ty,
                                count: $count,
                            },
                        )*
                    ],
                    label: Some(stringify!($name)),
                })
            }
        }
    };
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



/****************************************************************************************
 * PIPELINE and RENDER PASS MACROS
 ****************************************************************************************/





#[macro_export]
macro_rules! create_pipeline {
    ($device:expr, $layout:expr, $vs:expr, $fs:expr, $primitive:expr, $color_states:expr, $depth_stencil_state:expr, $multisample_state:expr) => {
        $device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&$layout),
            vertex: wgpu::VertexState {
                module: &$vs,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &$fs,
                entry_point: "main",
                targets: $color_states,
            }),
            primitive: $primitive,
            depth_stencil: Some($depth_stencil_state),
            multisample: $multisample_state,
        })
    };
}





#[macro_export]
macro_rules! create_render_pass {
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
