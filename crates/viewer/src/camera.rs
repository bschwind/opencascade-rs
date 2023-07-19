use glam::{vec3, Mat4};

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
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self { projection: Projection::Orthographic, aspect_ratio: width as f32 / height as f32 }
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

    pub fn matrix(&self) -> Mat4 {
        let proj = match self.projection {
            Projection::Orthographic => Mat4::orthographic_rh(
                -100.0 * self.aspect_ratio,
                100.0 * self.aspect_ratio,
                -100.0,
                100.0,
                -1000.0,
                1000.0,
            ),
            Projection::Perspective => {
                Mat4::perspective_rh(std::f32::consts::PI / 2.0, self.aspect_ratio, 0.01, 1000.0)
            },
        };

        let view = Mat4::look_at_rh(
            vec3(20.0, -30.0, 20.0), // Eye position
            vec3(0.0, 0.0, 0.0),     // Look-at target
            vec3(0.0, 0.0, 1.0),     // Up vector of the camera
        );

        proj * view
    }
}
