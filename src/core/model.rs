use crate::renderer::VertexArray;
use crate::renderer::InstanceArray;





pub struct Mesh 
{
    pub name: String,
    pub vertices: Vec<VertexArray>,
    pub indices: Vec<u32>,
    pub num_elements: u32,
} 



pub struct Material 
{
    pub name: String,
    pub diffuse_color: Color,
    pub diffuse_texture: wgpu::TextureView,
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


pub struct ModelInstance 
{
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub scale: cgmath::Vector3<f32>,
}



impl ModelInstance {
    pub fn to_array(&self) -> InstanceArray
    {   
        InstanceArray
        {
            model: (
                cgmath::Matrix4::from_translation(self.position) * 
                cgmath::Matrix4::from(self.rotation) * 
                cgmath::Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            ).into(),
        }
    }
}




pub struct ModelInstances
{
    pub instances: Vec<ModelInstance>,
} 
