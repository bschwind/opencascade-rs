use glam::DVec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{approximate_function, Face, IntoShape, Shape, Solid, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let r = 10.0;
    let a = 5.0;

    let face_profile: Face = Workplane::xz()
        .translated(DVec3::new(-r, 0.0, 0.0))
        .rotated(RVec::z(45.0.degrees()))
        .rect(a, a)
        .to_face();

    let path: Wire = Workplane::xy().sketch().arc((-r, 0.0), (0.0, r), (r, 0.0)).wire();

    let num_radii = 32;
    let pipe_solid: Solid = face_profile.sweep_along_with_radius_values(
        &path,
        approximate_function(num_radii, |t| (-(8.0 * std::f64::consts::PI * t).cos() + 3.0) / 4.0),
    );

    pipe_solid.into_shape()
}
