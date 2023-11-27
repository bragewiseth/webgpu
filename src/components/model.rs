
// model.rs
pub trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex 
{
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}




impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },

            ],
        }
    }
}
 

 






// cube
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





 

 
