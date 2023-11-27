// floor
pub const VERTICES: &[ModelVertex] = &[
    ModelVertex { position: [ 1.0,  1.0,  0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },
    ModelVertex { position: [-1.0,  1.0,  0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [-1.0, -1.0,  0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 1.0, 0.0] },
    ModelVertex { position: [ 1.0, -1.0,  0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0], color: [1.0, 0.0, 0.0] },

];


pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0,
];
