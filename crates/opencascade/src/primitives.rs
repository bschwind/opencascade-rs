use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi::{
    new_point, BRepBuilderAPI_MakeVertex_gp_Pnt, TopoDS_Edge, TopoDS_Face, TopoDS_Shape,
    TopoDS_Shell, TopoDS_Solid, TopoDS_Vertex, TopoDS_Vertex_to_owned, TopoDS_Wire,
};

pub struct Vertex {
    internal: UniquePtr<TopoDS_Vertex>,
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex =
            BRepBuilderAPI_MakeVertex_gp_Pnt(&new_point(point.x, point.y, point.z));
        let vertex = make_vertex.pin_mut().Vertex();
        let internal = TopoDS_Vertex_to_owned(vertex);

        Self { internal }
    }
}

pub struct Edge {
    internal: UniquePtr<TopoDS_Edge>,
}

impl Edge {
    pub fn segment(p1: DVec3, p2: DVec3) {}

    pub fn circle() {}

    pub fn ellipse() {}

    pub fn spline() {}

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) {}
}

pub struct Wire {
    inner: UniquePtr<TopoDS_Wire>,
}

pub struct Face {
    inner: UniquePtr<TopoDS_Face>,
}

pub struct Shell {
    inner: UniquePtr<TopoDS_Shell>,
}

pub struct Solid {
    inner: UniquePtr<TopoDS_Solid>,
}

pub struct Shape {
    inner: UniquePtr<TopoDS_Shape>,
}
