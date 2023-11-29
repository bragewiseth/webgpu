use cgmath::prelude::*;
use crate::components::entity;
use crate::components::entity_instancing:: { Instance, Instances };
use crate::components::model::ModelVertex;
use crate::components::mesh::Mesh;
use crate::components::material::Material;
use crate::components::texture;
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



pub fn new_entity(device: &wgpu::Device, queue: &wgpu::Queue) -> entity::Entity
{

    
    let mesh = Mesh{ vertex_buffer, index_buffer, num_indices }; // NEW!
    let instances = make_instances(device); // NEW!
    entity::Entity { mesh, material, instances: Some(instances) }

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

