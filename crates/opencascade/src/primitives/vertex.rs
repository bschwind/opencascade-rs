use crate::primitives::make_point;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi::{self};

pub struct Vertex {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Vertex>,
}

// You'll see several of these `impl AsRef` blocks for the various primitive
// geometry types. This is for functions which take an Iterator of primitives
// which are either owned or borrowed values. The general pattern looks like this:
//
//     pub fn do_something_with_edges<T: AsRef<Edge>>(edges: impl IntoIterator<Item = T>) {
//         for edge in edges.into_iter() {
//             let edge_ref = edge.as_ref();
//             // Do something with edge_ref
//         }
//     }
impl AsRef<Vertex> for Vertex {
    fn as_ref(&self) -> &Vertex {
        self
    }
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex = ffi::BRepBuilderAPI_MakeVertex_gp_Pnt(&make_point(point));
        let vertex = make_vertex.pin_mut().Vertex();
        let inner = ffi::TopoDS_Vertex_to_owned(vertex);

        Self { inner }
    }
}
