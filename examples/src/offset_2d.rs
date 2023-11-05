use glam::DVec3;
use opencascade::{
    primitives::{IntoShape, JoinType, Shape},
    workplane::Workplane,
};

// Demonstrates ofsetting a face or wire in 2D.

pub fn shape() -> Shape {
    let mut shapes = vec![];

    for (i, join_type) in
        [JoinType::Arc, JoinType::Tangent, JoinType::Intersection].into_iter().enumerate()
    {
        let solid_1 = Workplane::xy()
            .translated(DVec3::new(32.0 * i as f64, 0.0, 0.0))
            .rect(16.0, 10.0)
            .offset(4.0, join_type) // offset a wire
            .to_face()
            .extrude(DVec3::new(0.0, 0.0, 8.0))
            .into_shape();

        shapes.push(solid_1);

        let solid_2 = Workplane::xy()
            .translated(DVec3::new(32.0 * i as f64, 24.0, 0.0))
            .rect(16.0, 10.0)
            .to_face()
            .offset(4.0, join_type) // offset a face
            .extrude(DVec3::new(0.0, 0.0, 8.0))
            .into_shape();

        shapes.push(solid_2);
    }

    shapes.into_iter().reduce(|acc, item| acc.union(&item).shape).unwrap()
}
