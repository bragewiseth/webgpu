use crate::instances;
use crate::components::world;



fn new_world(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device) -> world::World
{
    let camera = instances::camera::FPSCamera::new(config, device);
    let cube = instances::cube::Cube::new_entity(device);

    world::World
    {
        entities: vec![cube],
        camera: camera,
    }


}




