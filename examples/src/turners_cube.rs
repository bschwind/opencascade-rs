use glam::DVec3;
use opencascade::{
    primitives::{IntoShape, Shape},
    workplane::Workplane,
};

pub fn shape() -> Shape {
    let edge_length_1 = 48.0;
    let hole_diamet_1 = 23.0;
    let wall_thickn_1 = 5.0;

    let edge_length_2 = edge_len_from_circum_diam(edge_length_1 - 2.0 * wall_thickn_1);
    let hole_diamet_2 = 11.0;
    let wall_thickn_2 = 4.0;

    let edge_length_3 = edge_len_from_circum_diam(edge_length_2 - 2.0 * wall_thickn_2);

    let c1 = hollow_cube(edge_length_1, hole_diamet_1, wall_thickn_1);
    let c2 = hollow_cube(edge_length_2, hole_diamet_2, wall_thickn_2);
    let c3 = cube(edge_length_3);

    c1.union(&c2).union(&c3).into_shape()
}

fn edge_len_from_circum_diam(d: f64) -> f64 {
    d / f64::sqrt(2.0)
}

fn hollow_cube(edge_length: f64, hole_diameter: f64, wall_thickness: f64) -> Shape {
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

    cube(edge_length).subtract(&cutout).into_shape()
}

fn cube(edge_length: f64) -> Shape {
    let wp = Workplane::xy();
    let wp = wp.translated(DVec3::new(0.0, 0.0, -0.5 * edge_length));
    let wire = wp.rect(edge_length, edge_length);
    wire.to_face().extrude(DVec3::new(0.0, 0.0, edge_length)).into_shape()
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
