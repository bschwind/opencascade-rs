use opencascade::bounding_box;
use opencascade::primitives::{Compound, IntoShape, Shape};
use opencascade::workplane::Workplane;

pub fn shape() -> Shape {
    let mut shapes = vec![];

    // Create some non-orthogonal shapes.
    for i in 0..=1 {
        // NOTE: Try changing the range values and the position vector
        let v = (2i32.pow(i)) as f64;
        let s = Shape::sphere(1.0).at(glam::dvec3(v, v, v * 2.0)).build();
        shapes.push(s);
    }

    // Add a circle to show that edges work, too.
    shapes.push(Workplane::xy().circle(0.0, 0.0, 2.0).into_shape());

    // Combine them to create a bounding box.
    let c = Compound::from_shapes(shapes);

    // Create the bounding box.
    let bb = bounding_box::aabb(&Shape::from(&c));

    // Create a box geometry for rendering.
    let bb_shape = Shape::box_from_corners(bb.min(), bb.max());

    let all_shapes =
        [vec![c.into_shape()], bb_shape.edges().map(|e| e.into_shape()).collect::<Vec<_>>()];

    // Combine the bounded and bounding geometry.
    Compound::from_shapes(all_shapes.iter().flatten()).into_shape()
}
