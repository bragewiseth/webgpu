use crate::core::renderer::InstanceRaw;
use crate::core::texture::Texture;
use crate::core::renderer::VertexBuffer;

use wgpu::util::DeviceExt;






// MESH {{{
pub struct Mesh 
{
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
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
                material: 0,
            }
    }
}




// }}}


// MATERIAL {{{
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
// }}}


// MODEL {{{
pub struct Model 
{
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
} // }}}


// INSTANCING {{{
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
// }}}






// DRAW LIGHT {{{
// pub trait DrawLight<'a> {
//     fn draw_light_mesh(
//         &mut self,
//         mesh: &'a Mesh,
//         camera_bind_group: &'a wgpu::BindGroup,
//         light_bind_group: &'a wgpu::BindGroup,
//     );
//     fn draw_light_mesh_instanced(
//         &mut self,
//         mesh: &'a Mesh,
//         instances: Range<u32>,
//         camera_bind_group: &'a wgpu::BindGroup,
//         light_bind_group: &'a wgpu::BindGroup,
//     );
//
//     fn draw_light_model(
//         &mut self,
//         model: &'a Model,
//         camera_bind_group: &'a wgpu::BindGroup,
//         light_bind_group: &'a wgpu::BindGroup,
//     );
//     fn draw_light_model_instanced(
//         &mut self,
//         model: &'a Model,
//         instances: Range<u32>,
//         camera_bind_group: &'a wgpu::BindGroup,
//         light_bind_group: &'a wgpu::BindGroup,
//     );
// }



// impl<'a, 'b> DrawLight<'b> for wgpu::RenderPass<'a>
// where
//     'b: 'a,
// {
//     fn draw_light_mesh(
//         &mut self,
//         mesh: &'b Mesh,
//         camera_bind_group: &'b wgpu::BindGroup,
//         light_bind_group: &'b wgpu::BindGroup,
//     ) {
//         self.draw_light_mesh_instanced(mesh, 0..1, camera_bind_group, light_bind_group);
//     }
//
//     fn draw_light_mesh_instanced(
//         &mut self,
//         mesh: &'b Mesh,
//         instances: Range<u32>,
//         camera_bind_group: &'b wgpu::BindGroup,
//         light_bind_group: &'b wgpu::BindGroup,
//     ) {
//         self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
//         self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
//         self.set_bind_group(0, camera_bind_group, &[]);
//         self.set_bind_group(1, light_bind_group, &[]);
//         self.draw_indexed(0..mesh.num_elements, 0, instances);
//     }
//
//     fn draw_light_model(
//         &mut self,
//         model: &'b Model,
//         camera_bind_group: &'b wgpu::BindGroup,
//         light_bind_group: &'b wgpu::BindGroup,
//     ) {
//         self.draw_light_model_instanced(model, 0..1, camera_bind_group, light_bind_group);
//     }
//     fn draw_light_model_instanced(
//         &mut self,
//         model: &'b Model,
//         instances: Range<u32>,
//         camera_bind_group: &'b wgpu::BindGroup,
//         light_bind_group: &'b wgpu::BindGroup,
//     ) {
//         for mesh in &model.meshes {
//             self.draw_light_mesh_instanced(mesh, instances.clone(), camera_bind_group, light_bind_group);
//         }
//     }
// } 
// }}}
