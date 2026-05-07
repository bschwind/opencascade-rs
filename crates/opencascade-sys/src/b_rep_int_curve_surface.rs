// B-Rep Intersect Curve and Surface
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_int_curve_surface.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type gp_Lin = crate::gp::gp_Lin;
        type gp_Pnt = crate::gp::gp_Pnt;

        type BRepIntCurveSurface_Inter;
        #[cxx_name = "construct_unique"]
        pub fn BRepIntCurveSurface_Inter_new() -> UniquePtr<BRepIntCurveSurface_Inter>;
        pub fn Init(
            self: Pin<&mut BRepIntCurveSurface_Inter>,
            shape: &TopoDS_Shape,
            line: &gp_Lin,
            tolerance: f64,
        );
        pub fn More(self: &BRepIntCurveSurface_Inter) -> bool;
        pub fn Next(self: Pin<&mut BRepIntCurveSurface_Inter>);
        pub fn BRepIntCurveSurface_Inter_face(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<TopoDS_Face>;
        pub fn BRepIntCurveSurface_Inter_point(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<gp_Pnt>;
        pub fn U(self: &BRepIntCurveSurface_Inter) -> f64;
        pub fn V(self: &BRepIntCurveSurface_Inter) -> f64;
        pub fn W(self: &BRepIntCurveSurface_Inter) -> f64;
    }
}
