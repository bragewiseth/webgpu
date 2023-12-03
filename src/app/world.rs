use crate::core::model::{Model, Instances, Instance, Mesh  };
use crate::core::renderer:: BindGroupLayouts ;
use crate::core::assets;
use crate::core::renderer::SCREENQUAD;
use crate::core::renderer::SCREENQUAD_INDICES;

use cgmath::prelude::*;
use wgpu::util::DeviceExt;


pub struct World
{       
    pub cube: Model,
    pub cube_instances: Instances,
    pub floor: Mesh,
    pub sphere: Model,
    pub sphere_instances: Instances,
    pub plane: Mesh,
}




impl World 
{
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue, layouts : &BindGroupLayouts  ) -> Self
    {
        let mat_bind_group_layout = &layouts.material;

        let sphere = assets::load_model("sphere1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();
        
        let sphere_1 = assets::load_model("sphere1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let cube = assets::load_model("cube1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let cube_1 = assets::load_model("cube1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let floor = assets::load_meshes_only("floor.obj", device) 
            .await
            .unwrap()
            .pop()
            .unwrap();


        let plane = Mesh::new(device, SCREENQUAD.to_vec(), SCREENQUAD_INDICES.to_vec());



        const SPACE_BETWEEN: f32 = 10.0;
        let sphere_instances = (0..3).map(|x| {
                let x = SPACE_BETWEEN * (x as f32 - 3 as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z:5.0 } ;

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };
                let scale = cgmath::Vector3 { x: 1.0, y: 1.0, z: 1.0 };
                Instance { position, rotation, scale }
        }).collect::<Vec<_>>();


        let instance_data = sphere_instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );
        let sphere_instances = Instances{ instances: sphere_instances, buffer };
        let sphere_x : [f32; 3] = [0.0, 10.0, 20.0];
        let sphere_y : [f32; 3] = [0.0, 0.0, 0.0];

        const NUM_INSTANCES_PER_SHPERE: u32 = 3;
        let cube_instances = (0..NUM_INSTANCES_PER_SHPERE).flat_map(|x| {
            (0..NUM_INSTANCES_PER_SHPERE).map(move |y| {

                let x = sphere_x[x as usize] + 2.0 * (x as f32 - NUM_INSTANCES_PER_SHPERE as f32); 
                let y = sphere_y[y as usize] + 2.0 * (y as f32 - NUM_INSTANCES_PER_SHPERE as f32);

                let position = cgmath::Vector3 { x, y, z: 5.0 };

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };
                let scale = cgmath::Vector3 { x: 1.0, y: 0.2, z: 0.2 };
                Instance {
                    position, rotation, scale
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = cube_instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );

        let cube_instances = Instances{ instances: cube_instances, buffer };






        Self
        {
            cube,
            cube_instances,
            floor,
            sphere,
            sphere_instances,
            plane,
        }


    }
}
