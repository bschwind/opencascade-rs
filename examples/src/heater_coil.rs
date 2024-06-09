use std::borrow::Borrow;

use glam::DVec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{Edge, Face, IntoShape, Shape, Solid, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let a = 1.0;
    let spiral_radius = 5.0;
    let spiral_pitch = 3.0;
    let spiral_half_turn_count = 5;
    let attach_len = 4.0;

    let face_profile: Face = Workplane::xz()
        .translated(DVec3::new(spiral_radius, 0.0, attach_len))
        .rotated(RVec::z(45.0.degrees()))
        .rect(a, a)
        .to_face();

    let sample_count = 80;
    let spiral_points: Vec<DVec3> = (0..sample_count)
        .map(|i| {
            let t = i as f64 / (sample_count - 1) as f64;
            let angle_rad = spiral_half_turn_count as f64 * std::f64::consts::PI * t;
            let (y, x) = angle_rad.sin_cos();

            let u = 0.5 * spiral_half_turn_count as f64 * t;
            let z = spiral_pitch * u;

            DVec3::new(spiral_radius * x, spiral_radius * y, z)
        })
        .collect();

    let p0 = spiral_points[0];
    let p1 = spiral_points[sample_count - 1];

    let coil = Edge::spline_from_points(spiral_points.into_iter());
    let attach_0 = Edge::segment(p0 - DVec3::new(0.0, attach_len, 0.0), p0);
    let attach_1 = Edge::segment(p1, p1 - DVec3::new(0.0, attach_len, 0.0));
    let path = Wire::from_edges(&[attach_0, coil, attach_1]);

    //path.into_shape().union(&face_profile.into_shape()).into_shape()

    let pipe_solid: Solid = face_profile.sweep_along(&path);
    pipe_solid.into_shape()
}
