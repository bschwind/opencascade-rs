pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_lib.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type Handle_Poly_Triangulation = crate::ffi::Handle_Poly_Triangulation;

        type BRepLib;
        pub fn BRepLibBuildCurves3d(shape: &TopoDS_Shape) -> bool;

        type BRepLib_ToolTriangulatedShape;

        pub fn compute_normals(face: &TopoDS_Face, triangulation: &Handle_Poly_Triangulation);
    }
}
