struct VertexInput 
{   
    @location(0) position: vec3<f32>,
}

struct VertexOutput 
{
    @builtin(position) clip_position: vec4<f32>,
}



@vertex
fn vs_main( model: VertexInput ) -> VertexOutput 
{
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}



struct Color
{
    color: vec4<f32>
}

@group(0) @binding(0)
var<uniform> c_diffuse: Color;
@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(2)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = vec2<f32>(in.clip_position.x, in.clip_position.y);
    return textureSample(t_diffuse, s_diffuse,uv );
    //return c_diffuse.color;
}
