use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi::{
    gp_Ax2_ctor, gp_DZ, new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_point, new_shape,
    new_vec, write_stl, BRepAlgoAPI_Common_ctor, BRepAlgoAPI_Cut_ctor, BRepAlgoAPI_Fuse_ctor,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve, BRepBuilderAPI_MakeFace_wire, BRepBuilderAPI_MakeWire,
    BRepBuilderAPI_MakeWire_ctor, BRepFilletAPI_MakeChamfer_ctor, BRepFilletAPI_MakeFillet_ctor,
    BRepMesh_IncrementalMesh_ctor, BRepPrimAPI_MakeBox_ctor, BRepPrimAPI_MakeCylinder_ctor,
    BRepPrimAPI_MakePrism_ctor, GC_MakeSegment_Value, GC_MakeSegment_point_point,
    StlAPI_Writer_ctor, TopAbs_ShapeEnum, TopExp_Explorer_ctor, TopoDS_Shape,
    TopoDS_Shape_to_owned, TopoDS_cast_to_edge,
};
use std::path::Path;

pub use glam;

pub struct Shape {
    shape: UniquePtr<TopoDS_Shape>,
}

impl Shape {
    /// Make a box with a corner at (0,0,0) and with size (x,y,z)
    pub fn make_box(x: f64, y: f64, z: f64) -> Self {
        let point = new_point(0.0, 0.0, 0.0);
        let mut my_box = BRepPrimAPI_MakeBox_ctor(&point, x, y, z);
        let shape = TopoDS_Shape_to_owned(my_box.pin_mut().Shape());

        Self { shape }
    }

    /// Make a box with one corner at p1, and the opposite corner
    /// at p2.
    pub fn make_box_point_point(p1: DVec3, p2: DVec3) -> Self {
        let min_corner = p1.min(p2);
        let max_corner = p1.max(p2);

        let point = new_point(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut my_box = BRepPrimAPI_MakeBox_ctor(&point, diff.x, diff.y, diff.z);
        let shape = new_shape(my_box.pin_mut().Shape());

        Self { shape }
    }

    /// Make a cylinder with its bottom at point p, with radius r and height h.
    pub fn make_cylinder(p: DVec3, r: f64, h: f64) -> Self {
        let point = new_point(p.x, p.y, p.z);
        let cylinder_axis = gp_DZ();
        let cylinder_coord_system = gp_Ax2_ctor(&point, cylinder_axis);

        let mut cylinder = BRepPrimAPI_MakeCylinder_ctor(&cylinder_coord_system, r, h);
        let shape = new_shape(cylinder.pin_mut().Shape());

        Self { shape }
    }

    /// Purposefully underpowered for now, this simply takes a list of points,
    /// creates a face out of them, and then extrudes it by h in the positive Z
    /// direction.
    pub fn extrude_polygon(points: &[DVec3], h: f64) -> Shape {
        assert!(points.len() >= 3);

        let mut make_wire = BRepBuilderAPI_MakeWire_ctor();

        let add_segment =
            |p1: DVec3, p2: DVec3, make_wire: &mut UniquePtr<BRepBuilderAPI_MakeWire>| {
                let p1 = new_point(p1.x, p1.y, p1.z);
                let p2 = new_point(p2.x, p2.y, p2.z);

                let segment = GC_MakeSegment_point_point(&p1, &p2);
                let mut edge = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
                    &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeSegment_Value(
                        &segment,
                    )),
                );

                make_wire.pin_mut().add_edge(edge.pin_mut().Edge());
            };

        for window in points.windows(2) {
            add_segment(window[0], window[1], &mut make_wire);
        }

        add_segment(*points.last().unwrap(), points[0], &mut make_wire);

        let wire_profile = make_wire.pin_mut().Wire();
        let mut face_profile = BRepBuilderAPI_MakeFace_wire(wire_profile, false);
        let prism_vec = new_vec(0.0, 0.0, h);
        let mut extrusion =
            BRepPrimAPI_MakePrism_ctor(face_profile.pin_mut().Shape(), &prism_vec, false, true);

        let shape = new_shape(extrusion.pin_mut().Shape());

        Self { shape }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) {
        let mut stl_writer = StlAPI_Writer_ctor();
        let triangulation = BRepMesh_IncrementalMesh_ctor(&self.shape, 0.001);
        let success = write_stl(
            stl_writer.pin_mut(),
            triangulation.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        println!("Done! Success = {success}");
    }

    pub fn fillet_edges(&mut self, radius: f64) {
        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(&self.shape);
        let mut edge_explorer = TopExp_Explorer_ctor(&self.shape, TopAbs_ShapeEnum::TopAbs_EDGE);

        while edge_explorer.More() {
            let edge = TopoDS_cast_to_edge(edge_explorer.Current());
            make_fillet.pin_mut().add_edge(radius, edge);
            edge_explorer.pin_mut().Next();
        }

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.shape = TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn chamfer_edges(&mut self, distance: f64) {
        let mut make_chamfer = BRepFilletAPI_MakeChamfer_ctor(&self.shape);
        let mut edge_explorer = TopExp_Explorer_ctor(&self.shape, TopAbs_ShapeEnum::TopAbs_EDGE);

        while edge_explorer.More() {
            let edge = TopoDS_cast_to_edge(edge_explorer.Current());
            make_chamfer.pin_mut().add_edge(distance, edge);
            edge_explorer.pin_mut().Next();
        }

        let filleted_shape = make_chamfer.pin_mut().Shape();

        self.shape = TopoDS_Shape_to_owned(filleted_shape);
    }

    pub fn subtract(&mut self, other: &Shape) {
        let mut cut_operation = BRepAlgoAPI_Cut_ctor(&self.shape, &other.shape);

        let cut_shape = cut_operation.pin_mut().Shape();
        self.shape = TopoDS_Shape_to_owned(cut_shape);
    }

    pub fn union(&mut self, other: &Shape) {
        let mut fuse_operation = BRepAlgoAPI_Fuse_ctor(&self.shape, &other.shape);

        let cut_shape = fuse_operation.pin_mut().Shape();
        self.shape = new_shape(cut_shape);
    }

    pub fn intersect(&mut self, other: &Shape) {
        let mut common_operation = BRepAlgoAPI_Common_ctor(&self.shape, &other.shape);

        let cut_shape = common_operation.pin_mut().Shape();
        self.shape = new_shape(cut_shape);
    }
}
