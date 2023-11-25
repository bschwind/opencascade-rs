use glam::DVec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::Shape,
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let r = 10.0;
    let a = 5.0;

    let profile = Workplane::xz()
        .rotated(RVec::z(45.0.degrees()))
        .translated(DVec3::new(-r, 0.0, 0.0))
        .rect(a, a);
    let spine = Workplane::xy().sketch().arc((-r, 0.0), (0.0, r), (r, 0.0)).wire();

    //spine.into_shape().union(&profile.into_shape()).into_shape()

    let pipe = profile.sweep_along(&spine);
    pipe.into()
}
