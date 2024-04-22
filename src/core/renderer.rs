/**
 this module is responsible for rendering the scene
 it contains all the necessary bindings and structures that the graphics pipeline needs
 having all the gpu related stuff in one place makes it easier to see what the pipeline is doing and its dependencies
 if i were to build a render pass i need to know what resources it needs and what buffers it needs to bind therefore
 having all the necessary bindings in one place makes it easier to see what the render pass needs
 and what it needs to bind to the gpu
*/
use crate::camera::Camera;
use cgmath::prelude::*;

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
pub fn update_camera_uniform(camera: &Camera, uniform: &mut CameraUniform) 
{
    uniform.view_position = camera.position.to_homogeneous().into();
    uniform.proj = camera.projection.calc_matrix().into();
    uniform.view = camera.calc_matrix().into();
}


/****************************************************************************************
 * VERTEX STUFF
 ****************************************************************************************/



pub trait VertexBufferTrait
{
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}



#[macro_export]
macro_rules! define_instance_buffer 
{
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


#[macro_export]
macro_rules! create_vertex_buffer {
    ($device:expr, $vertices:expr) => {
        $device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice($vertices),
            usage: wgpu::BufferUsages::VERTEX,
        })
    };
}


#[macro_export]
macro_rules! create_index_buffer {
    ($device:expr, $indices:expr) => {
        $device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice($indices),
            usage: wgpu::BufferUsages::INDEX,
        })
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


#[macro_export]
macro_rules! create_bind_group {
    ($device:expr, $layout:expr, $entries:expr) => {
        $device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &$layout,
            entries: $entries,
            label: None,
        })
    };
}



/****************************************************************************************
 * PIPELINE and RENDER PASS MACROS
 ****************************************************************************************/



#[macro_export]
macro_rules! create_pipline_layout {
    ($device:expr, $bind_groups:expr, $push_constants:expr) => {
        $device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: $bind_groups,
            push_constant_ranges: $push_constants,
        })
    };
}



#[macro_export]
macro_rules! create_pipeline_descriptor {
    ($layout:expr, $vs:expr, $fs:expr, $primitive:expr, $color_states:expr, $depth_stencil_state:expr, $multisample_state:expr) => {
        wgpu::RenderPipelineDescriptor {
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
        }
    };
}



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
