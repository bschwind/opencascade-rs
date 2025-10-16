// This example demonstrates using `Shape::empty()` for convenience.

use std::f64::consts::PI;

use glam::DVec3;
use opencascade::primitives::{IntoShape, Shape};

const KNOT_SIZE: f64 = 64.0;
const KNOT_P: f64 = 2.0;
const KNOT_Q: f64 = 5.0;
const BEAD_SIZE: f64 = 16.0;
const SAMPLE_COUNT: u32 = 128;

pub fn shape() -> Shape {
    let mut shape = Shape::empty();

    for t in (0..SAMPLE_COUNT).map(|i| i as f64 / SAMPLE_COUNT as f64) {
        let pos = torus_knot(t);
        let bead = Shape::sphere(BEAD_SIZE).at(KNOT_SIZE * pos).build();
        shape = shape.union(&bead).into_shape();
    }

    shape
}

fn torus_knot(t: f64) -> DVec3 {
    let phi = 2.0 * PI * t;
    let r = (KNOT_Q * phi).cos() + 2.0;
    let x = r * (KNOT_P * phi).cos();
    let y = r * (KNOT_P * phi).sin();
    let z = -(KNOT_Q * phi).sin();
    DVec3::new(x, y, z)
}
