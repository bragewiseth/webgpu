use fstop::model::{Model, Instances, Instance, Mesh, Material  };
use fstop::renderer:: BindGroupLayouts ;
use fstop::assets;

use cgmath::prelude::*;
use wgpu::util::DeviceExt;




let camera = Camera::new(
    cgmath::Point3::new(0.0, -10.0, 0.0),
    cgmath::Deg(0.0),
    cgmath::Deg(0.0),
    Projection::new(config.width, config.height, cgmath::Deg(45.0), 0.1, 100.0),
    CameraController::new(10.0, 0.2),
);

impl World 
{
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue, layouts : &BindGroupLayouts  ) -> Self
    {
        let mat_bind_group_layout = &layouts.material;

        let (sphere_mesh, sphere_mat) = assets::load_model("sphere.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();
        
        let (sphere_mesh1, sphere_mat1) = assets::load_model("sphere1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let (cube_mesh, cube_mat) = assets::load_model("cube1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let (cube_1, _) = assets::load_model("cube1.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let (floor_mesh, _) = assets::load_model("floor.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();
        
        let (plane, _) = assets::load_model("plane.obj", device, queue, mat_bind_group_layout)
            .await
            .unwrap();

        let mats = vec![sphere_mat, sphere_mat1, cube_mat].into_iter().flatten().collect::<Vec<_>>();

        let cube = Model { meshes: cube_1, materials: vec![2] };
        let sphere = Model { meshes: sphere_mesh, materials: vec![0] };
        let plane = Model { meshes: plane, materials: vec![0] };
        let floor = floor_mesh.into_iter().next().unwrap();


        const SPACE_BETWEEN: f32 = 3.0;
        let sphere_instances = (0..3).map(|x| 
        {
            let x = SPACE_BETWEEN * (x as f32 - 1.0);

            let position = cgmath::Vector3 { x, y: 0.0, z:1.0 } ;

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
                let scale = cgmath::Vector3 { x: 0.4, y: 0.4, z: 0.4 };
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

        let plane_instances = (0..1).map(|_| {
                let position = cgmath::Vector3 { x:0.0, y: 0.0, z:0.0 } ;
                let rotation = cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
                let scale = cgmath::Vector3 { x: 1.0, y: 1.0, z: 1.0 };
                Instance { position, rotation, scale }
        }).collect::<Vec<_>>();
        let instance_data = plane_instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Instance Buffer"),
                contents: bytemuck::cast_slice(&instance_data),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
            }
        );
        let plane_instances = Instances{ instances: plane_instances, buffer };




        Self
        {
            mats,
            cube,
            cube_instances,
            floor,
            sphere,
            sphere_instances,
            plane,
            plane_instances,
        }


    }
}



// let meshes = models
//     .into_iter()
//     .map(|m| 
//         {
//             let pos = (0..m.mesh.positions.len() / 3)
//                 .map(|i| [
//                         m.mesh.positions[i * 3],
//                         m.mesh.positions[i * 3 + 1],
//                         m.mesh.positions[i * 3 + 2],
//                     ]
//                 );
//
//             let uv : Vec<[f32; 2]> = if m.mesh.texcoords.len() > 0 {
//                 (0..m.mesh.texcoords.len() / 2)
//                     .map(|i| [m.mesh.texcoords[i * 2], m.mesh.texcoords[i * 2 + 1]])
//                     .collect()
//             } else {
//                 (0..m.mesh.positions.len() / 3)
//                     .map(|_| [0.0, 0.0])
//                     .collect()
//             }; 
//
//             let normals : Vec<[f32; 3]> = if m.mesh.normals.len() > 0 {
//                 (0..m.mesh.normals.len() / 3)
//                     .map(|i| [
//                         m.mesh.normals[i * 3],
//                         m.mesh.normals[i * 3 + 1],
//                         m.mesh.normals[i * 3 + 2],
//                     ])
//                     .collect()
//             } else {
//                 (0..m.mesh.positions.len() / 3)
//                     .map(|_| [0.0, 0.0, 0.0])
//                     .collect()
//             };
//
//             let vertices = pos.zip(uv).zip(normals).map(|((pos, uv), normal)| 
//             {
//                 VertexBuffer2
//                 {
//                     position: pos,
//                     uv,
//                     normal,
//                 }
//
//             }).collect::<Vec<_>>();
//
//             let indices = m.mesh.indices.clone();
