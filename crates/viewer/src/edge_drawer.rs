// A WGPU utility for drawing solid and dashed lines with variable thickness,
// round joins, and configurable dash and gap sizes.
//
// References:
// Instanced Line Rendering - https://wwwtyro.net/2019/11/18/instanced-lines.html
// Dashed Line Rendering - https://stackoverflow.com/a/54543267

use bytemuck::{Pod, Zeroable};
use glam::{vec4, Mat4, Vec3, Vec4};
use simple_game::graphics::GraphicsDevice;
use wgpu::util::DeviceExt;

struct Buffers {
    solid_vertex_uniform: wgpu::Buffer,
    dashed_vertex_uniform: wgpu::Buffer,
    round_strip_geometry: wgpu::Buffer,
    round_strip_geometry_len: usize,

    position_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

struct BindGroups {
    solid_vertex_uniform: wgpu::BindGroup,
    dashed_vertex_uniform: wgpu::BindGroup,
    static_vertex_bind_group: wgpu::BindGroup,
    dynamic_strip_buffer_bind_group: wgpu::BindGroup,
}

pub struct EdgeDrawer {
    solid_line_strip_pipeline: wgpu::RenderPipeline,
    dashed_line_strip_pipeline: wgpu::RenderPipeline,
    buffers: Buffers,
    bind_groups: BindGroups,
    screen_width: u32,
    screen_height: u32,
    draw_back_edges: bool,

    num_vertices: u32,
}

impl EdgeDrawer {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        let solid_line_strip_pipeline =
            Self::build_pipeline(device, target_format, depth_format, wgpu::CompareFunction::Less);

        let dashed_line_strip_pipeline = Self::build_pipeline(
            device,
            target_format,
            depth_format,
            wgpu::CompareFunction::Greater,
        );

        let buffers = Self::build_buffers(device);
        let bind_groups = Self::build_bind_groups(device, &solid_line_strip_pipeline, &buffers);

        Self {
            solid_line_strip_pipeline,
            dashed_line_strip_pipeline,
            buffers,
            bind_groups,
            screen_width,
            screen_height,
            draw_back_edges: false,
            num_vertices: 0,
        }
    }

    pub fn resize(&mut self, screen_width: u32, screen_height: u32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
    }

    pub fn toggle_back_edge_drawing(&mut self) {
        self.draw_back_edges = !self.draw_back_edges;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        rendered_line: &RenderedLine,
        render_pass: &mut wgpu::RenderPass,
        queue: &wgpu::Queue,
        camera_matrix: Mat4,
        transform: Mat4,
        dash_size: f32,
        gap_size: f32,
    ) {
        // Write dashed uniforms
        let mut uniforms = LineUniforms {
            proj: camera_matrix,
            transform,
            resolution: vec4(
                self.screen_width as f32,
                self.screen_height as f32,
                dash_size,
                gap_size,
            ),
        };

        queue.write_buffer(&self.buffers.dashed_vertex_uniform, 0, bytemuck::bytes_of(&uniforms));

        // Write solid uniforms
        uniforms.resolution.z = 0.0; // Dash size
        uniforms.resolution.w = 0.0; // Gap size

        queue.write_buffer(&self.buffers.solid_vertex_uniform, 0, bytemuck::bytes_of(&uniforms));

        render_pass.push_debug_group("Line drawer");
        {
            // let instance_buffer_size = rendered_line.vertex_buf.size();
            // let one_instance_size = std::mem::size_of::<LineVertex3>() as u64;

            // render_pass.set_vertex_buffer(0, self.buffers.round_strip_geometry.slice(..));
            // render_pass.set_vertex_buffer(
            //     0,
            //     rendered_line.vertex_buf.slice(..(instance_buffer_size - one_instance_size)),
            // );
            // render_pass.set_vertex_buffer(1, rendered_line.vertex_buf.slice(one_instance_size..));

            // Render dashed line strips
            // dbg!(&rendered_line.strip_instances);
            // if self.draw_back_edges {
            //     render_pass.set_pipeline(&self.dashed_line_strip_pipeline);
            //     render_pass.set_bind_group(0, &self.bind_groups.dashed_vertex_uniform, &[]);

            //     let mut offset = 0usize;
            //     let vertex_count = self.buffers.round_strip_geometry_len as u32;

            //     for line_strip_size in &rendered_line.line_sizes {
            //         let range = (offset as u32)..(offset + line_strip_size - 1) as u32;
            //         offset += line_strip_size;
            //         render_pass.draw(0..vertex_count, range);
            //     }
            // }

            // Render solid line strips
            render_pass.set_pipeline(&self.solid_line_strip_pipeline);
            render_pass.set_bind_group(0, &self.bind_groups.solid_vertex_uniform, &[]);
            render_pass.set_bind_group(1, &self.bind_groups.static_vertex_bind_group, &[]);
            render_pass.set_bind_group(2, &self.bind_groups.dynamic_strip_buffer_bind_group, &[]);

            // let mut offset = 0usize;
            // let vertex_count = self.buffers.round_strip_geometry_len as u32;

            // for line_strip_size in &rendered_line.line_sizes {
            //     let range = (offset as u32)..(offset + line_strip_size - 1) as u32;
            //     offset += line_strip_size;
            //     render_pass.draw(0..vertex_count, range);
            // }

            render_pass.draw(
                0..(self.num_vertices * self.buffers.round_strip_geometry_len as u32),
                0..(rendered_line.strip_instances.len() as u32),
            );
        }
        render_pass.pop_debug_group();
    }

