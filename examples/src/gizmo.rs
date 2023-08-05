use glam::DVec3;
use opencascade::{
    primitives::{Shape, Solid},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let arrow_length = 10.0;
    let cone_height = 2.0;
    let shaft_length = arrow_length - cone_height;

    let arrow = |workplane: Workplane| {
        let shaft =
            workplane.circle(0.0, 0.0, 0.1).to_face().extrude(workplane.normal() * arrow_length);
        let cone_base =
            workplane.translated(DVec3::new(0.0, 0.0, shaft_length)).circle(0.0, 0.0, 1.0);
        let cone_top =
            workplane.translated(DVec3::new(0.0, 0.0, arrow_length)).circle(0.0, 0.0, 0.05);
        let cone = Solid::loft([&cone_base, &cone_top].into_iter());
        let arrow_shape = shaft.union(&cone);

        arrow_shape.shape
    };

    arrow(Workplane::yz()).union(&arrow(Workplane::xz())).union(&arrow(Workplane::xy())).shape
}
