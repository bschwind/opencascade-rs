use glam::DVec3;
use opencascade::{
    primitives::{IntoShape, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let f = Workplane::xy().rect(1., 2.);

    f.extrude(DVec3::new(0., 0., 3.)).into_shape()
}
