use crate::renderer::VertexBufferTrait;





pub struct Mesh<T : VertexBufferTrait>
{
    pub name: String,
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
    pub num_elements: u32,
} 



pub struct Material 
{
    pub name: String,
    pub diffuse_color: Color,
    pub diffuse_texture: wgpu::Texture,
}



#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color
{
    pub color: [f32; 4],
}




pub struct Model<T : VertexBufferTrait> 
{
    pub meshes: Vec<Mesh<T>>,
    pub materials: Vec<u32>,
}


pub struct Instance 
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}

pub struct InstanceBuffer
{
    pub model: [[f32; 4]; 4],
}


impl Instance {
    pub fn to_array(&self) -> InstanceBuffer
    {   
        InstanceBuffer
        {
            model: (
                cgmath::Matrix4::from_translation(self.position) * 
                cgmath::Matrix4::from(self.rotation) * 
                cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            ).into(),
        }
    }
}




pub struct Instances
{
    pub instances: Vec<Instance>,
} 
