struct Globals {
    proj: mat4x4<f32>,
    transform: mat4x4<f32>,
};

// Uniforms
@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    @location(0)
    pos: vec3<f32>,

    @location(1)
    uv: vec2<f32>,

    @location(2)
    normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,

    @location(0)
    uv: vec2<f32>,

    @location(1)
    normal: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.uv = input.uv;
    out.normal = input.normal; // TODO(bschwind) - Need to transform this.
    out.pos = globals.proj * globals.transform * vec4<f32>(input.pos, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
