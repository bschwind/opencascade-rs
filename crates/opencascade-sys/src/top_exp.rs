pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/top_exp.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Edge = crate::topo_ds::TopoDS_Edge;
        type TopoDS_Vertex = crate::topo_ds::TopoDS_Vertex;
        type TopoDS_Wire = crate::topo_ds::TopoDS_Wire;
        type TopAbs_ShapeEnum = crate::top_abs::TopAbs_ShapeEnum;
        type TopTools_IndexedMapOfShape = crate::top_tools::TopTools_IndexedMapOfShape;
        type TopTools_IndexedDataMapOfShapeListOfShape =
            crate::top_tools::TopTools_IndexedDataMapOfShapeListOfShape;

        type TopExp_Explorer;
        #[cxx_name = "construct_unique"]
        pub fn TopExp_Explorer_new(
            shape: &TopoDS_Shape,
            to_find: TopAbs_ShapeEnum,
        ) -> UniquePtr<TopExp_Explorer>;
        pub fn More(self: &TopExp_Explorer) -> bool;
        pub fn Next(self: Pin<&mut TopExp_Explorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExp_Explorer) -> UniquePtr<TopoDS_Shape>;
        pub fn Current(self: &TopExp_Explorer) -> &TopoDS_Shape;

        type TopExp;
        #[Self = "TopExp"]
        #[cxx_name = "Vertices"]
        pub fn EdgeVertices(
            edge: &TopoDS_Edge,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
            with_orientation: bool,
        );
        #[Self = "TopExp"]
        #[cxx_name = "Vertices"]
        pub fn TopExp_WireVertices(
            wire: &TopoDS_Wire,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
        );
        pub fn TopExp_FirstVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        pub fn TopExp_LastVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        #[Self = "TopExp"]
        pub fn CommonVertex(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            vertex: Pin<&mut TopoDS_Vertex>,
        ) -> bool;
        #[Self = "TopExp"]
        pub fn MapShapes(
            shape: &TopoDS_Shape,
            shape_type: TopAbs_ShapeEnum,
            shape_map: Pin<&mut TopTools_IndexedMapOfShape>,
        );
        #[Self = "TopExp"]
        pub fn MapShapesAndAncestors(
            shape: &TopoDS_Shape,
            parent_type: TopAbs_ShapeEnum,
            child_type: TopAbs_ShapeEnum,
            shape_data_map: Pin<&mut TopTools_IndexedDataMapOfShapeListOfShape>,
        );
        #[Self = "TopExp"]
        pub fn MapShapesAndUniqueAncestors(
            shape: &TopoDS_Shape,
            parent_type: TopAbs_ShapeEnum,
            child_type: TopAbs_ShapeEnum,
            shape_data_map: Pin<&mut TopTools_IndexedDataMapOfShapeListOfShape>,
            use_orientation: bool,
        );
    }
}

unsafe impl Send for inner::TopExp_Explorer {}
