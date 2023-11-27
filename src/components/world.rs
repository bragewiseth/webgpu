use wgpu::Instance;

use crate::components::camera;
use crate::components::entity;

pub struct World 
{
    entities: Vec<entity::Entity>,
    instances: Instance;
    camera: camera::Camera,
    // Other scene properties
}


impl World {




    fn update(&mut self, delta_time: f32) {
        // Update objects, camera, etc.
    }

    fn draw(&self, encoder: &mut wgpu::CommandEncoder) {
        // Set up render pass and draw each object
    }
}
