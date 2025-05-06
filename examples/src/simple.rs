use glam::dvec3;
use opencascade::{
    primitives::{Compound, IntoShape, Shape},
    section::Section,
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let f = Workplane::xy().rect(1., 2.).to_face();

    let b = f.extrude(dvec3(0., 0., 3.)).into_shape();

    let p = Workplane::yz().rect(10., 10.).to_face().into_shape();

    let edges = Section::new(&b, &p).section_edges();

    Compound::from_shapes(vec![edges, vec![b], vec![p]].iter().flatten()).into_shape()
}
