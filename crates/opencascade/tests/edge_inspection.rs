use glam::DVec3;
use opencascade::primitives::{Edge, PositionMode};

#[test]
fn line_segment_length() {
    let edge = Edge::segment(DVec3::new(0.0, 0.0, 0.0), DVec3::new(3.0, 4.0, 0.0));
    let length = edge.length();
    approx_equal(length, 5.0);
}

#[test]
fn circle_length() {
    let edge = Edge::circle(DVec3::new(0.0, 0.0, 0.0), DVec3::Z, 5.0);
    let length = edge.length();
    approx_equal(length, std::f64::consts::PI * 10.0);
}

#[test]
fn line_tangent_at_midpoint() {
    let edge = Edge::segment(DVec3::new(0.0, 0.0, 0.0), DVec3::new(10.0, 0.0, 0.0));
    let tangent = edge.tangent_at(0.5, PositionMode::Parameter);
    let expected = DVec3::new(1.0, 0.0, 0.0);
    assert!(tangent.distance_squared(expected) < 1e-6, "expected {expected:?}, got {tangent:?}");
}

#[test]
fn circle_tangent_at_start_perpendicular_to_radius() {
    let edge = Edge::circle(DVec3::new(0.0, 0.0, 0.0), DVec3::Z, 5.0);
    let tangent = edge.tangent_at(0.0, PositionMode::Parameter);
    let radius_vec = edge.start_point() - DVec3::new(0.0, 0.0, 0.0);
    let dot = tangent.dot(radius_vec);
    assert!(dot.abs() < 1e-6, "tangent {tangent:?} dot radius {radius_vec:?} = {dot}, expected ~0");
    assert!(tangent.z.abs() < 1e-6, "tangent {tangent:?} should lie in XY plane");
}

#[test]
fn arc_center_returns_center_for_circle() {
    let center = DVec3::new(10.0, 20.0, 30.0);
    let edge = Edge::circle(center, DVec3::Z, 5.0);
    let result = edge.arc_center();
    assert!(result.is_some(), "Expected Some center for circle edge");
    let result = result.unwrap();
    assert!(result.distance_squared(center) < 1e-6, "expected {center:?}, got {result:?}");
}

#[test]
fn arc_center_returns_none_for_line() {
    let edge = Edge::segment(DVec3::ZERO, DVec3::X);
    let result = edge.arc_center();
    assert!(result.is_none(), "Expected None for line edge, got {result:?}");
}

#[test]
fn radius_returns_value_for_circle() {
    let edge = Edge::circle(DVec3::ZERO, DVec3::Z, 7.0);
    let result = edge.radius();
    assert!(result.is_some(), "Expected Some radius for circle edge");
    approx_equal(result.unwrap(), 7.0);
}

#[test]
fn radius_returns_none_for_line() {
    let edge = Edge::segment(DVec3::ZERO, DVec3::X);
    let result = edge.radius();
    assert!(result.is_none(), "Expected None for line edge, got {result:?}");
}

fn approx_equal(a: f64, b: f64) {
    let diff = (a - b).abs();
    let rel = diff / b.abs().max(1e-12);
    assert!(rel < 1e-4 || diff < 1e-6, "expected {b}, got {a} (diff={diff}, rel={rel})");
}
