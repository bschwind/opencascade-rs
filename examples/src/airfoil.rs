// This example demonstrates using a bezier surface to create an airfoil like shape

use glam::dvec3;
use opencascade::primitives::{Face, Shape, Surface};

pub fn shape() -> Shape {
    let points1 = [
        dvec3(0.0, 0.0, 0.0),
        dvec3(5.0, 0.0, 10.0),
        dvec3(20.0, 0.0, 10.0),
        dvec3(50.0, 0.0, 10.0),
        dvec3(100.0, 0.0, 0.0),
    ];
    let points2 = [
        dvec3(0.0, 0.0, 0.0),
        dvec3(-2.0, 0.0, -8.0),
        dvec3(50.0, 0.0, 0.0),
        dvec3(50.0, 0.0, 0.0),
        dvec3(100.0, 0.0, 0.0),
    ];

    let surface = Surface::bezier([points1, points2]);
    let face = Face::from_surface(&surface);

    let airfoil = face.extrude(dvec3(0.0, 50.0, 0.0));

    airfoil.into()
}
