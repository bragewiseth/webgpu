use crate::core::renderer::InstanceRaw;
use crate::core::renderer::VertexBuffer;
use crate::core::texture::Texture;

use wgpu::util::DeviceExt;




pub struct Mesh 
{
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
} 


impl Mesh {
    
    pub fn new(
        device: &wgpu::Device,
        vertices: Vec<impl VertexBuffer + bytemuck::Pod + bytemuck::Zeroable>,
        indices: Vec<u32>,
    ) -> Self {

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label:None,
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            Self {
                name: "some name".to_string(),
                vertex_buffer,
                index_buffer,
                num_elements: indices.len() as u32,
            }
    }
}






pub struct Material 
{
    pub name: String,
    pub diffuse_color: Color,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color
{
    pub color: [f32; 4],
}


pub struct Model 
{
    pub meshes: Vec<Mesh>,
    pub materials: Vec<u32>,
}


pub struct Instance 
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}



impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            // model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
            model: (
                cgmath::Matrix4::from_translation(self.position) * 
                cgmath::Matrix4::from(self.rotation) * 
                cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)).into(),
        }
    }
}


pub struct Instances
{
    pub instances: Vec<Instance>,
    pub buffer: wgpu::Buffer,
} 
