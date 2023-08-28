use glam::dvec3;
use opencascade::{
    primitives::{Direction, IntoShape, Shape},
    workplane::Workplane,
};

// Demonstrates filleting a 2D profile, extruding it, then chamfering
// the top edges, resulting in a nice, rounded chamfer.

pub fn shape() -> Shape {
    let shape = Workplane::xy()
        .rect(16.0, 10.0)
        .fillet(1.0)
        .to_face()
        .extrude(dvec3(0.0, 0.0, 3.0))
        .into_shape();

    let top_edges = shape.faces().farthest(Direction::PosZ).edges();

    shape.chamfer_edges(0.7, top_edges)
}
