pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/geom.hxx");

        type gp_Ax3 = crate::gp::gp_Ax3;
        type gp_Pnt = crate::gp::gp_Pnt;
        type Handle_Geom_CylindricalSurface = crate::ffi::Handle_Geom_CylindricalSurface;
        type Handle_Geom_Surface = crate::ffi::Handle_Geom_Surface;
        type Handle_Geom_BezierSurface = crate::ffi::Handle_Geom_BezierSurface;
        type Handle_Geom_BezierCurve = crate::ffi::Handle_Geom_BezierCurve;
        type Handle_Geom_Plane = crate::ffi::Handle_Geom_Plane;
        type TColgp_Array2OfPnt = crate::t_col_gp::TColgp_Array2OfPnt;
        type TColgp_HArray1OfPnt = crate::t_col_gp::TColgp_HArray1OfPnt;

        type Geom_TrimmedCurve;

        type Geom_CylindricalSurface;
        pub fn Geom_CylindricalSurface_ctor(
            axis: &gp_Ax3,
            radius: f64,
        ) -> UniquePtr<Handle_Geom_CylindricalSurface>;
        pub fn cylinder_to_surface(
            cylinder_handle: &Handle_Geom_CylindricalSurface,
        ) -> UniquePtr<Handle_Geom_Surface>;

        type Geom_BezierSurface;
        pub fn Geom_BezierSurface_ctor(
            poles: &TColgp_Array2OfPnt,
        ) -> UniquePtr<Handle_Geom_BezierSurface>;
        pub fn bezier_to_surface(
            bezier_handle: &Handle_Geom_BezierSurface,
        ) -> UniquePtr<Handle_Geom_Surface>;

        type Geom_BezierCurve;
        #[cxx_name = "construct_unique"]
        pub fn Geom_BezierCurve_ctor_points(
            poles: &TColgp_HArray1OfPnt,
        ) -> UniquePtr<Geom_BezierCurve>;

        pub fn Geom_BezierCurve_to_handle(
            law: UniquePtr<Geom_BezierCurve>,
        ) -> UniquePtr<Handle_Geom_BezierCurve>;

        pub fn handle_geom_plane_location(plane: &Handle_Geom_Plane) -> &gp_Pnt;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_Plane>;
    }
}
