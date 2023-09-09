use glam::{vec3, Mat4, Quat, Vec3};

const MIN_ZOOM_FACTOR: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Projection {
    /// Right-handed orthographic projection.
    Orthographic,
    /// Right-handed Perspective projection.
    Perspective,
}

pub struct Camera {
    projection: Projection,
    aspect_ratio: f32,
    // Zoom factor used for orthographic projection.
    zoom_factor: f32,
    // Position of the camera.
    position: Vec3,
    // The upward vector of the camera, determining its orientation.
    upward: Vec3,
    // The look-at target, in the center of the view.
    target: Vec3,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            projection: Projection::Orthographic,
            aspect_ratio: width as f32 / height as f32,
            zoom_factor: 1.0,
            position: vec3(20.0, -30.0, 20.0),
            upward: vec3(0.0, 0.0, 1.0),
            target: vec3(0.0, 0.0, 0.0),
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

    /// Pan the camera view horizontally and vertically. Look-at target will move along with the
    /// camera.
    pub fn pan(&mut self, x: f32, y: f32) {
        let forward = self.target - self.position;
        let rightward = self.upward.cross(forward).normalize();
        let translation = rightward * x + self.upward * y;
        self.position += translation;
        self.target += translation;
    }

    /// Zoom in or out, while looking at the same target.
    pub fn zoom(&mut self, zoom_delta: f32) {
        // Change the camera position for perspective projection.
        let forward = self.target - self.position;
        self.position += forward * zoom_delta;
        // Update the zoom factor for orthographic projection.
        self.zoom_factor = (self.zoom_factor - zoom_delta).max(MIN_ZOOM_FACTOR);
    }

    /// Orbit around the target while keeping the distance.
    pub fn rotate(&mut self, yaw: f32, pitch: f32) {
        let backward = (self.position - self.target).normalize();
        let leftward = self.upward.cross(backward);
        let rotation =
            Quat::from_axis_angle(self.upward, yaw) * Quat::from_axis_angle(leftward, pitch);
        self.position = (self.target + rotation * backward) * backward.length();
        self.upward = rotation * self.upward;
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

        let view = Mat4::look_at_rh(self.position, self.target, self.upward);

        proj * view
    }
}
