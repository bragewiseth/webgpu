// This file contains layouts for gpu input
use crate::core::camera::Camera;
use crate::core::texture::Texture;
use crate::core::model::{ Material, Model, Instances, Mesh };

use std::ops::Range;
use wgpu::util::DeviceExt;


pub const SCREENQUAD: [VertexOnly; 4] = [
    VertexOnly { position: [-1.0, -1.0, 0.0] },
    VertexOnly { position: [-1.0,  1.0, 0.0] },
    VertexOnly { position: [ 1.0, -1.0, 0.0] },
    VertexOnly { position: [ 1.0,  1.0, 0.0] },
];
pub const SCREENQUAD_INDICES: &[u32] = &[2, 1, 0, 3, 1, 2];



// VERTEX BUFFER LAYOUTS {{{
pub trait VertexBuffer
{
    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw 
{
    pub model: [[f32; 4]; 4],
}



#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex 
{
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub normal: [f32; 3],
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexOnly
{
    pub position: [f32; 3],
}

impl VertexBuffer for VertexOnly {}

impl VertexBuffer for InstanceRaw {
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
} // end VERTEX BUFFER LAYOUTS  }}}


// BIND GROUP LAYOUTS {{{
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



impl Resource for Material
{
    fn desc( device : &wgpu::Device ) -> wgpu::BindGroupLayout
    {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry 
                {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer 
                    {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
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
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ],
            label: Some("color_bind_group_layout"),
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
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ],
            label: Some("color_bind_group_layout"),
        })
    }
}
// end BIND GROUP LAYOUTS }}}


// PIPELINES {{{

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
}

pub enum PipelineResources
{
    Camera,
    Material
}

pub enum PipelineBuffers
{
    Model,
    Instance,
    VertexOnly
}


pub struct RenderPipeline
{
    pub pipeline: wgpu::RenderPipeline,
    pub resources: Vec<PipelineResources>,
    pub vertex_buffers: Vec<PipelineBuffers>,
}




impl RenderPipeline
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
                PipelineResources::Camera => { &layouts.camera },
                PipelineResources::Material => { &layouts.material },
            }
        }).collect();
        let buffers : Vec<wgpu::VertexBufferLayout<'static>> = vertex_buffers.iter().map(|x| 
        {
            match x
            {
                PipelineBuffers::Model => { ModelVertex::desc() },
                PipelineBuffers::Instance => { InstanceRaw::desc() },
                PipelineBuffers::VertexOnly => { VertexOnly::desc() },
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
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
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
                multiview: None, // 5.
            }
        );
        Self { pipeline, resources, vertex_buffers }
    }

}
// end PIPELINES }}}


// RENDERPASS {{{
pub struct Framebuffer
{
    pub texture: Option<Texture>,
    pub depth_texture: Option<Texture>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl Framebuffer
{
    pub fn make_bind_group(device : &wgpu::Device, layout : &BindGroupLayouts, texture: &Texture) -> wgpu::BindGroup
    {
        let diffuse_color = [1.0, 1.0, 1.0, 1.0];
        device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layout.material,
        entries: &[
            wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(
                    wgpu::BufferBinding {
                                buffer: &device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                                        label: Some("Material Color Buffer"),
                                        contents: bytemuck::cast_slice(&[diffuse_color]),
                                        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                                    }
                        ),
                                offset: 0,
                                size: None,
                            }
                ),
                    },
            wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
            wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    },
        ],
        label: None,
        })
    }
}



pub trait Draw<'a>
{

