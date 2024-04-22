use crate::camera::Camera;
use cgmath::prelude::*;
use wgpu::*;


pub const QUADVERTS: &[[f32; 3]; 4] = &[ [-1.0, -1.0, 0.0], [-1.0,  1.0, 0.0], [ 1.0, -1.0, 0.0], [ 1.0,  1.0, 0.0], ];
pub const QUADUV: &[[f32; 2]; 4] = &[[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]];
pub const QUADNORMALS: &[[f32; 3]; 4] = &[[0.0, 0.0, 0.0]; 4];
pub const QUADMESH_INDICES: &[u32] = &[2, 1, 0, 3, 1, 2];




/****************************************************************************************
 * VERTEX STUFF
 ****************************************************************************************/

pub trait VertexBufferTrait
{
    fn desc() -> VertexBufferLayout<'static>;
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
            fn desc() -> VertexBufferLayout<'static> {
                use std::mem;
                let mut attributes = Vec::new();
                let mut offset = 0;

                $(
                    let field_size = mem::size_of::<$size>() as BufferAddress;
                    for i in 0..$size.len() {
                        attributes.push(VertexAttribute {
                            offset,
                            shader_location: $location + i as u32,
                            format: $format,
                        });
                        offset += mem::size_of::<f32>() as BufferAddress * $size[i].len();
                    }
                )*

                VertexBufferLayout {
                    array_stride: offset,
                    step_mode: VertexStepMode::Instance,
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
        pub struct $name 
        {
            $(
                pub $field: $size,
            )*
        }

        impl VertexBufferTrait for $name {
            fn desc() -> VertexBufferLayout<'static> {
                use std::mem;
                let mut attributes = Vec::new();
                let mut offset = 0;

                $(
                    let field_size = mem::size_of::<$size>() as BufferAddress;
                    attributes.push(VertexAttribute {
                        offset,
                        shader_location: $location,
                        format: $format,
                    });
                    offset += field_size;
                )*

                VertexBufferLayout {
                    array_stride: offset,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &attributes,
                }
            }
        }
    };
}


#[macro_export]
macro_rules! create_vertex_buffer {
    ($device:expr, $vertices:expr) => {
        $device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice($vertices),
            usage: BufferUsages::VERTEX,
        })
    };
}


#[macro_export]
macro_rules! create_index_buffer {
    ($device:expr, $indices:expr) => {
        $device.create_buffer_init(&util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice($indices),
            usage: BufferUsages::INDEX,
        })
    };
}





/****************************************************************************************
 * BIND GROUP STUFF
 ****************************************************************************************/

pub fn texture_bindgroup_layout_entry(binding: u32, shader_stage: ShaderStages) -> BindGroupLayoutEntry 
{
    BindGroupLayoutEntry 
    {
        binding,
        visibility: shader_stage,
        ty: BindingType::Texture {
            multisampled: false,
            sample_type: TextureSampleType::Float { filterable: true },
            view_dimension: TextureViewDimension::D2,
        },
        count: None,
    }
}

pub fn sampler_bindgroup_layout_entry(
    binding: u32, 
    samplerbindingtype: SamplerBindingType, 
    shader_stage: ShaderStages
) -> BindGroupLayoutEntry 
{
    BindGroupLayoutEntry 
    {
        binding,
        visibility: shader_stage,
        ty: BindingType::Sampler(samplerbindingtype),
        count: None,
    }
}

pub fn uniform_bindgroup_layout_entry(
    binding: u32, 
    shader_stage: ShaderStages
) -> BindGroupLayoutEntry 
{
    BindGroupLayoutEntry 
    {
        binding,
        visibility: shader_stage,
        ty: BindingType::Buffer 
        {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn texture_bindgroup_entry(
    binding: u32, 
    texture: &TextureView, 
) -> BindGroupEntry 
{
    BindGroupEntry 
    {
        binding,
        resource: BindingResource::TextureView(texture),
    }
}

pub fn sampler_bindgroup_entry(
    binding: u32, 
    sampler: &Sampler, 
) -> BindGroupEntry 
{
    BindGroupEntry 
    {
        binding,
        resource: BindingResource::Sampler(sampler),
    }
}


pub fn uniform_bindgroup_entry(
    binding: u32, 
    buffer: &Buffer, 
) -> BindGroupEntry 
{
    BindGroupEntry 
    {
        binding,
        resource: BindingResource::Buffer(
            BufferBinding 
            {
                buffer,
                offset: 0,
                size: None,
            }
        ),
    }
}

/****************************************************************************************
 * USEFUL UNIFORMS
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

pub fn camera_to_uniform(camera: &Camera, uniform: &mut CameraUniform) 
{
    uniform.view_position = camera.position.to_homogeneous().into();
    uniform.proj = camera.projection.calc_matrix().into();
    uniform.view = camera.calc_matrix().into();
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniform
{
    color: [f32; 4],
    roughness: f32,
    metallic: f32,
    _padding: f32,
}


/****************************************************************************************
 * PIPELINE and RENDER PASS MACROS
 ****************************************************************************************/



#[macro_export]
macro_rules! create_pipline_layout {
    ($device:expr, $bind_groups:expr, $push_constants:expr) => {
        $device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: $bind_groups,
            push_constant_ranges: $push_constants,
        })
    };
}


#[macro_export]
macro_rules! create_pipeline_descriptor {
    ($layout:expr, $vs:expr, $fs:expr, $primitive:expr, $color_states:expr, $depth_stencil_state:expr, $multisample_state:expr) => {
        RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&$layout),
            vertex: VertexState {
                module: &$vs,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
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
        $device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&$layout),
            vertex: VertexState {
                module: &$vs,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
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
        $encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Simple Pass"),
                color_attachments: &[Some(
                    RenderPassColorAttachment 
                    {
                        view: &$view,
                        resolve_target: None,
                        ops: Operations 
                        {
                            load: LoadOp::Clear( Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, }   ),
                            store: StoreOp::Store,
                        },
                    }
                )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
        })
    };
    ($encoder:expr, $view:expr, $z_buffer:expr) => {
        $encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Simple Pass"),
            color_attachments: &[Some(
                RenderPassColorAttachment 
                {
                    view: &$view,
                    resolve_target: None,
                    ops: Operations 
                    {
                        load: LoadOp::Clear( Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0, }   ),
                        store: StoreOp::Store,
                    },
                }
                )],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &$z_buffer.view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
        })
    };
}
