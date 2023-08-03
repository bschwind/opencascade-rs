use glam::dvec3;
use opencascade::{
    primitives::{Direction, Face, Solid},
    workplane::Workplane,
};

pub fn main() {
    // A tapering chamfer from bottom to top 2->1
    let mut base = Workplane::xy().rect(10.0, 10.0);
    base.chamfer(2.0);

    let mut top = Workplane::xy().rect(10.0, 10.0);
    top.translate(dvec3(0.0, 0.0, 10.0));
    top.chamfer(1.0);

    let chamfered_box = Solid::loft([&base, &top]);

    // Insert the workplane into the chamfered box area so union returns edges
    let handle = Workplane::xy().translated(dvec3(0.0, 0.0, 0.1)).rect(5.0, 5.0);
    let handle_face = Face::from_wire(&handle);

    let handle_body = handle_face.extrude(dvec3(0.0, 0.0, -10.1));
    let mut chamfered_shape = chamfered_box.union(&handle_body);
    chamfered_shape.chamfer_new_edges(0.5);

    // Chamfer the top of the protrusion
    let top_edges = chamfered_shape
        .faces()
        .farthest(Direction::NegZ) // Get the face whose center of mass is the farthest in the negative Z direction
        .edges(); // Get all the edges of this face

    chamfered_shape.chamfer_edges(1.0, top_edges);

    // Can also just chamfer the whole shape with:
    // chamfered_shape.chamfer(0.5);

    chamfered_shape.write_stl("chamfer.stl").unwrap();
}
