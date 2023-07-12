use glam::dvec3;
use opencascade::primitives::{Edge, Face, ToAngle};
use opencascade::{primitives::Solid, workplane::Workplane};

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
    let mut chamfered_shape = chamfered_box.union(&handle_body);

    // chamfer the top of the protrusion by getting shape edges at z = -10.0
    let top_edges = chamfered_shape
        .edges()
        .filter(|edge| edge.start_point().z == -10.0 && edge.end_point().z == -10.0)
        .collect::<Vec<Edge>>();
    chamfered_shape.chamfer_edges(1.0, &top_edges);

    // chamfer the handle join edges
    let handle_join_edges = chamfered_shape
        .edges()
        .filter(|edge| {
            let start = edge.start_point();
            let end = edge.end_point();
            (start.x.abs() == 2.5 && start.z == 0.0) || (end.y.abs() == 2.5 && end.z == 0.0)
        })
        .collect::<Vec<Edge>>();
    chamfered_shape.chamfer_edges(0.5, &handle_join_edges);

    // can also just chamfer the whole shape
    // chamfered_shape.chamfer(0.5);

    chamfered_shape.write_stl("chamfer.stl").unwrap();
}
