// Vertex shader




struct CameraUniform 
{
    view_proj: mat4x4<f32>,
};


@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;
// Vertex shader


struct VertexInput 
{
    @location(0) position: vec3<f32>,
}




struct VertexOutput 
{
    @builtin(position) clip_position: vec4<f32>,
    @location(1) vertex_pos: vec3<f32>,
}




@vertex
fn vs_main( model: VertexInput ) -> VertexOutput 
{
    var out: VertexOutput;
    out.vertex_pos = model.position;
    out.clip_position =   camera.view_proj *  vec4<f32>(model.position, 1.0);
    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let white = vec3<f32>(0.05, 0.05, 0.05);
    let black = vec3<f32>(0.08, 0.08, 0.08);
    
    let r = length(in.vertex_pos) * 0.05;
    let x = floor(in.vertex_pos.x * 0.1);
    let y = floor(in.vertex_pos.y * 0.1);
    let checker = abs(x + y) % 2.0; // Alternates between 0 and 1

    let color = mix(white, black, checker);

    return vec4<f32>(color, 1.0);

    //return vec4<f32>(1.0, 1.0, 1.0, 1.0);

}