    fn draw_pipeline_instanced(
        &mut self, 
        pipeline: &'a RenderPipeline, 
        model : &'a Model, 
        instances : &'a Instances,
        num_instances: Range<u32>,
        camera: &'a wgpu::BindGroup) -> ();

    fn draw_pipeline(
        &mut self, 
        pipeline: &'a RenderPipeline, 
        model : &'a Model, 
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


    fn draw_model(
        &mut self,
        model: &'a Model,
    );


    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        instances: &'a Instances,
        num_instances: Range<u32>,
    );
}


// impl<'a, 'b> Draw<'b> for wgpu::RenderPass<'a>
// where
//     'b: 'a,
// {
impl<'a, 'b> Draw<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_pipeline_instanced(
        &mut self, 
        pipeline : &'b RenderPipeline, 
        model : &'b Model, 
        instances : &'b Instances,
        num_instances: Range<u32>,
        camera: &'b wgpu::BindGroup) -> ()
    {
        self.set_pipeline(&pipeline.pipeline);
        for (i,resource) in pipeline.resources.iter().enumerate()
        {
            match resource
            {
                PipelineResources::Camera => { self.set_bind_group(i as u32, camera, &[]); },
                PipelineResources::Material => { self.set_bind_group(i as u32, &model.materials[0].bind_group, &[]); },
            }
        }
        for (i,buffer) in pipeline.vertex_buffers.iter().enumerate()
        {
            match buffer
            {
                PipelineBuffers::Model => { self.set_vertex_buffer(i as u32, model.meshes[0].vertex_buffer.slice(..)); },
                PipelineBuffers::Instance => { self.set_vertex_buffer(i as u32 , instances.buffer.slice(..)); },
                PipelineBuffers::VertexOnly => { self.set_vertex_buffer(i as u32, model.meshes[0].vertex_buffer.slice(..)); },
            }
        }
        self.set_index_buffer(model.meshes[0].index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..model.meshes[0].num_elements, 0, num_instances);
    }


    fn draw_pipeline(
        &mut self, 
        pipeline : &'b RenderPipeline, 
        model : &'b Model, 
        camera: &'b wgpu::BindGroup) -> ()
    {
        self.set_pipeline(&pipeline.pipeline);
        for (i,resource) in pipeline.resources.iter().enumerate()
        {
            match resource
            {
                PipelineResources::Camera => { self.set_bind_group(i as u32, camera, &[]); },
                PipelineResources::Material => { self.set_bind_group(i as u32, &model.materials[0].bind_group, &[]); },
            }
        }
        for (i,buffer) in pipeline.vertex_buffers.iter().enumerate()
        {
            match buffer
            {
                PipelineBuffers::Model => { self.set_vertex_buffer(i as u32, model.meshes[0].vertex_buffer.slice(..)); },
                PipelineBuffers::Instance => { },
                PipelineBuffers::VertexOnly => { self.set_vertex_buffer(i as u32, model.meshes[0].vertex_buffer.slice(..)); },
            }
        }
        self.set_index_buffer(model.meshes[0].index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.draw_indexed(0..model.meshes[0].num_elements, 0, 0..1);
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

    fn draw_model(
        &mut self,
        model: &'b Model,) 
    {
        self.set_vertex_buffer(0, model.meshes[0].vertex_buffer.slice(..));
        self.set_index_buffer(model.meshes[0].index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(1, &model.materials[0].bind_group, &[]);
        self.draw_indexed(0..model.meshes[0].num_elements, 0, 0..1);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        instances: &'b Instances,
        num_instances: Range<u32>,)
    {
        self.set_vertex_buffer(0, model.meshes[0].vertex_buffer.slice(..));
        self.set_vertex_buffer(1, instances.buffer.slice(..));
        self.set_index_buffer(model.meshes[0].index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(1, &model.materials[0].bind_group, &[]);
        self.draw_indexed(0..model.meshes[0].num_elements, 0, num_instances);
    }
}

    
    


// end RENDERPASS }}}






// STUFF THAT COULD POTEINTIALLY BE USED LATER {{{

// pub struct Layouts
// {
//     pub camera: wgpu::BindGroupLayout,
//     pub texture: wgpu::BindGroupLayout,
//     pub color: wgpu::BindGroupLayout,
//     // pub light: wgpu::BindGroupLayout,
// }
// impl Layouts
// {
//     pub fn new(device : &wgpu::Device) -> Self
//     {
//         let camera = Camera::desc(device);
//         let texture = texture::Texture::desc(device);
//         let color = <[f32; 4]>::desc(device);
//         Self { camera, texture, color }
//     }
// }
//
//
//
//
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
