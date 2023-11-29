use crate::components::entity;
use crate::components::model::ModelVertex;
use crate::components::mesh::Mesh;
use crate::components::material::Material;
use crate::components::texture;
use wgpu::util::DeviceExt;
use crate::components::entity_instancing:: { Instance, Instances };
use cgmath::prelude::*;




// floor
pub const VERTICES: &[ModelVertex] = &[
    ModelVertex { position: [ 10000.0,  10000.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
    ModelVertex { position: [-10000.0,  10000.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [-10000.0, -10000.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    ModelVertex { position: [ 10000.0, -10000.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
    // ModelVertex { position: [ 10.0,  10.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
    // ModelVertex { position: [-10.0,  10.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    // ModelVertex { position: [-10.0, -10.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.5, 0.4] },
    // ModelVertex { position: [ 10.0, -10.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [0.6, 0.6, 0.4] },
];


pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
];




pub fn new_entity(device: &wgpu::Device, queue: &wgpu::Queue) -> entity::Entity
{
    let diffuse_bytes = include_bytes!("../../assets/img.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();
    let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "../../assets/img.png").unwrap(); // CHANGED!

    let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });


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


    let mesh = Mesh{ vertex_buffer, index_buffer, num_indices }; // NEW!
    let material = Material{ diffuse_texture, diffuse_bind_group, texture_bind_group_layout, 
        diffuse: [1.0; 4], specular: [1.0; 3], roughness: 0.0, metallic: 0.0, emissive: [0.0; 3] }; // NEW! 
    let instances = make_instances(device); // NEW!
    entity::Entity { mesh, material, instances: Some(instances) } // NEW!

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





