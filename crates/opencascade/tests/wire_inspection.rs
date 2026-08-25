use glam::DVec3;
use opencascade::primitives::{Edge, Wire};

#[test]
fn rectangle_length() {
    let wire = Wire::rect(7.0, 5.0);
    let length = wire.length();
    approx_equal(length, 24.0);
}

#[test]
fn closed_wire_returns_true() {
    let wire = Wire::rect(10.0, 10.0);
    assert!(wire.is_closed());
}

#[test]
fn open_wire_returns_false() {
    let edge = Edge::segment(DVec3::ZERO, DVec3::new(10.0, 0.0, 0.0));
    let wire = Wire::from_edges([&edge]);
    assert!(!wire.is_closed());
}

fn approx_equal(a: f64, b: f64) {
    let diff = (a - b).abs();
    let rel = diff / b.abs().max(1e-12);
    assert!(rel < 1e-4 || diff < 1e-6, "expected {b}, got {a} (diff={diff}, rel={rel})");
}
