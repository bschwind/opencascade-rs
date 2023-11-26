use glam::{vec3, Mat3, Mat4, Quat, Vec3};

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
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            projection: Projection::Orthographic,
            aspect_ratio: width as f32 / height as f32,
            zoom_factor: 1.0,
            target: vec3(0.0, 0.0, 0.0),
            radius: 100.0,
            orientation: Quat::IDENTITY,
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
    pub fn pan(&mut self, x: f32, y: f32) {}

    /// Zoom in or out, while looking at the same target.
    pub fn zoom(&mut self, zoom_delta: f32) {}

    /// Orbit around the target while keeping the distance.
    pub fn rotate(&mut self, rotator: Quat) {
        self.orientation *= rotator;
    }

    pub fn matrix(&self) -> Mat4 {
        // These magic numbers are configured so that the particular model we are loading is
        // visible in its entirety. They will be dynamically computed eventually when we have "fit
        // to view" function or alike.
        let proj = match self.projection {
            Projection::Orthographic => Mat4::orthographic_rh(
                -50.0 * self.zoom_factor * self.aspect_ratio,
                50.0 * self.zoom_factor * self.aspect_ratio,
                -50.0 * self.zoom_factor,
                50.0 * self.zoom_factor,
                -1000.0,
                1000.0,
            ),
            Projection::Perspective => {
                Mat4::perspective_rh(std::f32::consts::PI / 2.0, self.aspect_ratio, 0.01, 1000.0)
            },
        };

        let local_frame = self.get_local_frame();
        let position = self.target + self.radius * local_frame.z_axis;
        let upward = local_frame.y_axis;
        let view = Mat4::look_at_rh(position, self.target, upward);

        proj * view
    }
}
