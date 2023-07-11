use glam::dvec3;
use opencascade::{primitives::Solid, workplane::Workplane};

pub fn main() {
    let mut base = Workplane::xy().rect(10.0, 10.0);
    base.chamfer(1.0, Some(2.0));

    let mut top = Workplane::xy().rect(10.0, 10.0);
    top.chamfer_angle(2.0, 1.0);
    top.translate(dvec3(0.0, 0.0, 10.0));

    let chamfered_box = Solid::loft([&base, &top].into_iter());

    chamfered_box.write_stl("chamfer.stl").unwrap();
}
