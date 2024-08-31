use bytemuck::{Pod, Zeroable};
use glam::Mat4;
use opencascade::mesh::Mesh;
use simple_game::graphics::GraphicsDevice;
use wgpu::{self, util::DeviceExt, Buffer, RenderPipeline};

pub struct SurfaceDrawer {
    vertex_uniform: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    pipeline: RenderPipeline,
}

impl SurfaceDrawer {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        depth_format: wgpu::TextureFormat,
    ) -> Self {
        // Uniform buffer
        let cad_mesh_uniforms = CadMeshUniforms::default();

        let vertex_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Line drawer vertex shader uniform buffer"),
            contents: bytemuck::bytes_of(&cad_mesh_uniforms),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("CadMesh bind group layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<CadMeshUniforms>() as u64
                    ),
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("CadMesh pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buffers = &[wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<CadMeshVertex>()) as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![
                0 => Float32x3, // pos
                1 => Float32x2, // uv
                2 => Float32x3, // normal
            ],
        }];

        let draw_shader =
            GraphicsDevice::load_wgsl_shader(device, include_str!("../shaders/surface.wgsl"));

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("CadMesh render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &draw_shader,
                entry_point: "vs_main",
                buffers: vertex_buffers,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                // strip_index_format: Some(wgpu::IndexFormat::Uint32),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                ..wgpu::PrimitiveState::default()
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &draw_shader,
                entry_point: "fs_main",
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
            multiview: None,
            cache: None,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: vertex_uniform.as_entire_binding(),
            }],
            label: None,
        });

        Self { vertex_uniform, uniform_bind_group, pipeline }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        depth_view: &wgpu::TextureView,
        queue: &wgpu::Queue,
        cad_mesh: &CadMesh,
        camera_matrix: Mat4,
        transform: Mat4,
    ) {
        let uniforms = CadMeshUniforms { proj: camera_matrix, transform };

        queue.write_buffer(&self.vertex_uniform, 0, bytemuck::bytes_of(&uniforms));

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("CadMesh render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: render_target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_index_buffer(cad_mesh.index_buf.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.set_vertex_buffer(0, cad_mesh.vertex_buf.slice(..));
        render_pass.draw_indexed(0..(cad_mesh.num_indices as u32), 0, 0..1);
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable)]
struct CadMeshUniforms {
    proj: Mat4,
    transform: Mat4,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct CadMeshVertex {
    pos: [f32; 3],
    uv: [f32; 2],
    normal: [f32; 3],
}

pub struct CadMesh {
    num_indices: usize,
    vertex_buf: Buffer,
    index_buf: Buffer,
}

impl CadMesh {
    pub fn from_mesh(mesh: &Mesh, device: &wgpu::Device) -> Self {
        let vertex_data: Vec<_> = mesh
            .vertices
            .iter()
            .zip(mesh.uvs.iter())
            .zip(mesh.normals.iter())
            .map(|((v, uv), normal)| CadMeshVertex {
                pos: [v.x as f32, v.y as f32, v.z as f32],
                uv: [uv.x as f32, uv.y as f32],
                normal: [normal.x as f32, normal.y as f32, normal.z as f32],
            })
            .collect();

        let index_data: Vec<_> = mesh.indices.iter().map(|i| *i as u32).collect();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CadMesh Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("CadMesh Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self { num_indices: mesh.indices.len(), vertex_buf, index_buf }
    }
}
