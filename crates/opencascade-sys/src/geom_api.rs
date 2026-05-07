pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/geom_api.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Vec = crate::gp::gp_Vec;
        type Handle_TColgp_HArray1OfPnt = crate::t_col_gp::Handle_TColgp_HArray1OfPnt;
        type Handle_Geom_BSplineCurve = crate::geom::Handle_Geom_BSplineCurve;
        type Handle_Geom_Surface = crate::geom::Handle_Geom_Surface;

        type GeomAPI_Interpolate;
        #[cxx_name = "construct_unique"]
        pub fn GeomAPI_Interpolate_new(
            points: &Handle_TColgp_HArray1OfPnt,
            periodic: bool,
            tolerance: f64,
        ) -> UniquePtr<GeomAPI_Interpolate>;
        pub fn Load(
            self: Pin<&mut GeomAPI_Interpolate>,
            initial_tangent: &gp_Vec,
            final_tangent: &gp_Vec,
            scale: bool,
        );
        pub fn Perform(self: Pin<&mut GeomAPI_Interpolate>);
        pub fn GeomAPI_Interpolate_Curve(
            interpolate: &GeomAPI_Interpolate,
        ) -> UniquePtr<Handle_Geom_BSplineCurve>;

        type GeomAPI_ProjectPointOnSurf;
        #[cxx_name = "construct_unique"]
        pub fn GeomAPI_ProjectPointOnSurf_new(
            origin: &gp_Pnt,
            surface: &Handle_Geom_Surface,
        ) -> UniquePtr<GeomAPI_ProjectPointOnSurf>;
        pub fn LowerDistanceParameters(self: &GeomAPI_ProjectPointOnSurf, u: &mut f64, v: &mut f64);
    }
}
