use glam::{dvec3, DVec3};
use opencascade::{
    primitives::{Direction, IntoShape, Shape, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let thumbtack_big_diameter = 10.75;
    let thumbtack_big_radius = thumbtack_big_diameter / 2.0;
    let thumbtack_pin_radius = 1.2 / 2.0;
    let thickness = 1.5;
    let cable_width = 7.6;
    let cable_height = 2.3;

    let overhang_width = 1.0;

    let path: Wire = Workplane::xz()
        .sketch()
        .line_dx(thickness)
        .line_dy(-(thumbtack_big_diameter + cable_width + 2.0))
        .line_dx(cable_height + 0.1)
        .line_dy(cable_width + 0.1)
        .line_dx(-overhang_width)
        .line_dy(thickness)
        .line_dx(overhang_width + thickness)
        .line_dy(-(thickness + cable_width + thickness))
        .line_dx(-(thickness + cable_height + thickness))
        .close();

    let mut bracket = path.to_face().extrude(dvec3(0.0, thumbtack_big_diameter, 0.0)).into_shape();

    let top_edges = bracket.faces().farthest(Direction::PosZ).edges().parallel_to(Direction::PosX);

    bracket = bracket.fillet_edges(thumbtack_big_diameter / 2.1, top_edges);

    let cylinder = Shape::cylinder(
        dvec3(thickness, thumbtack_big_radius, -thumbtack_big_radius),
        thumbtack_big_radius + 0.2,
        -DVec3::X,
        thickness - 1.0,
    );

    bracket = bracket.subtract(&cylinder).fillet(0.2);

    bracket = bracket.drill_hole(
        dvec3(thickness, thumbtack_big_radius, -thumbtack_big_radius),
        -DVec3::X,
        thumbtack_pin_radius,
    );

    bracket
}
