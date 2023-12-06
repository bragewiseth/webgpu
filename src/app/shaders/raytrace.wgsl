// Vertex shader




struct CameraUniform 
{
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};


@group(0) @binding(0) // 1.
var<uniform> camera: CameraUniform;
// Vertex shader





struct VertexInput 
{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
}




struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) pos: vec3<f32>,
}




@vertex
fn vs_main( model: VertexInput) -> VertexOutput {

    var out: VertexOutput;
    out.uv = model.uv;
    out.pos = model.position;
    out.clip_position =  vec4<f32>(model.position, 1.0);
    return out;
}


struct Color { color: vec4<f32>, };

@group(1) @binding(2)
var<uniform> c_diffuse: Color;
@group(1) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(1)@binding(0)
var s_diffuse: sampler;





@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> 
{

    // ray trace sphere 
    var ray_origin: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0);
    var ray_dir: vec3<f32> = in.pos - ray_origin;
    var ray_len: f32 = length(ray_dir);
    ray_dir = normalize(ray_dir);
    var ray_len2: f32 = ray_len * ray_len;
    var ray_len4: f32 = ray_len2 * ray_len2;
    var radius: f32 = 1.0;
    var radius2: f32 = radius * radius;
    var radius4: f32 = radius2 * radius2;
    var a: f32 = ray_len4;
    var b: f32 = 2.0 * ray_len2 * dot(ray_dir, ray_origin);
    var c: f32 = ray_len4 - radius4;
    var d: f32 = b * b - 4.0 * a * c;
    var t: f32 = (-b + sqrt(d)) / (2.0 * a);
    var hit: vec3<f32> = ray_origin + ray_dir * t;
    var normal: vec3<f32> = normalize(hit);
    return vec4<f32>(normal * 0.5 + 0.5, 1.0);

}
