use crate::core::model::{Model, Instance};
use crate::core::resources;
use crate::core::pipeline;
use crate::app::models::floor;
use crate::app::models::cube;

use cgmath::prelude::*;

pub struct World
{       
    cube: Model,
    cube_instances: Vec<Instance>,
    floor: Model,
    sphere: Model,
    sphere_instances: Vec<Instance>,
}



impl World 
{
    pub async fn new(config: &wgpu::SurfaceConfiguration, device: &wgpu::Device, queue: &wgpu::Queue, layouts: &pipeline::Layouts ) -> Self
    {


        let sphere = resources::load_model("shpere.obj", &device, &queue, &layouts.texture)
            .await
            .unwrap();


        const SPACE_BETWEEN: f32 = 3.0;
        let sphere_instances = (0..1).flat_map(|z| {
            (0..1).map(move |x| {
                let x = 2.0 * (x as f32 - 1.0 as f32 / 2.0);
                let z = 2.0 * (z as f32 - 1.0 as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance { position, rotation, }
            })
        }).collect::<Vec<_>>();


        let (cube, instances) = cube::new(&device, &layouts );
        let floor = floor::new(&device, &layouts);




        Self
        {
            cube,
            cube_instances: instances.instances,
            floor,
            sphere,
            sphere_instances,
        }


    }
}
