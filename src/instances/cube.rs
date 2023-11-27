
use crate::instances;
use crate::components::vertex::ModelVertex;
use crate::components::mesh;







pub const VERTICES: &[ModelVertex] = &[

    ModelVertex { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0], color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [-0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0], color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [ 0.5, -0.5,  0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },

];

// wire cube
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

const NUM_INSTANCES_PER_ROW: u32 = 3;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(1.0, 1.0, 2.0);




fn new_entity() -> entity::Entity
{
    let diffuse_bytes = include_bytes!("../assets/img.png");
    let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
    let diffuse_rgba = diffuse_image.to_rgba8();
    let diffuse_texture = texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "../assets/img.png").unwrap(); // CHANGED!

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


    let mesh = mesh::Mesh{ vertex_buffer, index_buffer, num_indices }; // NEW!
    let material = material::Material{ diffuse_bind_group, }; // NEW!
    let instances = make_instances(); // NEW!
    cube = entity::Entity::new(mesh, material, instances); // NEW!

}







fn make_instances() -> Instances 
{
    let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
        (0..NUM_INSTANCES_PER_ROW).map(move |x| {
            let position = cgmath::Vector3 { x: x as f32 * 2.0, y: 0.0, z: z as f32 * 2.0 } - INSTANCE_DISPLACEMENT;

            let rotation = if position.is_zero() {
                // this is needed so an object at (0, 0, 0) won't get scaled to zero
                // as Quaternions can effect scale if they're not created correctly
                cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
            } else {
                cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
            };

            Instance {
                position, rotation,
            }
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





















//#############
let cube = entity::Entity::new(mesh, material); // NEW!


let instances = (0..NUM_INSTANCES_PER_ROW).flat_map(|z| {
    (0..NUM_INSTANCES_PER_ROW).map(move |x| {
        let position = cgmath::Vector3 { x: x as f32 * 2.0, y: 0.0, z: z as f32 * 2.0 } - INSTANCE_DISPLACEMENT;

        let rotation = if position.is_zero() {
            // this is needed so an object at (0, 0, 0) won't get scaled to zero
            // as Quaternions can effect scale if they're not created correctly
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0))
        } else {
            cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
        };

        Instance {
            position, rotation,
        }
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
