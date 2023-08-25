use crate::primitives::{Face, Shape, Solid, Wire};
use glam::{dvec3, DVec3};
use opencascade_sys::ffi;

/// Collections of helper functions for the [`Shape`] struct that provides an "ad-hoc"
/// API. New code is encouraged to use more fine-grained API in the [`primitives`] module.
pub struct AdHocShape;

impl AdHocShape {
    /// Make a box with a corner at (0,0,0) and with size (x,y,z)
    pub fn make_box(x: f64, y: f64, z: f64) -> Shape {
        let point = ffi::new_point(0.0, 0.0, 0.0);
        let mut my_box = ffi::BRepPrimAPI_MakeBox_ctor(&point, x, y, z);

        Shape::from_shape(my_box.pin_mut().Shape())
    }

    /// Make a box with one corner at p1, and the opposite corner
    /// at p2.
    pub fn make_box_point_point(p1: DVec3, p2: DVec3) -> Shape {
        let min_corner = p1.min(p2);
        let max_corner = p1.max(p2);

        let point = ffi::new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box = ffi::BRepPrimAPI_MakeBox_ctor(&point, diff.x, diff.y, diff.z);

        Shape::from_shape(my_box.pin_mut().Shape())
    }

    /// Make a cylinder with its bottom at point p, with radius r and height h.
    pub fn make_cylinder(p: DVec3, r: f64, h: f64) -> Shape {
        let point = ffi::new_point(p.x, p.y, p.z);
        let cylinder_axis = ffi::gp_DZ();
        let cylinder_coord_system = ffi::gp_Ax2_ctor(&point, cylinder_axis);

        let mut cylinder = ffi::BRepPrimAPI_MakeCylinder_ctor(&cylinder_coord_system, r, h);

        Shape::from_shape(cylinder.pin_mut().Shape())
    }

    /// Purposefully underpowered for now, this simply takes a list of points,
    /// creates a face out of them, and then extrudes it by h in the positive Z
    /// direction.
    pub fn extrude_polygon(points: &[DVec3], h: f64) -> Solid {
        assert!(points.len() >= 3);

        let wire = Wire::from_ordered_points(points);
        Face::from_wire(&wire).extrude(dvec3(0.0, 0.0, h))
    }

    /// Drills a cylindrical hole starting at point p, pointing down the Z axis
    /// (this will later change to be an arbitrary axis).
    pub fn drill_hole(shape: &Shape, p: DVec3, dir: DVec3, radius: f64) -> Shape {
        let point = ffi::new_point(p.x, p.y, p.z);
        let dir = ffi::gp_Dir_ctor(dir.x, dir.y, dir.z);

        let hole_axis = ffi::gp_Ax1_ctor(&point, &dir);

        let mut make_hole = ffi::BRepFeat_MakeCylindricalHole_ctor();
        make_hole.pin_mut().Init(&shape.inner, &hole_axis);

        make_hole.pin_mut().Perform(radius);
        make_hole.pin_mut().Build();

        Shape::from_shape(make_hole.pin_mut().Shape())
    }
}
