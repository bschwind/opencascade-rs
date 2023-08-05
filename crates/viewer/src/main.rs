use crate::{
    edge_drawer::{EdgeDrawer, LineBuilder, LineVertex3, RenderedLine},
    surface_drawer::{CadMesh, SurfaceDrawer},
};
use glam::{vec3, DVec3, Mat4};
use simple_game::{
    graphics::{
        text::{AxisAlign, StyledText, TextAlignment, TextSystem},
        DepthTexture, GraphicsDevice,
    },
    util::FPSCounter,
    GameApp,
};
use smaa::{SmaaMode, SmaaTarget};
use winit::{
    event::{KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

mod camera;
mod edge_drawer;
mod surface_drawer;

const MIN_SCALE: f32 = 0.01;

struct ViewerApp {
    camera: camera::Camera,
    depth_texture: DepthTexture,
    text_system: TextSystem,
    fps_counter: FPSCounter,
    line_drawer: EdgeDrawer,
    surface_drawer: SurfaceDrawer,
    smaa_target: SmaaTarget,
    rendered_edges: RenderedLine,
    cad_mesh: CadMesh,
    angle: f32,
    scale: f32,
}

impl GameApp for ViewerApp {
    fn window_title() -> &'static str {
        "Viewer"
    }

    fn init(graphics_device: &mut GraphicsDevice) -> Self {
        // Model sourced from:
        // https://nist.gov/ctl/smart-connected-systems-division/smart-connected-manufacturing-systems-group/mbe-pmi-0
        // let keycap = Shape::read_step("crates/viewer/models/nist_ftc_06.step").unwrap();
        let keycap = examples::flywheel::shape();

        let mesh = keycap.mesh();
        let cad_mesh = CadMesh::from_mesh(&mesh, graphics_device.device());

        // Pre-render the model edges.
        let line_thickness = 3.0;
        let mut line_builder = LineBuilder::new();

        for edge in keycap.edges() {
            let mut segments = vec![];

            let mut last_point: Option<DVec3> = None;
            let mut length_so_far = 0.0;

            for point in edge.approximation_segments() {
                if let Some(last_point) = last_point {
                    length_so_far += (point - last_point).length();
                }

                segments.push(LineVertex3::new(
                    vec3(point.x as f32, point.y as f32, point.z as f32),
                    line_thickness,
                    length_so_far as f32,
                ));

                last_point = Some(point);
            }

            line_builder.add_round_line_strip(&segments);
        }

        let rendered_edges = line_builder.build(graphics_device.device());

        // Create SMAA target
        let (width, height) = graphics_device.surface_dimensions();
        let device = graphics_device.device();
        let queue = graphics_device.queue();
        let swapchain_format = graphics_device.surface_config().format;

        let smaa_target =
            SmaaTarget::new(device, queue, width, height, swapchain_format, SmaaMode::Smaa1X);

        let surface_texture_format = graphics_device.surface_texture_format();

        let depth_texture = DepthTexture::new(device, width, height);
        let depth_texture_format = depth_texture.format();

        Self {
            camera: camera::Camera::new(width, height),
            depth_texture,
            text_system: TextSystem::new(device, surface_texture_format, width, height),
            fps_counter: FPSCounter::new(),
            line_drawer: EdgeDrawer::new(
                device,
                surface_texture_format,
                depth_texture_format,
                width,
                height,
            ),
            surface_drawer: SurfaceDrawer::new(
                device,
                surface_texture_format,
                depth_texture_format,
            ),
            smaa_target,
            cad_mesh,
            rendered_edges,
            angle: 0.0,
            scale: 1.0,
        }
    }

    fn resize(&mut self, graphics_device: &mut GraphicsDevice, width: u32, height: u32) {
        self.camera.resize(width, height);
        self.depth_texture = DepthTexture::new(graphics_device.device(), width, height);
        self.text_system.resize(width, height);
        self.line_drawer.resize(width, height);
        self.smaa_target.resize(graphics_device.device(), width, height);
    }

    fn handle_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::TouchpadRotate { delta, .. } => {
                self.angle += 2.0 * delta * std::f32::consts::PI / 180.0;
            },
            WindowEvent::TouchpadMagnify { delta, .. } => {
                self.scale += *delta as f32;
                self.scale = self.scale.max(MIN_SCALE);
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput { virtual_keycode: Some(keycode), .. },
                ..
            } => match keycode {
                VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                VirtualKeyCode::P => self.camera.use_perspective(),
                VirtualKeyCode::O => self.camera.use_orthographic(),
                _ => {},
            },
            _ => {},
        }
    }

    fn tick(&mut self, _dt: f32) {}

    fn render(&mut self, graphics_device: &mut GraphicsDevice, _window: &Window) {
        let mut frame_encoder = graphics_device.begin_frame();

        let smaa_render_target = self.smaa_target.start_frame(
            graphics_device.device(),
            graphics_device.queue(),
            &frame_encoder.backbuffer_view,
        );

        let camera_matrix = self.camera.matrix();
        let transform = Mat4::from_rotation_z(self.angle)
            * Mat4::from_scale(vec3(self.scale, self.scale, self.scale));

        self.surface_drawer.render(
            &mut frame_encoder.encoder,
            &smaa_render_target,
            &self.depth_texture.view,
            graphics_device.queue(),
            &self.cad_mesh,
            camera_matrix,
            transform,
        );

        let dash_size = 0.5;
        let gap_size = 0.5;

        self.line_drawer.draw(
            &self.rendered_edges,
            &mut frame_encoder.encoder,
            &smaa_render_target,
            Some(&self.depth_texture.view),
            graphics_device.queue(),
            camera_matrix,
            transform,
            dash_size,
            gap_size,
        );

        self.text_system.render_horizontal(
            TextAlignment {
                x: AxisAlign::Start(10),
                y: AxisAlign::Start(10),
                max_width: None,
                max_height: None,
            },
            &[StyledText::default_styling(&format!("FPS: {}", self.fps_counter.fps()))],
            &mut frame_encoder.encoder,
            &smaa_render_target,
            graphics_device.queue(),
        );

        graphics_device.queue().submit(Some(frame_encoder.encoder.finish()));

        smaa_render_target.resolve();
        frame_encoder.frame.present();

        self.fps_counter.tick();
    }
}

fn main() {
    simple_game::run_game_app::<ViewerApp>();
}
