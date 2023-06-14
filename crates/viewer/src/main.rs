use crate::surface_drawer::{CadMesh, SurfaceDrawer};
use glam::{dvec3, vec3, Mat4};
use opencascade::{
    primitives::{Face, Shape, Solid, Wire},
    workplane::Workplane,
};
use simple_game::{
    graphics::{
        text::{AxisAlign, StyledText, TextAlignment, TextSystem},
        DepthTexture, FullscreenQuad, GraphicsDevice, LineDrawer, LineVertex3,
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

mod surface_drawer;

struct ViewerApp {
    depth_texture: DepthTexture,
    fullscreen_quad: FullscreenQuad,
    text_system: TextSystem,
    fps_counter: FPSCounter,
    line_drawer: LineDrawer,
    surface_drawer: SurfaceDrawer,
    smaa_target: SmaaTarget,
    model_edges: Vec<Vec<LineVertex3>>,
    cad_mesh: CadMesh,
    angle: f32,
    scale: f32,
}

impl GameApp for ViewerApp {
    fn window_title() -> &'static str {
        "Viewer"
    }

    fn init(graphics_device: &mut GraphicsDevice) -> Self {
        let keycap = keycap();

        let mesh = keycap.mesh();
        let cad_mesh = CadMesh::from_mesh(&mesh, graphics_device);

        let mut model_edges = vec![];

        let thickness = 2.0;

        for edge in keycap.edges() {
            let mut segments = vec![];
            for point in edge.approximation_segments() {
                segments.push(LineVertex3::new(
                    vec3(point.x as f32, point.y as f32, point.z as f32),
                    thickness,
                ));
            }

            model_edges.push(segments);
        }

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
            fullscreen_quad: FullscreenQuad::new(device, surface_texture_format),
            text_system: TextSystem::new(device, surface_texture_format, width, height),
            fps_counter: FPSCounter::new(),
            line_drawer: LineDrawer::new(
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
            model_edges,
            cad_mesh,
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

        self.fullscreen_quad.render(&mut frame_encoder.encoder, &smaa_render_target);

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

        let mut line_recorder = self.line_drawer.begin();
        for segment_list in &self.model_edges {
            line_recorder.draw_round_line_strip(segment_list);
        }

        line_recorder.end(
            &mut frame_encoder.encoder,
            &smaa_render_target,
            Some(&self.depth_texture.view),
            graphics_device.queue(),
            build_camera_matrix(width, height),
            transform,
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

fn keycap() -> Shape {
    const KEYCAP_PITCH: f64 = 19.05;

    let convex = false;
    let keycap_unit_size_x = 1.0;
    let keycap_unit_size_y = 1.0;
    let height = 16.0;
    let angle = 13.0;
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
            .transformed(dvec3(0.0, height - 2.1, -bx / 2.0), dvec3(0.0, 0.0, angle))
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
            .transformed(dvec3(0.0, height, bx / 2.0), dvec3(0.0, 0.0, angle))
            .sketch()
            .move_to(-by / 2.0 + 2.0, 0.0)
            .three_point_arc((0.0, (-depth + 1.5).min(-0.1)), (by / 2.0 - 2.0, 0.0))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_mid = Workplane::yz()
            .transformed(dvec3(0.0, height, 0.0), dvec3(0.0, 0.0, angle))
            .sketch()
            .move_to(-by / 2.0 - 2.0, -0.5)
            .three_point_arc((0.0, -depth), (by / 2.0 + 2.0, -0.5))
            .line_to(by / 2.0, height)
            .line_to(-by / 2.0, height)
            .close();

        let scoop_left = Workplane::yz()
            .transformed(dvec3(0.0, height, -bx / 2.0), dvec3(0.0, 0.0, angle))
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
        .transformed(dvec3(0.0, 0.0, height / 4.0), dvec3(0.0, 0.0, 0.0))
        .rect(bx - thickness * 3.0, by - thickness * 3.0);

    let shell_top = Workplane::xy()
        .transformed(dvec3(0.0, 0.0, height - height / 4.0 - 4.5), dvec3(angle, 0.0, 0.0))
        .rect(tx - thickness * 2.0 + 0.5, ty - thickness * 2.0 + 0.5);

    let shell = Solid::loft([&shell_bottom, &shell_mid, &shell_top].into_iter());

    let (keycap, _edges) = keycap.subtract(&shell);

    keycap
}

fn main() {
    simple_game::run_game_app::<ViewerApp>();
}
