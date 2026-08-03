use glam::dvec3;
use opencascade::{
    primitives::{IntoShape, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let brick_width = 47.3;
    let brick_height = 29.2;
    let bracket_depth = 100.0;
    let thickness = 3.0;
    let wing_width = 20.0;
    let hole_radius = 1.35;
    let use_four_holes = false;

    let port_cutout = Workplane::xz()
        .sketch()
        .line_dy(brick_height)
        .line_dx(brick_width)
        .line_dy(-brick_height)
        .line_dx(wing_width + thickness)
        .line_dy(thickness)
        .line_dx(-wing_width)
        .line_dy(brick_height)
        .line_dx(-(brick_width + thickness * 2.0))
        .line_dy(-brick_height)
        .line_dx(-wing_width)
        .line_dy(-thickness)
        .close()
        .to_face();

    let mut bracket = port_cutout.extrude(dvec3(0.0, bracket_depth, 0.0)).into_shape();

    let drill_x_left = -thickness - (wing_width / 2.0);
    let drill_x_right = brick_width + thickness + (wing_width / 2.0);
    let drill_z = thickness;

    let drill_positions = if use_four_holes {
        vec![
            dvec3(drill_x_left, bracket_depth / 4.0, drill_z),
            dvec3(drill_x_left, (bracket_depth * 3.0) / 4.0, drill_z),
            dvec3(drill_x_right, bracket_depth / 4.0, drill_z),
            dvec3(drill_x_right, (bracket_depth * 3.0) / 4.0, drill_z),
        ]
    } else {
        vec![
            dvec3(drill_x_left, bracket_depth / 2.0, drill_z),
            dvec3(drill_x_right, bracket_depth / 2.0, drill_z),
        ]
    };

    for drill_pos in drill_positions {
        bracket = bracket.drill_hole(drill_pos, dvec3(0.0, 0.0, -1.0), hole_radius);
    }

    bracket
}
