use glam::{Mat3, Mat4, Quat, Vec2, Vec3};

const MIN_ZOOM_FACTOR: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Projection {
    /// Right-handed orthographic projection.
    Orthographic,
    /// Right-handed Perspective projection.
    Perspective,
}

pub struct OrbitCamera {
    projection: Projection,
    aspect_ratio: f32,
    // Zoom factor used for orthographic projection.
    zoom_factor: f32,
    // The look-at target, in the center of the view.
    target: Vec3,
    // The radius of the orbit
    radius: f32,
    // The orientation of the camera around the target point
    orientation: Quat,
}

impl OrbitCamera {
    pub fn new(width: u32, height: u32, init_pos: Vec3) -> Self {
        let target = Vec3::ZERO;
        let radius = init_pos.length();
        let look_at_matrix = Mat4::look_at_rh(init_pos, target, Vec3::Z);
        let orientation = Quat::from_mat4(&look_at_matrix).inverse();
        Self {
            projection: Projection::Orthographic,
            aspect_ratio: width as f32 / height as f32,
            zoom_factor: 1.0,
            target,
            radius,
            orientation,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn use_perspective(&mut self) {
        self.projection = Projection::Perspective;
    }

    pub fn use_orthographic(&mut self) {
        self.projection = Projection::Orthographic;
    }

    fn get_local_frame(&self) -> Mat3 {
        Mat3::from_quat(self.orientation)
    }

    /// Pan the camera view horizontally and vertically. Look-at target will move along with the
    /// camera.
    pub fn pan(&mut self, delta: Vec2) {
        self.target -= self.get_local_frame() * delta.extend(0.0);
    }

    /// Zoom in or out, while looking at the same target.
    pub fn zoom(&mut self, zoom_delta: f32) {
        self.zoom_factor = f32::max(self.zoom_factor * f32::exp(zoom_delta), MIN_ZOOM_FACTOR);
    }

    /// Orbit around the target while keeping the distance.
    pub fn rotate(&mut self, rotator: Quat) {
        self.orientation = (self.orientation * rotator).normalize();
    }

    pub fn matrix(&self) -> Mat4 {
        // These magic numbers are configured so that the particular model we are loading is
        // visible in its entirety. They will be dynamically computed eventually when we have "fit
        // to view" function or alike.

        let (proj, effective_radius) = match self.projection {
            Projection::Orthographic => {
                let proj = Mat4::orthographic_rh(
                    -50.0 * self.zoom_factor * self.aspect_ratio,
                    50.0 * self.zoom_factor * self.aspect_ratio,
                    -50.0 * self.zoom_factor,
                    50.0 * self.zoom_factor,
                    -1000.0,
                    1000.0,
                );
                (proj, self.radius)
            },
            Projection::Perspective => {
                let proj = Mat4::perspective_rh(
                    std::f32::consts::PI / 2.0,
                    self.aspect_ratio,
                    1.0,
                    10_000.0,
                );
                (proj, self.zoom_factor * self.radius)
            },
        };

        let local_frame = self.get_local_frame();
        let position = self.target + effective_radius * local_frame.z_axis;

        // NOTE(mkovaxx): This is computing inverse(translation * orientation), but more efficiently
        let view =
            Mat4::from_quat(self.orientation.conjugate()) * Mat4::from_translation(-position);

        proj * view
    }
}
