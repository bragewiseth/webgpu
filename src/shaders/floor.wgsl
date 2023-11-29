// Vertex shader




struct CameraUniform 
{
    view_proj: mat4x4<f32>,
};


@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;
// Vertex shader


struct VertexInput 
{
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) color: vec3<f32>,
}

struct InstanceInput 
{
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};


struct VertexOutput 
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) vertex_pos: vec3<f32>,
}




@vertex
fn vs_main( model: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );    


    var out: VertexOutput;
    out.vertex_pos = model.position;
    out.tex_coords = model.tex_coords;
    out.clip_position =   camera.view_proj *  vec4<f32>(model.position, 1.0);
    return out;
}




@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let white = vec3<f32>(0.05, 0.05, 0.05);
    let black = vec3<f32>(0.08, 0.08, 0.08);

    let x = floor(in.vertex_pos.x * 0.5);
    let y = floor(in.vertex_pos.y * 0.5);
    let checker = abs(x + y) % 2.0; // Alternates between 0 and 1

    let color = mix(white, black, checker);

    return vec4<f32>(color, 1.0);

    //return vec4<f32>(1.0, 1.0, 1.0, 1.0);

}
