use crate::instances;
use crate::components::world;



pub fn new_world(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue) -> world::World
{

    let camera = instances::camera::FPSCamera::new(config, device);
    let cube = instances::cube::new_entity(device,queue);
    let floor = instances::floor::new_entity(device,queue);

    world::World
    {
        entities: vec![cube,floor],
        camera,
    }


}




