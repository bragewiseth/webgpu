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

const NUM_INSTANCES_PER_ROW: u32 = 10;



impl World 
{
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue, layouts : &BindGroupLayouts  ) -> Self
    {
        let mat_bind_group_layout = &layouts.material;

        let sphere = assets::load_model("sphere.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();
        
        let sphere_1 = assets::load_model("sphere1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let cube = assets::load_model("cube.obj", device, queue, mat_bind_group_layout)
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



        const SPACE_BETWEEN: f32 = 3.0;
        let sphere_instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z } + cgmath::Vector3 { x: 0.0, y: 0.0, z: 20.0 }; 

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();


        let instance_data = sphere_instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let sphere_instances = Instances{ instances: sphere_instances, buffer };


        let cube_instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
            (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = cgmath::Vector3 { x, y: 0.0, z } + cgmath::Vector3 { x: 0.0, y: 0.0, z: 20.0 }; 

                let rotation = if position.is_zero() {
                    cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
                } else {
                    cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                };

                Instance {
                    position, rotation,
                }
            })
        }).collect::<Vec<_>>();

        let instance_data = cube_instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX,
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
