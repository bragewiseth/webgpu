use crate::components::model::ModelVertex;
use crate::components::mesh::Mesh;
use wgpu::util::DeviceExt;





pub const VERTICES: &[ModelVertex] = &[
    ModelVertex { position: [ 1.0,  1.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
    ModelVertex { position: [-1.0,  1.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [-1.0, -1.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [ 1.0, -1.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },

];

pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
];




pub fn new_entity(device: &wgpu::Device) -> Mesh
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

    let num_indices = INDICES.len() as u32;


    Mesh{ vertex_buffer, index_buffer, num_indices }

}



