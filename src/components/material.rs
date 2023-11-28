use crate::components::texture;


pub struct Material 
{
    pub diffuse_texture: texture::Texture,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
    pub diffuse: [f32; 4],     // Diffuse color (RGBA)
    pub specular: [f32; 3],    // Specular color (RGB)
    pub roughness: f32,        // Roughness value
    pub metallic: f32,         // Metallic value
    pub emissive: [f32; 3],    // Emissive color (RGB)
}

// impl Material {
    // fn new(device: &wgpu::Device, queue: &wgpu::Queue, texture_data: &[u8]) -> Self {
        // Create texture and bind group
    // }
// }
