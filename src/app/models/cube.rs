use crate::core::model::{Model, Mesh, Instance, Instances, Diffuse, Material, ColorUniform };
use crate::core::pipeline::{ Layouts, ModelVertex };


use cgmath::prelude::*;
use wgpu::util::DeviceExt;





pub const VERTICES: &[ModelVertex] = &[

    ModelVertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0]},// color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [-0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0]},// color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0]},// color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0]},// color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [ 0.5, -0.5,  0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0]},// color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0]},// color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0]},// color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0]},// color: [1.0, 0.0, 0.0] },

];

// pub const INDICES: &[u16] = &[
//     // front face
//     0, 1, 1, 2, 2, 3, 3, 0,
//     
//     // back face
//     4, 5, 5, 6, 6, 7, 7, 4,
//
//     // edges connecting front and back faces
//     0, 5, 1, 6, 2, 7, 3, 4,
// ];

// triangle cube
pub const INDICES: &[u16] = &[
    // front face
    2, 1, 0, 0, 3, 2,
    // back face
    4, 5, 6, 6, 7, 4,
    // right face
    3, 0, 5, 5, 4, 3,
    // left face
    6, 1, 2, 2, 7, 6,
    // top face
    0, 1, 6, 6, 5, 0,
    // bottom face
    2, 3, 4, 4, 7, 2,
    
];

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3 { x: 5.0, y: 0.0, z: 0.0 };


pub fn new(device: &wgpu::Device,  layouts: &Layouts ) -> (Model, Instances)
{



    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    let num_elements = INDICES.len() as u32;
    // let diffuse = Diffuse::ColorFactor(ColorUniform::new([0.1, 0.2, 0.3, 1.0]));
    // let diffuse_uniform = diffuse.to_uniform();
    // let diffuse_buffer = device.create_buffer_init(
    //     &wgpu::util::BufferInitDescriptor {
    //         label: Some("Diffuse Buffer"),
    //         contents: bytemuck::cast_slice(&[diffuse_uniform]),
    //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    //     }
    // );
    // let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
    //     layout: &layouts.color,
    //     entries: &[
    //         wgpu::BindGroupEntry {
    //             binding: 0,
    //             resource: diffuse_buffer.as_entire_binding(),
    //             }
    //     ],
    //     label: Some("color_bind_group"),
    // });
    
    let diffuse_bytes = include_bytes!("../../assets/cube.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.as_rgba8().unwrap();
    let diffuse_texture = texture::Texture::from_image(&device, &diffuse_rgba, "cube").unwrap();

    let diffuse_bind_group = device.create_bind_group(
        &wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view), // CHANGED!
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler), // CHANGED!
                }
            ],
            label: Some("diffuse_bind_group"),
        }
    );

    let material = Material
    { 
        name: "cube".to_string(),
        diffuse,
        bind_group, 
    };
    
    let mesh = Mesh{ name: "cube".to_string(), vertex_buffer, index_buffer, num_elements, material: 0 };
    let instances = make_instances(device);
    let cube = Model { meshes: vec![mesh], materials: vec![material] };
    (cube, instances)
}





fn make_instances(device: &wgpu::Device) -> Instances 
{
    let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
            let position = cgmath::Vector3 { x: 0.0, y: 0.0,  z: x as f32 * 1.5 } + INSTANCE_DISPLACEMENT;

            let rotation = if position.is_zero() {
                // this is needed so an object at (0, 0, 0) won't get scaled to zero
                // as Quaternions can effect scale if they're not created correctly
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
            } else {
                cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(0.0))
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

