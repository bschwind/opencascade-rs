use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::ffi::{
    cast_face_to_shape, cast_solid_to_shape, cast_wire_to_shape, gp_Ax1_ctor, gp_Dir, gp_Dir_ctor,
    gp_Pnt, map_shapes, new_HandleGeomCurve_from_HandleGeom_TrimmedCurve, new_indexed_map_of_shape,
    new_point, new_transform, new_vec, outer_wire, write_stl,
    BRepBuilderAPI_MakeEdge_HandleGeomCurve, BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt,
    BRepBuilderAPI_MakeFace_wire, BRepBuilderAPI_MakeVertex_gp_Pnt, BRepBuilderAPI_MakeWire_ctor,
    BRepBuilderAPI_Transform_ctor, BRepFilletAPI_MakeFillet2d_add_fillet,
    BRepFilletAPI_MakeFillet2d_ctor, BRepFilletAPI_MakeFillet_ctor, BRepMesh_IncrementalMesh_ctor,
    BRepOffsetAPI_ThruSections_ctor, BRepPrimAPI_MakePrism_ctor, GC_MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point, Message_ProgressRange_ctor, StlAPI_Writer_ctor,
    TopAbs_ShapeEnum, TopExp_Explorer_ctor, TopoDS_Compound, TopoDS_Compound_to_owned, TopoDS_Edge,
    TopoDS_Edge_to_owned, TopoDS_Face, TopoDS_Face_to_owned, TopoDS_Shape, TopoDS_Shell,
    TopoDS_Solid, TopoDS_Solid_to_owned, TopoDS_Vertex, TopoDS_Vertex_to_owned, TopoDS_Wire,
    TopoDS_Wire_to_owned, TopoDS_cast_to_compound, TopoDS_cast_to_face, TopoDS_cast_to_solid,
    TopoDS_cast_to_vertex, TopoDS_cast_to_wire,
};
use std::path::Path;

pub fn make_point(p: DVec3) -> UniquePtr<gp_Pnt> {
    new_point(p.x, p.y, p.z)
}

pub fn make_dir(p: DVec3) -> UniquePtr<gp_Dir> {
    gp_Dir_ctor(p.x, p.y, p.z)
}

pub struct Vertex {
    inner: UniquePtr<TopoDS_Vertex>,
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex = BRepBuilderAPI_MakeVertex_gp_Pnt(&make_point(point));
        let vertex = make_vertex.pin_mut().Vertex();
        let inner = TopoDS_Vertex_to_owned(vertex);

        Self { inner }
    }
}

pub struct Edge {
    inner: UniquePtr<TopoDS_Edge>,
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let mut make_edge = BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(&make_point(p1), &make_point(p2));
        let edge = make_edge.pin_mut().Edge();
        let inner = TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn circle() {}

    pub fn ellipse() {}

    pub fn spline() {}

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let make_arc =
            GC_MakeArcOfCircle_point_point_point(&make_point(p1), &make_point(p2), &make_point(p3));

        let mut make_edge = BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            &new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(&GC_MakeArcOfCircle_Value(&make_arc)),
        );

        let edge = make_edge.pin_mut().Edge();
        let inner = TopoDS_Edge_to_owned(edge);

        Self { inner }
    }

    pub fn tangent_arc(p1: DVec3, tangent: DVec3, p3: DVec3) {}
}

pub struct Wire {
    inner: UniquePtr<TopoDS_Wire>,
}

