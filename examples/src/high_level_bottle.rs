use glam::{dvec3, DVec3};
use opencascade::primitives::{Direction::PosZ, Edge, Face, IntoShape, Shape, Wire};

pub fn shape() -> Shape {
    let height = 70.0;
    let width = 50.0;
    let thickness = 30.0;

    // Define the points making up the bottle's profile.
    let point_1 = dvec3(-width / 2.0, 0.0, 0.0);
    let point_2 = dvec3(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = dvec3(0.0, -thickness / 2.0, 0.0);
    let point_4 = dvec3(width / 2.0, -thickness / 4.0, 0.0);
    let point_5 = dvec3(width / 2.0, 0.0, 0.0);

    let arc = Edge::arc(point_2, point_3, point_4);
    let segment_1 = Edge::segment(point_1, point_2);
    let segment_2 = Edge::segment(point_4, point_5);

    let wire = Wire::from_edges([&segment_1, &arc, &segment_2]);
    let mirrored_wire = wire.mirror_along_axis(dvec3(0.0, 0.0, 0.0), dvec3(1.0, 0.0, 0.0));

    let wire_profile = Wire::from_wires([&wire, &mirrored_wire]);
    let face_profile = Face::from_wire(&wire_profile);

    let body = face_profile.extrude(dvec3(0.0, 0.0, height)).into_shape().fillet(thickness / 12.0);

    // Create the neck and join it with the body
    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let neck = Shape::cylinder(dvec3(0.0, 0.0, height), neck_radius, DVec3::Z, neck_height);
    let bottle = neck.union(&body);

    let top_face = bottle.faces().farthest(PosZ);
    bottle.hollow(-thickness / 50.0, [top_face])
}
