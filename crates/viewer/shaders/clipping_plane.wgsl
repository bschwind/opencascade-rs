struct VertexInput {
    @location(0)
    pos: vec2<f32>,
};

struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(input.pos, 0.0, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.976, 0.384, 0.035, 1.0);
}
