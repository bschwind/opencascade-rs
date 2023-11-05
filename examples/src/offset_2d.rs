use glam::DVec3;
use opencascade::{
    primitives::{IntoShape, Shape},
    workplane::Workplane,
};

// Demonstrates ofsetting a face or wire in 2D.

pub fn shape() -> Shape {
    let solid = Workplane::xy()
        .rect(16.0, 10.0)
        .offset(4.0) // offset a wire
        .to_face()
        .extrude(DVec3::new(0.0, 0.0, 8.0))
        .into_shape();

    let face = Workplane::xy()
        .translated(DVec3::new(0.0, 20.0, 0.0))
        .rect(16.0, 10.0)
        .to_face()
        .offset(4.0) // offset a face
        .extrude(DVec3::new(0.0, 0.0, 8.0))
        .into_shape();

    solid.union(&face).into()
}
