use crate::core::renderer::
{ 
    Framebuffer, 
    Draw,
    BindGroupLayouts,
    RenderPipeline,
    PipelineResources,
    PipelineBuffers,
    Resource, VertexUV,
    SCREENQUAD,
    SCREENQUAD_INDICES,
    VertexBuffer,
};

pub struct Engine
{ 
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    screenquad : Mesh,
}
