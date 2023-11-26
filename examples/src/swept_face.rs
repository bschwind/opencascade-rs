use glam::DVec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{Face, IntoShape, Shape, Solid, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let r = 10.0;
    let a = 5.0;

    let face_profile: Face = Workplane::xz()
        .rotated(RVec::z(45.0.degrees()))
        .translated(DVec3::new(-r, 0.0, 0.0))
        .rect(a, a)
        .to_face();

    let path: Wire = Workplane::xy().sketch().arc((-r, 0.0), (0.0, r), (r, 0.0)).wire();

    let pipe_solid: Solid = face_profile.sweep_along(&path);

    pipe_solid.into_shape()
}
