
// use crate::components::camera;
use crate::instances::camera;
use crate::components::entity;


pub struct World 
{
    pub entities: Vec<entity::Entity>,
    pub camera: camera::FPSCamera,
}


// impl World {
//
//
//
//
//     fn update(&mut self, delta_time: f32) {
//         // Update objects, camera, etc.
//     }
//
//     fn draw(&self, encoder: &mut wgpu::CommandEncoder) {
//         // Set up render pass and draw each object
//     }
// }
