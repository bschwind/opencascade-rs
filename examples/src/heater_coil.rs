use glam::DVec3;
use opencascade::{
    primitives::{Edge, Face, IntoShape, Shape, Solid, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let radius = 1.0;
    let spiral_radius = 5.0;
    let spiral_pitch = 0.25;
    let spiral_half_turn_count = 5;

    let face_profile: Face = Workplane::xz()
        .translated(DVec3::new(-spiral_radius, 0.0, 0.0))
        .circle(0.0, 0.0, radius)
        .to_face();

    let sample_count = 100;
    let spiral_points = (0..sample_count).map(|i| {
        let t = i as f64 / (sample_count - 1) as f64;
        let angle_rad = spiral_half_turn_count as f64 * std::f64::consts::PI * t;
        let (y, x) = angle_rad.sin_cos();
        let z = spiral_radius * spiral_pitch * i as f64;
        DVec3::new(spiral_radius * x, spiral_radius * y, z)
    });

    let coil = Edge::spline_from_points(spiral_points);
    let path = Wire::from_edges(&[coil]);

    let pipe_solid: Solid = face_profile.sweep_along(&path);
    pipe_solid.into_shape()
}
