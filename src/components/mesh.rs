
pub struct Mesh 
{
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}



// impl Mesh {
//     fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u32]) -> Self 
//     {
//
//     }
//
//     fn draw(&self, render_pass: &mut wgpu::RenderPass) 
//     {
//         // Bind and draw the mesh
//     }
// }
