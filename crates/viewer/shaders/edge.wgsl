struct Globals {
    proj: mat4x4<f32>,
    transform: mat4x4<f32>,
    resolution: vec4<f32>, // X = screen width, Y = screen height, Z = dash_size, W = gap_size
};

// Uniforms
@group(0) @binding(0)
var<uniform> globals: Globals;

struct VertexInput {
    // Per-vertex data
    @location(0)
    pos: vec3<f32>,

    // Per-instance data
    @location(1)
    point_a: vec4<f32>,

    @location(2)
    length_so_far_a: vec4<f32>,

    @location(3)
    point_b: vec4<f32>,

    @location(4)
    length_so_far_b: vec4<f32>,
};

struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,

    @location(0)
    dist: f32,

    @location(1)
    plane_distance: f32,
};

@vertex
fn main_vs(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let a_width = input.point_a.w;
    let b_width = input.point_b.w;

    // Transform the segment endpoints to clip space
    let clip0 = globals.proj * globals.transform * vec4<f32>(input.point_a.xyz, 1.0);
    let clip1 = globals.proj * globals.transform * vec4<f32>(input.point_b.xyz, 1.0);

    let interpolated_pos = vec4<f32>(mix(input.point_a.xyz, input.point_b.xyz, vec3<f32>(input.pos.z)), 1.0);

    // Transform the segment endpoints to screen space
    let a = globals.resolution.xy * (0.5 * clip0.xy / clip0.w + 0.5);
    let b = globals.resolution.xy * (0.5 * clip1.xy / clip1.w + 0.5);

    let x_basis = normalize(b - a);
    let y_basis = vec2<f32>(-x_basis.y, x_basis.x);

    let offset_a = a + a_width * (input.pos.x * x_basis + input.pos.y * y_basis);
    let offset_b = b + b_width * (input.pos.x * x_basis + input.pos.y * y_basis);

    let final_pos = mix(offset_a, offset_b, vec2<f32>(input.pos.z));

    let clip = mix(clip0, clip1, vec4<f32>(input.pos.z));

    out.pos = vec4<f32>(clip.w * ((2.0 * final_pos) / globals.resolution.xy - 1.0), clip.z, clip.w);
    out.dist = mix(input.length_so_far_a.x, input.length_so_far_b.x, input.pos.z);

    var plane = vec4<f32>(-1.0, 0.8, 1.2, 40.0);
    out.plane_distance = dot(plane, interpolated_pos);

    return out;
}

@fragment
fn main_fs(input: VertexOutput) -> @location(0) vec4<f32> {
    if input.plane_distance > 0.0 {
        discard;
    }

    let r = 0.0;
    let g = 0.0;
    let b = 0.0;

    let dash_size = globals.resolution.z;
    let gap_size = globals.resolution.w;

    if (fract(input.dist / (dash_size + gap_size)) > dash_size / (dash_size + gap_size)) {
        discard;
    }

    return vec4<f32>(r, g, b, 1.0);
}
