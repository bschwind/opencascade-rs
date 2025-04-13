struct Globals {
    proj: mat4x4<f32>,
    transform: mat4x4<f32>,
    resolution: vec4<f32>, // X = screen width, Y = screen height, Z = dash_size, W = gap_size
};

struct StaticVertex {
    pos: vec3<f32>
};

struct SegmentVertex {
    pos: vec4<f32>,
    length_so_far: vec4<f32>,
}

struct StripInstance {
    start_index: u32,
    count: u32,
};

// Uniforms
@group(0) @binding(0)
var<uniform> globals: Globals;

@group(1) @binding(0)
var<storage, read> static_geometry: array<StaticVertex>;

@group(2) @binding(0)
var<storage, read> points: array<SegmentVertex>;
@group(2) @binding(1)
var<storage, read> instances: array<StripInstance>;

struct VertexInput {
    // // Per-vertex data
    // @location(0)
    // pos: vec3<f32>,

    @builtin(vertex_index)
    vertex_index: u32,

    @builtin(instance_index)
    instance_index: u32,

    // // Per-instance data
    // @location(0)
    // point_a: vec4<f32>,

    // @location(1)
    // length_so_far_a: vec4<f32>,

    // @location(2)
    // point_b: vec4<f32>,

    // @location(3)
    // length_so_far_b: vec4<f32>,
};

struct VertexOutput {
    @builtin(position)
    pos: vec4<f32>,

    @location(0)
    dist: f32,
};

@vertex
fn main_vs(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // TODO - use vertex_index to find correct position
    let vertex_array_len = arrayLength(&static_geometry);
    let position_index = input.vertex_index % vertex_array_len;
    let point_index = input.vertex_index / vertex_array_len;

    let instance = instances[input.instance_index];

    if point_index >= instance.start_index + instance.count - 1 {
        out.pos = vec4<f32>(2.0, 2.0, 0.0, 1.0);
        out.dist = 0.0;
        return out;
    }

    if point_index < instance.start_index {
        out.pos = vec4<f32>(2.0, 2.0, 0.0, 1.0);
        out.dist = 0.0;
        return out;
    }

    let p1 = points[point_index];
    let p2 = points[point_index + 1];



    let a_width = p1.pos.w;
    let b_width = p2.pos.w;

    // Transform the segment endpoints to clip space
    let clip0 = globals.proj * globals.transform * vec4<f32>(p1.pos.xyz, 1.0);
    let clip1 = globals.proj * globals.transform * vec4<f32>(p2.pos.xyz, 1.0);

    // Transform the segment endpoints to screen space
    let a = globals.resolution.xy * (0.5 * clip0.xy / clip0.w + 0.5);
    let b = globals.resolution.xy * (0.5 * clip1.xy / clip1.w + 0.5);

    let x_basis = normalize(b - a);
    let y_basis = vec2<f32>(-x_basis.y, x_basis.x);

    
    // let input_pos = vec3<f32>(f32(input.vertex_index), f32(input.instance_index), 0.0);
    let input_pos = static_geometry[position_index].pos;

    let offset_a = a + a_width * (input_pos.x * x_basis + input_pos.y * y_basis);
    let offset_b = b + b_width * (input_pos.x * x_basis + input_pos.y * y_basis);

    let final_pos = mix(offset_a, offset_b, vec2<f32>(input_pos.z));

    let clip = mix(clip0, clip1, vec4<f32>(input_pos.z));

    out.pos = vec4<f32>(clip.w * ((2.0 * final_pos) / globals.resolution.xy - 1.0), clip.z, clip.w);
    out.dist = mix(p1.length_so_far.x, p2.length_so_far.x, input_pos.z);

    return out;
}

@fragment
fn main_fs(input: VertexOutput) -> @location(0) vec4<f32> {
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
