pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/top_exp.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;
        type TopoDS_Edge = crate::ffi::TopoDS_Edge;
        type TopoDS_Vertex = crate::ffi::TopoDS_Vertex;
        type TopoDS_Wire = crate::ffi::TopoDS_Wire;
        type TopAbs_ShapeEnum = crate::ffi::TopAbs_ShapeEnum;

        type TopExp_Explorer;
        #[cxx_name = "construct_unique"]
        pub fn TopExp_Explorer_ctor(
            shape: &TopoDS_Shape,
            to_find: TopAbs_ShapeEnum,
        ) -> UniquePtr<TopExp_Explorer>;

        pub fn More(self: &TopExp_Explorer) -> bool;
        pub fn Next(self: Pin<&mut TopExp_Explorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExp_Explorer) -> UniquePtr<TopoDS_Shape>;
        pub fn Current(self: &TopExp_Explorer) -> &TopoDS_Shape;

        pub fn TopExp_FirstVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        pub fn TopExp_LastVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        pub fn TopExp_EdgeVertices(
            edge: &TopoDS_Edge,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
        );
        pub fn TopExp_WireVertices(
            wire: &TopoDS_Wire,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
        );
        pub fn TopExp_CommonVertex(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            vertex: Pin<&mut TopoDS_Vertex>,
        ) -> bool;
    }
}

unsafe impl Send for inner::TopExp_Explorer {}
