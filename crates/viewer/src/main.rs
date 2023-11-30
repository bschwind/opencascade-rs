use crate::{
    edge_drawer::{EdgeDrawer, LineBuilder, LineVertex3, RenderedLine},
    surface_drawer::{CadMesh, SurfaceDrawer},
};
use anyhow::Error;
use camera::OrbitCamera;
use clap::{Parser, ValueEnum};
use glam::{vec2, vec3, DVec3, Mat4, Quat, Vec2, Vec3};
use opencascade::primitives::Shape;
use simple_game::{
    graphics::{
        text::{AxisAlign, StyledText, TextAlignment, TextSystem},
        DepthTexture, GraphicsDevice,
    },
    util::FPSCounter,
    GameApp,
};
use smaa::{SmaaMode, SmaaTarget};
use std::path::PathBuf;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta::PixelDelta, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

mod camera;
mod edge_drawer;
mod surface_drawer;

// Multipliers to convert mouse position deltas to a more intuitve camera perspective change.
const ZOOM_MULTIPLIER: f32 = 5.0;
const TOUCHPAD_ZOOM_MULTIPLIER: f32 = 0.5;
const ROTATE_MULTIPLIER: f32 = 8.0;
const TOUCHPAD_ROTATE_MULTIPLIER: f32 = -0.05;
const PAN_MULTIPLIER: f32 = 150.0;
const TOUCHPAD_PAN_MULTIPLIER: f32 = 100.0;

#[derive(Default)]
struct MouseState {
    left_button_down: bool,
    middle_button_down: bool,
    right_button_down: bool,
    last_position: PhysicalPosition<f64>,
}

impl MouseState {
    fn delta(&mut self, position: PhysicalPosition<f64>) -> (f64, f64) {
        let delta = (position.x - self.last_position.x, position.y - self.last_position.y);
        self.last_position = position;
        delta
    }

    fn input(&mut self, button: MouseButton, state: ElementState) {
        match button {
            MouseButton::Left => self.left_button_down = state == ElementState::Pressed,
            MouseButton::Middle => self.middle_button_down = state == ElementState::Pressed,
            MouseButton::Right => self.right_button_down = state == ElementState::Pressed,
            _ => {},
        }
    }
}

struct ViewerApp {
    client_rect: Vec2,
    camera: OrbitCamera,
    depth_texture: DepthTexture,
    text_system: TextSystem,
    fps_counter: FPSCounter,
    line_drawer: EdgeDrawer,
    surface_drawer: SurfaceDrawer,
    smaa_target: SmaaTarget,
    rendered_edges: RenderedLine,
    cad_mesh: CadMesh,
    mouse_state: MouseState,
}

#[derive(Parser, Debug, Clone)]
struct AppArgs {
    #[arg(long, group = "model")]
    step_file: Option<PathBuf>,

    #[arg(long, value_enum, group = "model")]
    example: Option<Example>,
}

#[derive(Debug, Copy, Clone, PartialEq, ValueEnum)]
enum Example {
    Airfoil,
    BoxShape,
    Chamfer,
    Gizmo,
    HighLevelBottle,
    KeyboardCase,
    Keycap,
    Offset2d,
    RoundedChamfer,
    SweptFace,
    SweptWire,
    TurnersCube,
    VariableFillet,
}

impl Example {
    pub fn shape(self) -> Shape {
        match self {
            Example::Airfoil => examples::airfoil::shape(),
            Example::BoxShape => examples::box_shape::shape(),
            Example::Chamfer => examples::chamfer::shape(),
            Example::Gizmo => examples::gizmo::shape(),
            Example::HighLevelBottle => examples::high_level_bottle::shape(),
            Example::KeyboardCase => examples::keyboard_case::shape(),
            Example::Keycap => examples::keycap::shape(),
            Example::Offset2d => examples::offset_2d::shape(),
            Example::RoundedChamfer => examples::rounded_chamfer::shape(),
            Example::SweptFace => examples::swept_face::shape(),
            Example::SweptWire => examples::swept_wire::shape(),
            Example::TurnersCube => examples::turners_cube::shape(),
            Example::VariableFillet => examples::variable_fillet::shape(),
        }
    }
}