impl Wire {
    pub fn from_edges<'a>(edges: impl Iterator<Item = &'a Edge>) -> Self {
        let mut make_wire = BRepBuilderAPI_MakeWire_ctor();

        for edge in edges {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn from_wires<'a>(wires: impl Iterator<Item = &'a Wire>) -> Self {
        let mut make_wire = BRepBuilderAPI_MakeWire_ctor();

        for wire in wires {
            make_wire.pin_mut().add_wire(&wire.inner);
        }

        let wire = make_wire.pin_mut().Wire();
        let inner = TopoDS_Wire_to_owned(wire);

        Self { inner }
    }

    pub fn mirror_along_axis(&self, axis_origin: DVec3, axis_dir: DVec3) -> Self {
        let axis_dir = make_dir(axis_dir);
        let axis = gp_Ax1_ctor(&make_point(axis_origin), &axis_dir);

        let mut transform = new_transform();

        transform.pin_mut().set_mirror_axis(&axis);

        let wire_shape = cast_wire_to_shape(&self.inner);

        let mut brep_transform = BRepBuilderAPI_Transform_ctor(wire_shape, &transform, false);

        let mirrored_shape = brep_transform.pin_mut().Shape();
        let mirrored_wire = TopoDS_cast_to_wire(mirrored_shape);
        let inner = TopoDS_Wire_to_owned(mirrored_wire);

        Self { inner }
    }

    pub fn rect(width: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let p1 = dvec3(-half_width, half_height, 0.0);
        let p2 = dvec3(half_width, half_height, 0.0);
        let p3 = dvec3(half_width, -half_height, 0.0);
        let p4 = dvec3(-half_width, -half_height, 0.0);

        let top = Edge::segment(p1, p2);
        let right = Edge::segment(p2, p3);
        let bottom = Edge::segment(p3, p4);
        let left = Edge::segment(p4, p1);

        Self::from_edges([&top, &right, &bottom, &left].into_iter())
    }

    pub fn fillet(&mut self, radius: f64) {
        // Create a face from this wire
        let face = Face::from_wire(self);
        // use BRepFilletAPI_MakeFillet2d
        let mut make_fillet = BRepFilletAPI_MakeFillet2d_ctor(&face.inner);

        // add all vertices from the face
        let face_shape = cast_face_to_shape(&face.inner);

        // We use a shape map here to avoid duplicates.
        let mut shape_map = new_indexed_map_of_shape();
        map_shapes(face_shape, TopAbs_ShapeEnum::TopAbs_VERTEX, shape_map.pin_mut());

        for i in 1..=shape_map.Extent() {
            let vertex = TopoDS_cast_to_vertex(shape_map.FindKey(i));
            BRepFilletAPI_MakeFillet2d_add_fillet(make_fillet.pin_mut(), vertex, radius);
        }

        make_fillet.pin_mut().Build(&Message_ProgressRange_ctor());

        let result_shape = make_fillet.pin_mut().Shape();
        // convert back to a wire with BRepTools::OuterWire

        let result_face = TopoDS_cast_to_face(result_shape);
        let wire = outer_wire(result_face);

        self.inner = wire;
    }

    // Create a closure-based API
    pub fn freeform() {}
}

pub struct Face {
    inner: UniquePtr<TopoDS_Face>,
}

impl Face {
    pub fn from_wire(wire: &Wire) -> Self {
        let only_plane = false;
        let make_face = BRepBuilderAPI_MakeFace_wire(&wire.inner, only_plane);

        let face = make_face.Face();
        let inner = TopoDS_Face_to_owned(face);

        Self { inner }
    }

    pub fn extrude(&self, dir: DVec3) -> Solid {
        let prism_vec = new_vec(dir.x, dir.y, dir.z);

        let copy = false;
        let canonize = true;

        let inner_shape = cast_face_to_shape(&self.inner);
        let mut make_solid = BRepPrimAPI_MakePrism_ctor(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().Shape();
        let solid = TopoDS_cast_to_solid(extruded_shape);
        let inner = TopoDS_Solid_to_owned(solid);

        Solid { inner }
    }
}

pub struct Shell {
    inner: UniquePtr<TopoDS_Shell>,
}

pub struct Solid {
    inner: UniquePtr<TopoDS_Solid>,
}

impl Solid {
    // TODO(bschwind) - Do some cool stuff from this link:
    // https://neweopencascade.wordpress.com/2018/10/17/lets-talk-about-fillets/
    // Key takeaway: Use the `SectionEdges` function to retrieve edges that were
    // the result of combining two shapes.
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let inner_shape = cast_solid_to_shape(&self.inner);

        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(inner_shape);
        make_fillet.pin_mut().add_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.pin_mut().Shape();

        let compund = TopoDS_cast_to_compound(filleted_shape);
        let inner = TopoDS_Compound_to_owned(compund);

        Compound { inner }
    }

    pub fn loft<'a>(wires: impl Iterator<Item = &'a Wire>) -> Self {
        let is_solid = true;
        let mut make_loft = BRepOffsetAPI_ThruSections_ctor(is_solid);

        for wire in wires {
            // make_wire.pin_mut().add_wire(&wire.inner);
            make_loft.pin_mut().AddWire(&wire.inner);
        }

        // TODO(bschwind) - is this needed?
        make_loft.pin_mut().CheckCompatibility(false);

        let shape = make_loft.pin_mut().Shape();
        let solid = TopoDS_cast_to_solid(shape);
        let inner = TopoDS_Solid_to_owned(solid);

        Self { inner }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), ()> {
        let inner_shape = cast_solid_to_shape(&self.inner);

        let mut stl_writer = StlAPI_Writer_ctor();
        let triangulation = BRepMesh_IncrementalMesh_ctor(inner_shape, 0.001);
        let success = write_stl(
            stl_writer.pin_mut(),
            triangulation.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        if success {
            Ok(())
        } else {
            Err(()) // TODO(bschwind) - Make an error type
        }
    }
}

pub struct Compound {
    inner: UniquePtr<TopoDS_Compound>,
}

pub struct Shape {
    inner: UniquePtr<TopoDS_Shape>,
}
