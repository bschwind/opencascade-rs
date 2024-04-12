use glam::{dvec3, DVec3};
use opencascade::{
    primitives::{Direction, Face, IntoShape, Shape, Wire},
    workplane::Workplane,
};
use std::f64::consts::PI;

pub fn shape() -> Shape {
    let width = 15.0;
    let thickness = 2.5;
    let cable_radius = 6.0 / 2.0;
    let leg_length = 15.0;

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

    let pipe_solid = face_profile.sweep_along(&path).into_shape();

    // Retrieve the vertical edges on the farthest left and right faces,
    // we want to fillet them.
    let left_edges =
        pipe_solid.faces().farthest(Direction::NegX).edges().parallel_to(Direction::PosZ);

    let right_edges =
        pipe_solid.faces().farthest(Direction::PosX).edges().parallel_to(Direction::PosZ);

    let mut bracket =
        pipe_solid.fillet_edges(width / 2.5, left_edges.chain(right_edges)).fillet(1.0);

    let drill_point = bend_start + (leg_length / 2.0);

    let indentation_height = thickness - 1.63;
    let thumbtack_pin_radius = 3.15 / 2.0;
    let thumbtack_base_radius = 10.1 / 2.0;

    for x_pos in [drill_point, -drill_point] {
        let cylinder = Shape::cylinder(
            dvec3(x_pos, 0.0, (thickness / 2.0) - indentation_height),
            thumbtack_base_radius,
            DVec3::Z,
            3.0,
        );

        bracket = bracket.subtract(&cylinder).chamfer_new_edges(0.3);
    }

    for x_pos in [drill_point, -drill_point] {
        bracket = bracket.drill_hole(dvec3(-x_pos, 0.0, 0.0), DVec3::Z, thumbtack_pin_radius);
    }

    bracket
}
