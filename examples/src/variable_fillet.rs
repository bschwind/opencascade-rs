use glam::dvec3;
use opencascade::{
    primitives::{Direction, IntoShape, Shape},
    workplane::Workplane,
};

// Demonstrates a variable fillet radius along the edge of a cube.
pub fn shape() -> Shape {
    let base = Workplane::xy().rect(50.0, 50.0);
    let shape = base.to_face().extrude(dvec3(0.0, 0.0, 50.0));
    let mut shape = shape.into_shape();

    let right_edge = shape.faces().farthest(Direction::PosX).edges().next().unwrap();
    shape.variable_fillet_edge(
        [(0.0, 7.0), (0.2, 20.0), (0.5, 3.0), (0.8, 20.0), (1.0, 7.0)],
        &right_edge,
    );

    shape
}