    fn build_pipeline(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
        depth_compare: wgpu::CompareFunction,
    ) -> wgpu::RenderPipeline {
        let draw_shader =
            GraphicsDevice::load_wgsl_shader(device, include_str!("../shaders/edge.wgsl"));

        let vertex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<LineUniforms>() as u64,
                        ),
                    },
                    count: None,
                }],
                label: None,
            });

        let static_vertex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            });

        let dynamic_vertex_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                label: None,
            });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Round line strip renderer"),
                bind_group_layouts: &[
                    &vertex_bind_group_layout,
                    &static_vertex_bind_group_layout,
                    &dynamic_vertex_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &draw_shader,
                entry_point: Some("main_vs"),
                buffers: &[
                    // wgpu::VertexBufferLayout {
                    //     array_stride: std::mem::size_of::<RoundLineStripVertex>() as u64,
                    //     step_mode: wgpu::VertexStepMode::Vertex,
                    //     attributes: &wgpu::vertex_attr_array![
                    //         0 => Float32x3, // XY position of this particular vertex, with Z indicating sides.
                    //     ],
                    // },
                    // wgpu::VertexBufferLayout {
                    //     array_stride: std::mem::size_of::<LineVertex3>() as u64,
                    //     step_mode: wgpu::VertexStepMode::Instance,
                    //     attributes: &wgpu::vertex_attr_array![
                    //         0 => Float32x4, // Point A
                    //         1 => Float32x4, // Length so far
                    //     ],
                    // },
                    // wgpu::VertexBufferLayout {
                    //     array_stride: std::mem::size_of::<LineVertex3>() as u64,
                    //     step_mode: wgpu::VertexStepMode::Instance,
                    //     attributes: &wgpu::vertex_attr_array![
                    //         2 => Float32x4, // Point B
                    //         3 => Float32x4, // Length so far
                    //     ],
                    // },
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &draw_shader,
                entry_point: Some("main_fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Front), // TODO - figure out culling
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare,
                stencil: wgpu::StencilState::default(),
                // TODO(bschwind) - Allow configuration of depth bias.
                bias: wgpu::DepthBiasState { constant: -50, slope_scale: 0.0, clamp: 0.0 },
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        })
    }

    fn build_bind_groups(
        device: &wgpu::Device,
        pipeline: &wgpu::RenderPipeline,
        buffers: &Buffers,
    ) -> BindGroups {
        let uniform_bind_group_layout = pipeline.get_bind_group_layout(0);
        let static_vertex_bind_group_layout = pipeline.get_bind_group_layout(1);
        let vertex_pos_bind_group_layout = pipeline.get_bind_group_layout(2);

        let solid_vertex_uniform = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.solid_vertex_uniform.as_entire_binding(),
            }],
            label: None,
        });

        let dashed_vertex_uniform = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.dashed_vertex_uniform.as_entire_binding(),
            }],
            label: None,
        });

        let static_vertex_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &static_vertex_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffers.round_strip_geometry.as_entire_binding(),
            }],
            label: None,
        });

        let dynamic_strip_buffer_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &vertex_pos_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffers.position_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: buffers.instance_buffer.as_entire_binding(),
                    },
                ],
                label: None,
            });

        BindGroups {
            solid_vertex_uniform,
            dashed_vertex_uniform,
            static_vertex_bind_group,
            dynamic_strip_buffer_bind_group,
        }
    }

    fn build_buffers(device: &wgpu::Device) -> Buffers {
        const CIRCLE_RESOLUTION: usize = 30;

        // Uniform buffer
        let line_uniforms = LineUniforms::default();

        let solid_vertex_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line drawer vertex shader uniform buffer"),
            contents: bytemuck::bytes_of(&line_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let dashed_vertex_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line drawer vertex shader uniform buffer"),
            contents: bytemuck::bytes_of(&line_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Round strip geometry
        let mut round_strip_vertices = vec![
            RoundLineStripVertex { pos: [0.0, -0.5, 0.0] },
            RoundLineStripVertex { pos: [0.0, 0.5, 0.0] },
            RoundLineStripVertex { pos: [0.0, 0.5, 1.0] },
            RoundLineStripVertex { pos: [0.0, -0.5, 0.0] },
            RoundLineStripVertex { pos: [0.0, 0.5, 1.0] },
            RoundLineStripVertex { pos: [0.0, -0.5, 1.0] },
        ];

        // Left circle cap
        for i in 0..CIRCLE_RESOLUTION {
            let frac_1 = (std::f32::consts::PI / 2.0)
                + (i as f32 / CIRCLE_RESOLUTION as f32) * std::f32::consts::PI;
            let frac_2 = (std::f32::consts::PI / 2.0)
                + ((i + 1) as f32 / CIRCLE_RESOLUTION as f32) * std::f32::consts::PI;

            round_strip_vertices.push(RoundLineStripVertex { pos: [0.0, 0.0, 0.0] });
            round_strip_vertices
                .push(RoundLineStripVertex { pos: [0.5 * frac_2.cos(), 0.5 * frac_2.sin(), 0.0] });
            round_strip_vertices
                .push(RoundLineStripVertex { pos: [0.5 * frac_1.cos(), 0.5 * frac_1.sin(), 0.0] });
        }

        // Right circle cap
        for i in 0..CIRCLE_RESOLUTION {
            let frac_1 = (3.0 * std::f32::consts::PI / 2.0)
                + (i as f32 / CIRCLE_RESOLUTION as f32) * std::f32::consts::PI;
            let frac_2 = (3.0 * std::f32::consts::PI / 2.0)
                + ((i + 1) as f32 / CIRCLE_RESOLUTION as f32) * std::f32::consts::PI;

            round_strip_vertices.push(RoundLineStripVertex { pos: [0.0, 0.0, 1.0] });
            round_strip_vertices
                .push(RoundLineStripVertex { pos: [0.5 * frac_2.cos(), 0.5 * frac_2.sin(), 1.0] });
            round_strip_vertices
                .push(RoundLineStripVertex { pos: [0.5 * frac_1.cos(), 0.5 * frac_1.sin(), 1.0] });
        }

        // TODO(bschwind) - set this as a storage buffer
        let round_strip_geometry = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Round line segment geometry buffer"),
            contents: bytemuck::cast_slice(&round_strip_vertices),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

        let position_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Edge Drawer vertex position buffer"),
            size: 1, // Needs to be at least one, or else create_bind_group panics.
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Edge Drawer instance buffer"),
            size: 1, // Needs to be at least one, or else create_bind_group panics.
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Buffers {
            solid_vertex_uniform,
            dashed_vertex_uniform,
            round_strip_geometry,
            round_strip_geometry_len: round_strip_vertices.len(),
            position_buffer,
            instance_buffer,
        }
    }

    pub fn update_line_buffer(&mut self, device: &wgpu::Device, line_builder: LineBuilder) {
        self.num_vertices = line_builder.round_line_strips.len() as u32;

        self.buffers.position_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Updated line position buffer"),
                contents: bytemuck::cast_slice(&line_builder.round_line_strips),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        self.buffers.instance_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Updated instance buffer"),
                contents: bytemuck::cast_slice(&line_builder.strip_instances),
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            });

        let vertex_pos_bind_group_layout = self.solid_line_strip_pipeline.get_bind_group_layout(2);

        // Update our bind group
        dbg!("updating bind group");
        self.bind_groups.dynamic_strip_buffer_bind_group =
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &vertex_pos_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.buffers.position_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: self.buffers.instance_buffer.as_entire_binding(),
                    },
                ],
                label: None,
            });
    }
}