impl GameApp for ViewerApp {
    fn window_title() -> &'static str {
        "Viewer"
    }

    fn init(graphics_device: &mut GraphicsDevice) -> Self {
        let args = AppArgs::parse();

        let shape = if let Some(step_file) = args.step_file {
            Shape::read_step(step_file).expect("Failed to read STEP file, {step_file}")
        } else if let Some(example) = args.example {
            example.shape()
        } else {
            eprintln!("Warning - no example or STEP file specified, you get a default cube.");
            Shape::cube_centered(50.0)
        };

        let mesh = shape.mesh().expect("example shape should yield a valid triangulation");
        let cad_mesh = CadMesh::from_mesh(&mesh, graphics_device.device());

        // Pre-render the model edges.
        let line_thickness = 3.0;
        let mut line_builder = LineBuilder::new();

        for edge in shape.edges() {
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
            client_rect: vec2(width as f32, height as f32),
            camera: OrbitCamera::new(width, height, Vec3::new(40.0, -40.0, 20.0)),
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
            mouse_state: Default::default(),
        }
    }

    fn resize(&mut self, graphics_device: &mut GraphicsDevice, width: u32, height: u32) {
        self.client_rect = vec2(width as f32, height as f32);
        self.camera.resize(width, height);
        self.depth_texture = DepthTexture::new(graphics_device.device(), width, height);
        self.text_system.resize(width, height);
        self.line_drawer.resize(width, height);
        self.smaa_target.resize(graphics_device.device(), width, height);
    }

    fn handle_window_event(
        &mut self,
        event: &WindowEvent,
        window_target: &EventLoopWindowTarget<()>,
    ) {
        let screen_diagonal = self.client_rect.length();

        match event {
            WindowEvent::TouchpadRotate { delta, .. } => {
                let axis = Vec3::new(0.0, 0.0, 1.0);
                let rotator = Quat::from_axis_angle(axis, TOUCHPAD_ROTATE_MULTIPLIER * delta);
                self.camera.rotate(rotator);
            },
            WindowEvent::CursorMoved { position, .. } => {
                let delta = self.mouse_state.delta(*position);
                // On the screen, Y is DOWN, but in camera space, it's UP
                let camera_space_delta =
                    Vec2::new(delta.0 as f32, -delta.1 as f32) / screen_diagonal;
                if self.mouse_state.left_button_down {
                    // Construct the camera space rotation axis perpendicular to delta
                    let axis = Vec3::new(camera_space_delta.y, -camera_space_delta.x, 0.0);
                    let magnitude = axis.length();
                    if magnitude > 0.0 {
                        let rotator =
                            Quat::from_axis_angle(axis.normalize(), ROTATE_MULTIPLIER * magnitude);
                        self.camera.rotate(rotator);
                    }
                }
                if self.mouse_state.middle_button_down {
                    self.camera.pan(PAN_MULTIPLIER * camera_space_delta);
                }
                if self.mouse_state.right_button_down {
                    self.camera.zoom(camera_space_delta.y * ZOOM_MULTIPLIER);
                }
            },
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse_state.input(*button, *state)
            },
            WindowEvent::MouseWheel { delta: PixelDelta(delta), .. } => {
                // winit can not distinguish mouse wheel and touchpad pan events unfortunately.
                // Because of that, we assign pan operation to MouseWheel events. For mice, you
                // need to instead use mouse move while holding down the right button.

                // On the screen, Y is DOWN, but in camera space, it's UP
                let camera_space_delta =
                    Vec2::new(delta.x as f32, -delta.y as f32) / screen_diagonal;

                self.camera.pan(TOUCHPAD_PAN_MULTIPLIER * camera_space_delta);
            },
            WindowEvent::TouchpadMagnify { delta, .. } => {
                let zoom_delta = *delta as f32 * TOUCHPAD_ZOOM_MULTIPLIER;
                self.camera.zoom(-zoom_delta);
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(key_code), .. },
                ..
            } => match key_code {
                KeyCode::Escape => window_target.exit(),
                KeyCode::KeyP => self.camera.use_perspective(),
                KeyCode::KeyO => self.camera.use_orthographic(),
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
        let transform = Mat4::IDENTITY;

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

fn main() -> Result<(), Error> {
    simple_game::run_game_app::<ViewerApp>()?;
    Ok(())
}
