use crate::core::model::{Model, Mesh, Instance, Instances, Diffuse, Material, ColorUniform };
use crate::core::pipeline::{ Layouts, ModelVertex };


use cgmath::prelude::*;
use wgpu::util::DeviceExt;




// floor
pub const VERTICES: &[ModelVertex] = &[
    ModelVertex { position: [ 10000.0,  10000.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0]},// color: [0.6, 0.6, 0.4] },
    ModelVertex { position: [-10000.0,  10000.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0]},// color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [-10000.0, -10000.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0]},// color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [ 10000.0, -10000.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0]}// color: [0.6, 0.6, 0.4] },
    // ModelVertex { position: [ 10.0,  10.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
    // ModelVertex { position: [-10.0,  10.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    // ModelVertex { position: [-10.0, -10.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    // ModelVertex { position: [ 10.0, -10.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
];


pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
];




pub fn new(device: &wgpu::Device,  layouts: &Layouts ) -> Model
{

    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    // NEW!
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );

    let num_elements = INDICES.len() as u32;

    let diffuse = Diffuse::ColorFactor(ColorUniform::new([0.1, 0.2, 0.3, 1.0]));
    let diffuse_uniform = diffuse.to_uniform();
    let diffuse_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Diffuse Buffer"),
            contents: bytemuck::cast_slice(&[diffuse_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &layouts.color,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: diffuse_buffer.as_entire_binding(),
                }
        ],
        label: Some("color_bind_group"),
    });


    let material = Material
    { 
        name: "floor".to_string(),
        diffuse,
        bind_group
    };
    
    let mesh = Mesh{ name: "floor".to_string(), vertex_buffer, index_buffer, num_elements, material: 0 };
    // let instances = make_instances(device);
    let floor = Model { meshes: vec![mesh], materials: vec![material] };
    floor
}


fn make_instances(device: &wgpu::Device) -> Instances 
{
    let instances = (0..1).flat_map(|z| {
        (0..1).map(move |x| {
            let position = cgmath::Vector3 { x: x as f32 , y: 0.0, z: z as f32 };

            let rotation = if position.is_zero() {
                // this is needed so an object at (0, 0, 0) won't get scaled to zero
                // as Quaternions can effect scale if they're not created correctly
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
            } else {
                cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
            };

            Instance { position, rotation, }
        })
    }).collect::<Vec<_>>();

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    Instances { instances, instance_buffer }
}





