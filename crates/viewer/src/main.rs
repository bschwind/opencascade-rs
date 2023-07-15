use crate::{
    edge_drawer::{EdgeDrawer, LineBuilder, LineVertex3, RenderedLine},
    surface_drawer::{CadMesh, SurfaceDrawer},
};
use glam::{dvec3, vec3, DVec3, Mat4};
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{Face, Shape, Solid, Wire},
    workplane::Workplane,
};
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

mod edge_drawer;
mod surface_drawer;

struct ViewerApp {
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
        let keycap = Shape::read_step("crates/viewer/models/nist_ftc_06.step");

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
            },
            WindowEvent::KeyboardInput {
                input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::Escape), .. },
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => {},
        }
    }

    fn tick(&mut self, _dt: f32) {}

    fn render(&mut self, graphics_device: &mut GraphicsDevice, _window: &Window) {
        let mut frame_encoder = graphics_device.begin_frame();
        let (width, height) = frame_encoder.surface_dimensions();

        let smaa_render_target = self.smaa_target.start_frame(
            graphics_device.device(),
            graphics_device.queue(),
            &frame_encoder.backbuffer_view,
        );

        let camera_matrix = build_camera_matrix(width, height);
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
            build_camera_matrix(width, height),
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

fn build_camera_matrix(width: u32, height: u32) -> Mat4 {
    let aspect_ratio = width as f32 / height as f32;
    let proj = Mat4::perspective_rh(std::f32::consts::PI / 2.0, aspect_ratio, 0.01, 1000.0);

    let view = Mat4::look_at_rh(
        vec3(20.0, -30.0, 20.0), // Eye position
        vec3(0.0, 0.0, 0.0),     // Look-at target
        vec3(0.0, 0.0, 1.0),     // Up vector of the camera
    );

    proj * view
}

#[allow(unused)]
fn cube() -> Shape {
    let rect = Wire::rect(10.0, 10.0);
    let face = Face::from_wire(&rect);

    face.extrude(dvec3(0.0, 0.0, 10.0)).to_shape()
}

#[allow(unused)]
fn keycap() -> Shape {
    const KEYCAP_PITCH: f64 = 19.05;

    let convex = false;
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let height = 16.0;
    let angle = 13.0.degrees();
    let depth: f64 = 2.8;
    let thickness: f64 = 1.5;
    let base = 18.2;
    let top = 13.2;
    let curve = 1.7;
    let bottom_fillet = 0.5;
    let top_fillet = 5.0;
    let tension = if convex { 0.4 } else { 1.0 };

    let top_diff = base - top;

    let bx = KEYCAP_PITCH * keycap_unit_size_x - (KEYCAP_PITCH - base);
    let by = KEYCAP_PITCH * keycap_unit_size_y - (KEYCAP_PITCH - base);

    let tx = bx - top_diff;
    let ty = by - top_diff;

    let mut base = Workplane::xy().rect(bx, by);
    base.fillet(bottom_fillet);

    let mut mid = Workplane::xy().rect(bx, by);
    mid.fillet((top_fillet - bottom_fillet) / 3.0);
    mid.transform(dvec3(0.0, 0.0, height / 4.0), dvec3(1.0, 0.0, 0.0), angle / 4.0);

    // We should use `ConnectEdgesToWires` for `Wire::from_edges`, as it
    // likely puts these arcs in the order we want.
    let mut top_wire = Workplane::xy()
        .sketch()
        .arc((curve, curve * tension), (0.0, ty / 2.0), (curve, ty - curve * tension))
        .arc((curve, ty - curve * tension), (tx / 2.0, ty), (tx - curve, ty - curve * tension))
        .arc((tx - curve, ty - curve * tension), (tx, ty / 2.0), (tx - curve, curve * tension))
        .arc((tx - curve, curve * tension), (tx / 2.0, 0.0), (curve, curve * tension))
        .wire();

    top_wire.fillet(top_fillet);
    top_wire.translate(dvec3(-tx / 2.0, -ty / 2.0, 0.0));
    top_wire.transform(dvec3(0.0, 0.0, height), dvec3(1.0, 0.0, 0.0), angle);

    let mut keycap = Solid::loft([&base, &mid, &top_wire].into_iter());

    let scoop = if convex {
        let scoop = Workplane::yz()
            .transformed(dvec3(0.0, height - 2.1, -bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0, -1.0)
            .three_point_arc((0.0, 2.0), (by / 2.0, -1.0))
            .line_to(by / 2.0, 10.0)
            .line_to(-by / 2.0, 10.0)
            .close();

        let scoop = Face::from_wire(&scoop);
        scoop.extrude(dvec3(bx, 0.0, 0.0))
    } else {
        let scoop_right = Workplane::yz()
            .transformed(dvec3(0.0, height, bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 + 2.0, 0.0)
            .three_point_arc((0.0, (-depth + 1.5).min(-0.1)), (by / 2.0 - 2.0, 0.0))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_mid = Workplane::yz()
            .transformed(dvec3(0.0, height, 0.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 - 2.0, -0.5)
            .three_point_arc((0.0, -depth), (by / 2.0 + 2.0, -0.5))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_left = Workplane::yz()
            .transformed(dvec3(0.0, height, -bx / 2.0), RVec::z(angle))
            .sketch()
            .move_to(-by / 2.0 + 2.0, 0.0)
            .three_point_arc((0.0, (-depth + 1.5).min(-0.1)), (by / 2.0 - 2.0, 0.0))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        Solid::loft([&scoop_right, &scoop_mid, &scoop_left].into_iter())
    };

    let (mut keycap, edges) = keycap.subtract(&scoop);
    keycap.fillet_edges(0.6, &edges);

    let shell_bottom = Workplane::xy().rect(bx - thickness * 2.0, by - thickness * 2.0);

    let shell_mid = Workplane::xy()
        .translated(dvec3(0.0, 0.0, height / 4.0))
        .rect(bx - thickness * 3.0, by - thickness * 3.0);

    let shell_top = Workplane::xy()
        .transformed(dvec3(0.0, 0.0, height - height / 4.0 - 4.5), RVec::x(angle))
        .rect(tx - thickness * 2.0 + 0.5, ty - thickness * 2.0 + 0.5);

    let shell = Solid::loft([&shell_bottom, &shell_mid, &shell_top].into_iter());

    let (keycap, _edges) = keycap.subtract(&shell);

    keycap
}

#[allow(unused)]
fn gizmo() -> Shape {
    let arrow_length = 10.0;
    let cone_height = 2.0;
    let shaft_length = arrow_length - cone_height;

    let arrow = |workplane: Workplane| {
        let shaft =
            workplane.circle(0.0, 0.0, 0.1).to_face().extrude(workplane.normal() * arrow_length);
        let cone_base =
            workplane.translated(DVec3::new(0.0, 0.0, shaft_length)).circle(0.0, 0.0, 1.0);
        let cone_top =
            workplane.translated(DVec3::new(0.0, 0.0, arrow_length)).circle(0.0, 0.0, 0.05);
        let cone = Solid::loft([&cone_base, &cone_top].into_iter());
        let (arrow_shape, _) = shaft.union(&cone);

        arrow_shape
    };

    // TODO(bschwind) - Make it easier to chain union operations together.
    arrow(Workplane::yz())
        .union_shape(&arrow(Workplane::xz()))
        .0
        .union_shape(&arrow(Workplane::xy()))
        .0
}

fn main() {
    simple_game::run_game_app::<ViewerApp>();
}
