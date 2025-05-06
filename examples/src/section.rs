use core::f64;
use glam::dvec3;
use opencascade::{
    angle::Angle::Radians,
    primitives::{Compound, IntoShape, Shape},
    section,
    workplane::Workplane,
};

pub fn shape() -> Shape {
    // Create a wire by sketching two arcs on the XY plane
    let s = Workplane::xy()
        .sketch()
        .arc((0., -2.), (1., -1.), (0., 0.))
        .arc((0., 0.), (-1., 1.), (0., 2.))
        .wire();

    // Create a circular face on the YZ plane
    let f = Workplane::yz().circle(0., 0., 0.5).to_face();

    // Sweep the circular face along the wire to create a 3D shape
    let shape = f.sweep_along(&s).into_shape();

    // Create a cutting plane and rotate
    let p = Workplane::yz()
        .rect(10., 10.)
        .transform(dvec3(0., 0., 0.), dvec3(0., 0., 1.), Radians(f64::consts::PI / 8.))
        .to_face()
        .into_shape();

    // Compute the intersection edges between the swept shape and the transformed rectangle
    let edges = section::edges(&shape, &p);

    // Combine the intersection edges, the swept shape, and the rectangle's edges into a compound shape
    let all_shapes = vec![
        edges,                                       // Section edges
        vec![shape],                                 // The shape, run with this line commented out
        p.edges().map(|e| e.into_shape()).collect(), // The edges of the cutting plane
    ];

    Compound::from_shapes(all_shapes.iter().flatten()).into_shape()
}
