use glam::dvec3;
use opencascade::{
    primitives::{approximate_function, Direction, IntoShape, Shape},
    workplane::Workplane,
};

// Demonstrates a variable fillet radius along the edge of a cube.
pub fn shape() -> Shape {
    let base = Workplane::xy().rect(50.0, 50.0);
    let mut shape = base.to_face().extrude(dvec3(0.0, 0.0, 50.0)).into_shape();

    let mut right_face_edges = shape.faces().farthest(Direction::PosX).edges();
    let first_edge = right_face_edges.next().unwrap();
    let another_edge = right_face_edges.next().unwrap();

    // Manually define fillet radii at normalized 't' values (0-1), where
    // t is 0 at the start of the edge, and 1 at the end of the edge.
    shape = shape.variable_fillet_edge(
        [(0.0, 7.0), (0.2, 20.0), (0.5, 3.0), (0.8, 20.0), (1.0, 7.0)],
        &first_edge,
    );

    // Or define fillet radii by providing n, the number of radii to generate,
    // and a function which accepts t and returns a radius for the fillet at t.
    let num_radii = 5;
    shape = shape.variable_fillet_edge(
        approximate_function(num_radii, |t| {
            let t_squared = t * t;
            let val = t_squared / (2.0 * (t_squared - t) + 1.0);
            (val + 0.2) * 10.0
        }),
        &another_edge,
    );

    let left_face_edges = shape.faces().farthest(Direction::NegX).edges();

    // Fillet all edges on the left face with a rough bell curve, for fun.
    shape.variable_fillet_edges(
        approximate_function(num_radii, |t| {
            let val = ((2.0 * std::f64::consts::PI * (t - 1.0 / 4.0)).sin() + 1.0) / 2.0;
            val * 10.0
        }),
        left_face_edges,
    )
}
