pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("b_rep_tools.hxx");

        type TopoDS_Face = crate::ffi::TopoDS_Face;
        type TopoDS_Wire = crate::ffi::TopoDS_Wire;

        type BRepTools;

        fn outer_wire(face: &TopoDS_Face) -> UniquePtr<TopoDS_Wire>;
    }
}
