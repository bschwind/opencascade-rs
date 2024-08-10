use glam::dvec3;
use opencascade::{
    primitives::{IntoShape, Shape, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let outer_diameter = 10.9;
    let outer_radius = outer_diameter / 2.0;

    let pin_diameter = 8.0;
    let pin_radius = pin_diameter / 2.0;
    let pin_height = 10.0;

    let disk_thickness = 1.29;

    let height = 5.0;

    let bottom_profile: Wire = Workplane::xy()
        .sketch()
        .move_to(-outer_radius, -height / 2.0)
        .line_to(-outer_radius, height / 2.0)
        .three_point_arc((0.0, height / 2.0 + outer_radius), (outer_radius, height / 2.0))
        .line_to(outer_radius, -height / 2.0)
        .three_point_arc((0.0, -height / 2.0 - outer_radius), (-outer_radius, -height / 2.0))
        .wire();

    let bottom_volume =
        bottom_profile.to_face().extrude(dvec3(0.0, 0.0, disk_thickness)).into_shape();

    let pin_profile: Wire = Workplane::xy()
        .translated(dvec3(0.0, 0.0, 0.1))
        .sketch()
        .move_to(-pin_radius, -height / 2.0)
        .line_to(-pin_radius, height / 2.0)
        .three_point_arc((0.0, height / 2.0 + pin_radius), (pin_radius, height / 2.0))
        .line_to(pin_radius, -height / 2.0)
        .three_point_arc((0.0, -height / 2.0 - pin_radius), (-pin_radius, -height / 2.0))
        .wire();

    let pin_volume = pin_profile.to_face().extrude(dvec3(0.0, 0.0, pin_height)).into();

    let thumbtack_circle: Wire = Workplane::xy().circle(0.0, -height / 2.0, outer_radius);
    let thumbtack_opening =
        thumbtack_circle.to_face().extrude(dvec3(0.0, 0.0, pin_height)).into_shape();

    let mut cutout = bottom_volume.union(&pin_volume).union(&thumbtack_opening);

    // let hana_block = Shape::read_step("examples/models/hana-block-solid.step").unwrap();
    let hana_block = Shape::box_centered(200.0, 200.0, 2.0);

    dbg!(hana_block.shape_type());

    cutout.set_global_translation(dvec3(-90.0, 80.0, 0.4));

    let shape: Shape = hana_block.subtract(&cutout).into();

    shape.write_step("output.step").unwrap();

    shape
}
