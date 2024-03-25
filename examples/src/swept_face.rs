use glam::DVec3;
use opencascade::{
    primitives::{Face, IntoShape, Shape, Solid, Wire},
    workplane::Workplane,
};
use std::f64::consts::PI;

pub fn shape() -> Shape {
    let width = 20.0;
    let thickness = 5.0;
    let cable_radius = 20.0;
    let leg_length = 30.0;

    let pre_bend_radius = thickness;

    let bend_start = cable_radius + (thickness / 2.0) + pre_bend_radius;
    let max_extent = bend_start + leg_length;

    let face_profile: Face = Workplane::yz()
        .translated(DVec3::new(0.0, 0.0, -max_extent))
        .rect(width, thickness)
        .to_face();

    let x = (PI / 4.0).cos() * pre_bend_radius;
    let y = (1.0 - (PI / 4.0).sin()) * pre_bend_radius;

    let path: Wire = Workplane::xz()
        .sketch()
        .move_to(-max_extent, 0.0)
        .line_to(-bend_start, 0.0)
        .three_point_arc((-bend_start + x, y), (-bend_start + pre_bend_radius, pre_bend_radius))
        .three_point_arc((0.0, bend_start), (bend_start - pre_bend_radius, pre_bend_radius))
        .three_point_arc((bend_start - x, y), (bend_start, 0.0))
        .line_to(max_extent, 0.0)
        .wire();

    let pipe_solid: Solid = face_profile.sweep_along(&path);

    pipe_solid.into_shape()
}
