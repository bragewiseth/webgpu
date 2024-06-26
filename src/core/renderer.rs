use crate::scene::camera::Camera;
use cgmath::prelude::*;
use wgpu::*;


pub const QUADVERTS: &[[f32; 3]; 4] = &[ [-1.0, -1.0, 0.0], [-1.0,  1.0, 0.0], [ 1.0, -1.0, 0.0], [ 1.0,  1.0, 0.0], ];
pub const QUADUV: &[[f32; 2]; 4] = &[[0.0, 1.0], [0.0, 0.0], [1.0, 1.0], [1.0, 0.0]];
pub const QUADNORMALS: &[[f32; 3]; 4] = &[[0.0, 0.0, 0.0]; 4];
pub const QUADMESH_INDICES: &[u32] = &[2, 1, 0, 3, 1, 2];




/****************************************************************************************
 * VERTEX STUFF
 ****************************************************************************************/



#[macro_export]
macro_rules! define_instance_buffer 
{
    ($name:ident, $(($field:ident, $size:ty, $format:expr, $location:expr, $divide:expr)),*) => {
        #[repr(C)]
        #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct $name {
            $(
                pub $field: $size,
            )*
        }

        impl $name {
            fn desc() -> VertexBufferLayout<'static> 
            {
                use std::mem;
                let mut attributes = Vec::new();
                let mut offset = 0;

                $(
                    for i in 0..$divide
                    {
                        attributes.push(VertexAttribute {
                            offset,
                            shader_location: $location + i as u32,
                            format: $format,
                        });
                        offset += mem::size_of::<$size>() as BufferAddress / $divide as BufferAddress;
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
    ($name:ident, $(($field:ident, $size:ty, $format:expr, $location:expr)),*) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        pub struct $name 
        {
            $(
                pub $field: $size,
            )*
        }

        impl $name {
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
    texture_view: &TextureView,
) -> BindGroupEntry 
{
    BindGroupEntry 
    {
        binding,
        resource: BindingResource::TextureView(texture_view),
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
    buffer: BindingResource<'static>,
) -> BindGroupEntry<'static>
{
    BindGroupEntry 
    {
        binding,
        resource: buffer,
    }
}

/****************************************************************************************
 * USEFUL UNIFORMS
 ****************************************************************************************/

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform 
{
    pub view: [[f32; 4]; 4],
    pub proj: [[f32; 4]; 4],
    pub view_position: [f32; 4],
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


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform
{
    position: [f32; 4],
    color: [f32; 4],
    intensity: f32,
    _padding: f32,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PlayerUniform
{
    pub model: [[f32; 4]; 4],
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

    pub fn new_with(camera: &Camera) -> Self
    {
        Self 
        {
            view: camera.calc_matrix().into(),
            proj: camera.projection.calc_matrix().into(),
            view_position: camera.position.to_homogeneous().into()
        }
    }
}

pub fn camera_to_uniform(camera: &Camera, uniform: &mut CameraUniform) 
{
    uniform.view_position = camera.position.to_homogeneous().into();
    uniform.proj = camera.projection.calc_matrix().into();
    uniform.view = camera.calc_matrix().into();
}


impl MaterialUniform 
{
    pub fn new() -> Self
    {
        Self 
        {
            color: [1.0, 1.0, 1.0, 1.0],
            roughness: 0.0,
            metallic: 0.0,
            _padding: 0.0,
        }
    }
    pub fn new_with(color: [f32; 3], roughness: f32, metallic: f32) -> Self
    {
        Self 
        {
            color : [color[0], color[1], color[2], 1.0],
            roughness,
            metallic,
            _padding: 0.0,
        }
    }
}

/****************************************************************************************
 * PIPELINE and RENDER PASS MACROS
 ****************************************************************************************/


pub fn alphablend_color_target_state(format: TextureFormat) -> ColorTargetState 
{
    ColorTargetState 
    {
        format,
        blend: Some(BlendState 
        {
            color: BlendComponent 
            {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha: BlendComponent 
            {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
        }),
        write_mask: ColorWrites::ALL,
    }
} 


pub fn primitive_state(topology: PrimitiveTopology, polygon_mode: PolygonMode) -> PrimitiveState 
{
    PrimitiveState 
    {
        topology,
        strip_index_format: None,
        front_face: FrontFace::Cw,
        cull_mode: Some(Face::Back),
        unclipped_depth: false,
        polygon_mode,
        conservative: false,
    }
}


pub fn depth_stencil_state(format: TextureFormat, depth_write_enabled: bool, depth_compare: CompareFunction) -> DepthStencilState 
{
    DepthStencilState 
    {
        format,
        depth_write_enabled,
        depth_compare,
        stencil: StencilState::default(),
        bias: DepthBiasState::default(),
    }
}

pub fn multisample_state(count: u32) -> MultisampleState 
{
    MultisampleState 
    {
        count,
        mask: !0,
        alpha_to_coverage_enabled: false,
    }
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
    ($encoder:expr, $view:expr, $z_buffer_view:expr) => {
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
                    view: &$z_buffer_view,
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
