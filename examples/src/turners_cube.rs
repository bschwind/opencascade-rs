use glam::DVec3;
use opencascade::{
    primitives::{BooleanShape, IntoShape, Shape, Solid},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let cube = hollow_cube(48.0, 23.0, 5.0);

    cube
}

fn hollow_cube(edge_length: f64, hole_diameter: f64, wall_thickness: f64) -> Shape {
    let wp = Workplane::xy();
    let wp = wp.translated(DVec3::new(0.0, 0.0, -0.5 * edge_length));

    let wire = wp.rect(edge_length, edge_length);
    let cube = wire.to_face().extrude(DVec3::new(0.0, 0.0, edge_length)).into_shape();

    let p1 = rolling_pin(
        &Workplane::xy(),
        0.5 * hole_diameter,
        edge_length,
        0.5 * edge_length - wall_thickness,
        edge_length - 2.0 * wall_thickness,
    );
    let p2 = rolling_pin(
        &Workplane::yz(),
        0.5 * hole_diameter,
        edge_length,
        0.5 * edge_length - wall_thickness,
        edge_length - 2.0 * wall_thickness,
    );
    let p3 = rolling_pin(
        &Workplane::zx(),
        0.5 * hole_diameter,
        edge_length,
        0.5 * edge_length - wall_thickness,
        edge_length - 2.0 * wall_thickness,
    );

    let cutout = p1.union(&p2).union(&p3).into_shape();

    cube.subtract(&cutout).into_shape()
}

fn rolling_pin(wp: &Workplane, r1: f64, h1: f64, r2: f64, h2: f64) -> Shape {
    let c_long = cylinder(&wp, r1, h1);
    let c_wide = cylinder(&wp, r2, h2);
    c_long.union(&c_wide).into_shape()
}

fn cylinder(wp: &Workplane, radius: f64, height: f64) -> Shape {
    let wp = wp.translated(DVec3::new(0.0, 0.0, -0.5 * height));
    let wire = wp.circle(0.0, 0.0, radius);
    wire.to_face().extrude(height * wp.normal()).into_shape()
}
