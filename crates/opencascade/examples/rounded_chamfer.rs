use glam::dvec3;
use opencascade::{
    primitives::{Direction, IntoShape},
    workplane::Workplane,
};

// Demonstrates filleting a 2D profile, extruding it, then chamfering
// the top edges, resulting in a nice, rounded chamfer.

pub fn main() {
    let mut base = Workplane::xy().rect(16.0, 10.0);
    base.fillet(1.0);
    let shape = base.to_face().extrude(dvec3(0.0, 0.0, 3.0));
    let mut shape = shape.into_shape();

    let top_edges = shape.faces().farthest(Direction::PosZ).edges();
    shape.chamfer_edges(0.7, top_edges);

    shape.write_stl("rounded_chamfer.stl").unwrap();
}
