use crate::primitives::Shape;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;
use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

/// Wrapper for the [`Shape`] struct that provides an "ad-hoc" API. New code is encouraged
/// to use more fine-grained API in the [`primitives`] module.
pub struct AdHocShape(pub Shape);

impl Deref for AdHocShape {
    type Target = Shape;

    fn deref(&self) -> &Shape {
        &self.0
    }
}

impl DerefMut for AdHocShape {
    fn deref_mut(&mut self) -> &mut Shape {
        &mut self.0
    }
}

impl AdHocShape {
    /// Internal helper to create [Self] from the FFI inner type.
    fn from_inner(inner: UniquePtr<ffi::TopoDS_Shape>) -> Self {
        Self(Shape { inner })
    }

    /// Make a box with a corner at (0,0,0) and with size (x,y,z)
    pub fn make_box(x: f64, y: f64, z: f64) -> Self {
        let point = ffi::new_point(0.0, 0.0, 0.0);
        let mut my_box = ffi::BRepPrimAPI_MakeBox_ctor(&point, x, y, z);
        let inner = ffi::TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Make a box with one corner at p1, and the opposite corner
    /// at p2.
    pub fn make_box_point_point(p1: DVec3, p2: DVec3) -> Self {
        let min_corner = p1.min(p2);
        let max_corner = p1.max(p2);

        let point = ffi::new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box = ffi::BRepPrimAPI_MakeBox_ctor(&point, diff.x, diff.y, diff.z);
        let inner = ffi::TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Make a cylinder with its bottom at point p, with radius r and height h.
    pub fn make_cylinder(p: DVec3, r: f64, h: f64) -> Self {
        let point = ffi::new_point(p.x, p.y, p.z);
        let cylinder_axis = ffi::gp_DZ();
        let cylinder_coord_system = ffi::gp_Ax2_ctor(&point, cylinder_axis);

        let mut cylinder = ffi::BRepPrimAPI_MakeCylinder_ctor(&cylinder_coord_system, r, h);
        let inner = ffi::TopoDS_Shape_to_owned(cylinder.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Purposefully underpowered for now, this simply takes a list of points,
    /// creates a face out of them, and then extrudes it by h in the positive Z
    /// direction.
    pub fn extrude_polygon(points: &[DVec3], h: f64) -> Self {
        assert!(points.len() >= 3);

        let mut make_wire = ffi::BRepBuilderAPI_MakeWire_ctor();

        let add_segment =
            |p1: DVec3, p2: DVec3, make_wire: &mut UniquePtr<ffi::BRepBuilderAPI_MakeWire>| {
                let p1 = ffi::new_point(p1.x, p1.y, p1.z);
                let p2 = ffi::new_point(p2.x, p2.y, p2.z);

                let segment = ffi::GC_MakeSegment_point_point(&p1, &p2);
                let mut edge = ffi::BRepBuilderAPI_MakeEdge_HandleGeomCurve(
                    &ffi::new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
                        &ffi::GC_MakeSegment_Value(&segment),
                    ),
                );

                make_wire.pin_mut().add_edge(edge.pin_mut().Edge());
            };

        for window in points.windows(2) {
            add_segment(window[0], window[1], &mut make_wire);
        }

        add_segment(*points.last().unwrap(), points[0], &mut make_wire);

        let wire_profile = make_wire.pin_mut().Wire();
        let mut face_profile = ffi::BRepBuilderAPI_MakeFace_wire(wire_profile, false);
        let prism_vec = ffi::new_vec(0.0, 0.0, h);
        let mut extrusion = ffi::BRepPrimAPI_MakePrism_ctor(
            face_profile.pin_mut().Shape(),
            &prism_vec,
            false,
            true,
        );

        let inner = ffi::TopoDS_Shape_to_owned(extrusion.pin_mut().Shape());

        Self::from_inner(inner)
    }

    /// Drills a cylindrical hole starting at point p, pointing down the Z axis
    /// (this will later change to be an arbitrary axis).
    pub fn drill_hole(&mut self, p: DVec3, dir: DVec3, radius: f64) {
        let point = ffi::new_point(p.x, p.y, p.z);
        let dir = ffi::gp_Dir_ctor(dir.x, dir.y, dir.z);

        let hole_axis = ffi::gp_Ax1_ctor(&point, &dir);

        // let cylinder_axis = ffi::gp_DZ();
        // let cylinder_coord_system = ffi::gp_Ax2_ctor(&point, cylinder_axis);

        let mut make_hole = ffi::BRepFeat_MakeCylindricalHole_ctor();
        make_hole.pin_mut().Init(&self.inner, &hole_axis);

        make_hole.pin_mut().Perform(radius);
        make_hole.pin_mut().Build();

        // let mut cylinder = ffi::BRepPrimAPI_MakeCylinder_ctor(&cylinder_coord_system, r, h);
        // let shape = ffi::TopoDS_Shape_to_owned(cylinder.pin_mut().Shape());

        // Self { shape }

        self.inner = ffi::TopoDS_Shape_to_owned(make_hole.pin_mut().Shape());
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) {
        let mut stl_writer = ffi::StlAPI_Writer_ctor();
        let triangulation = ffi::BRepMesh_IncrementalMesh_ctor(&self.inner, 0.001);
        let success = ffi::write_stl(
            stl_writer.pin_mut(),
            triangulation.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        println!("Done! Success = {success}");
    }

    pub fn fillet_edges(&mut self, radius: f64) {
        let mut make_fillet = ffi::BRepFilletAPI_MakeFillet_ctor(&self.inner);
        let mut edge_explorer =
            ffi::TopExp_Explorer_ctor(&self.inner, ffi::TopAbs_ShapeEnum::TopAbs_EDGE);

        while edge_explorer.More() {
            let edge = ffi::TopoDS_cast_to_edge(edge_explorer.Current());
            make_fillet.pin_mut().add_edge(radius, edge);
            edge_explorer.pin_mut().Next();
        }

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edges(&mut self, distance: f64) {
        let mut make_chamfer = ffi::BRepFilletAPI_MakeChamfer_ctor(&self.inner);
        let mut edge_explorer =
            ffi::TopExp_Explorer_ctor(&self.inner, ffi::TopAbs_ShapeEnum::TopAbs_EDGE);

        while edge_explorer.More() {
            let edge = ffi::TopoDS_cast_to_edge(edge_explorer.Current());
            make_chamfer.pin_mut().add_edge(distance, edge);
            edge_explorer.pin_mut().Next();
        }

        let filleted_shape = make_chamfer.pin_mut().Shape();

        self.inner = ffi::TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn subtract(&mut self, other: &Shape) {
        let mut cut_operation = ffi::BRepAlgoAPI_Cut_ctor(&self.inner, &other.inner);

        let cut_shape = cut_operation.pin_mut().Shape();
        self.inner = ffi::TopoDS_Shape_to_owned(cut_shape);
    }

    pub fn union(&mut self, other: &Shape) {
        let mut fuse_operation = ffi::BRepAlgoAPI_Fuse_ctor(&self.inner, &other.inner);

        let cut_shape = fuse_operation.pin_mut().Shape();
        self.inner = ffi::TopoDS_Shape_to_owned(cut_shape);
    }

    pub fn intersect(&mut self, other: &Shape) {
        let mut common_operation = ffi::BRepAlgoAPI_Common_ctor(&self.inner, &other.inner);

        let cut_shape = common_operation.pin_mut().Shape();
        self.inner = ffi::TopoDS_Shape_to_owned(cut_shape);
    }
}
