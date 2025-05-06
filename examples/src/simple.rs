use core::f64;
use glam::dvec3;
use opencascade::{
    angle::Angle::Radians,
    primitives::{Compound, IntoShape, Shape},
    section::Section,
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let s = Workplane::xy()
        .sketch()
        .arc((0., -2.), (1., -1.), (0., 0.))
        .arc((0., 0.), (-1., 1.), (0., 2.))
        .wire();

    let f = Workplane::yz().circle(0., 0., 0.5).to_face();

    let shape = f.sweep_along(&s).into_shape();

    let p = Workplane::yz()
        .rect(10., 10.)
        .transform(dvec3(0., 0., 0.), dvec3(0., 0., 1.), Radians(f64::consts::PI / 6.))
        .to_face()
        .into_shape();

    let edges = Section::new(&shape, &p).section_edges();

    Compound::from_shapes(vec![edges, vec![shape], vec![p]].iter().flatten()).into_shape()
}
