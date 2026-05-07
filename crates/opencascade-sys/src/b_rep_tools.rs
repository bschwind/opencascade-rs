pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_tools.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type TopoDS_Wire = crate::topo_ds::TopoDS_Wire;

        type BRepTools;

        fn outer_wire(face: &TopoDS_Face) -> UniquePtr<TopoDS_Wire>;

        #[cxx_name = "write_brep_text"]
        pub fn write(shape: &TopoDS_Shape, path: String) -> bool;
        #[cxx_name = "read_brep_text"]
        pub fn read(path: String) -> UniquePtr<TopoDS_Shape>;
    }
}
