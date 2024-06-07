use glam::DVec3;
use opencascade::{
    angle::{RVec, ToAngle},
    primitives::{approximate_function, IntoShape, Shape, Shell, Wire},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let r = 10.0;
    let a = 5.0;

    let wire_profile: Wire = Workplane::xz()
        .rotated(RVec::z(45.0.degrees()))
        .translated(DVec3::new(-r, 0.0, 0.0))
        .rect(a, a);

    let path: Wire = Workplane::xy().sketch().arc((-r, 0.0), (0.0, r), (r, 0.0)).wire();

    let num_radii = 5;
    let pipe_shell: Shell = wire_profile.sweep_along_with_radius_values(
        &path,
        approximate_function(num_radii, |t| {
            let val = ((2.0 * std::f64::consts::PI * (t - 1.0 / 4.0)).sin() + 1.0) / 2.0;
            val * 10.0
        }),
    );

    pipe_shell.into_shape()
}