pub struct LineBuilder {
    round_line_strips: Vec<LineVertex3>,
    round_line_strip_indices: Vec<usize>,
    strip_instances: Vec<LineStripInstance>,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable)]
struct LineStripInstance {
    start_index: u32,
    count: u32,
}

impl LineBuilder {
    pub fn new() -> Self {
        Self {
            round_line_strips: Vec::new(),
            round_line_strip_indices: Vec::new(),
            strip_instances: Vec::new(),
        }
    }

    /// A special-case where round line joins and caps are desired. This can be achieved
    /// with a single draw call.
    pub fn add_round_line_strip(&mut self, positions: &[LineVertex3]) {
        self.strip_instances.push(LineStripInstance {
            start_index: self.round_line_strips.len() as u32,
            count: positions.len() as u32,
        });

        self.round_line_strips.extend_from_slice(positions);
        self.round_line_strip_indices.push(positions.len());
    }

    pub fn build(self, device: &wgpu::Device) -> RenderedLine {
        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rendered line vertex buffer"),
            contents: bytemuck::cast_slice(&self.round_line_strips),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let line_sizes = self.round_line_strip_indices;

        RenderedLine { vertex_buf, line_sizes, strip_instances: self.strip_instances }
    }
}

#[derive(Debug)]
pub struct RenderedLine {
    vertex_buf: wgpu::Buffer,
    line_sizes: Vec<usize>,
    strip_instances: Vec<LineStripInstance>,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable)]
struct LineUniforms {
    proj: Mat4,
    transform: Mat4,
    resolution: Vec4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct LineVertex3 {
    /// XYZ position of the line vertex, W = line thickness
    pos: Vec4,
    length_so_far: Vec4,
}

impl LineVertex3 {
    pub fn new(pos: Vec3, thickness: f32, length_so_far: f32) -> Self {
        Self {
            pos: vec4(pos.x, pos.y, pos.z, thickness),
            length_so_far: vec4(length_so_far, 0.0, 0.0, 0.0),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct RoundLineStripVertex {
    /// XY position of the line vertex, with Z indicating:
    /// 0: The left part of the line segment.
    /// 1: The right part of the line segment.
    pos: [f32; 3],
}
