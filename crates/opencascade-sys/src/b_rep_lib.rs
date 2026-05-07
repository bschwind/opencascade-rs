pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_lib.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type Handle_Poly_Triangulation = crate::poly::Handle_Poly_Triangulation;

        type BRepLib;
        #[Self = "BRepLib"]
        pub fn BuildCurves3d(shape: &TopoDS_Shape) -> bool;

        type BRepLib_ToolTriangulatedShape;
        #[Self = "BRepLib_ToolTriangulatedShape"]
        pub fn ComputeNormals(face: &TopoDS_Face, triangulation: &Handle_Poly_Triangulation);
    }
}
