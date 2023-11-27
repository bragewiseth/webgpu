


struct Instances
{
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}


pub struct Entity 
{
    mesh: Mesh,
    material: Material,
    transformation : Option<Transformation>,
    instanses : Instances
}


impl Entity {
    fn draw(&self, render_pass: &mut wgpu::RenderPass) 
    {
        // Set up transformations
        // Call draw on the mesh
    }

}
