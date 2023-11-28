use crate::components::mesh::Mesh;
use crate::components::material::Material;
use crate::components::entity_instancing::Instances;





pub struct Entity 
{
    pub mesh: Mesh,
    pub material: Material,
    pub instances : Option<Instances>,
}


// impl Entity {
//     fn draw(&self, render_pass: &mut wgpu::RenderPass) 
//     {
//         // Set up transformations
//         // Call draw on the mesh
//     }
//
// }
