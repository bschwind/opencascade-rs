use glam::dvec3;
use opencascade::{
    primitives::{Direction, Edge, Face, Solid, ToAngle},
    workplane::Workplane,
};

pub fn main() {
    let mut base = Workplane::xy().rect(10.0, 10.0);
    // asymmetric chamfer
    base.chamfer(1.0, Some(2.0));

    let mut top = Workplane::xy().rect(10.0, 10.0);
    // 45 degree chamfer - other angles will be asymmetric
    top.chamfer_angle(2.0, 45.degrees());
    top.translate(dvec3(0.0, 0.0, 10.0));

    let chamfered_box = Solid::loft([&base, &top].into_iter());

    let handle = Workplane::xy().rect(5.0, 5.0);
    let handle_face = Face::from_wire(&handle);

    let handle_body = handle_face.extrude(dvec3(0.0, 0.0, -10.0));
    let (mut chamfered_shape, _fuse_edges) = chamfered_box.union(&handle_body);

    // chamfer the top of the protrusion
    let top_edges: Vec<_> = chamfered_shape
        .faces()
        .farthest(Direction::NegZ) // Get the face whose center of mass is the farthest in the negative Z direction
        .expect("Should have a face on the bottom of the handle")
        .edges() // Get all the edges of this face
        .collect();
    chamfered_shape.chamfer_edges(1.0, &top_edges);

    // chamfer the handle join edges
    // let handle_join_edges = chamfered_shape
    //     .edges()
    //     .filter(|edge| {
    //         let start = edge.start_point();
    //         let end = edge.end_point();
    //         (start.x.abs() == 2.5 && start.z == 0.0) || (end.y.abs() == 2.5 && end.z == 0.0)
    //     })
    //     .collect::<Vec<Edge>>();
    // chamfered_shape.chamfer_edges(0.5, &fuse_edges);

    // can also just chamfer the whole shape
    // chamfered_shape.chamfer(0.5);

    chamfered_shape.write_stl("chamfer.stl").unwrap();
}
