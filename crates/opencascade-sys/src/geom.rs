pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/geom.hxx");

        type gp_Ax3 = crate::gp::gp_Ax3;
        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Circ = crate::gp::gp_Circ;
        type TColgp_Array2OfPnt = crate::t_col_gp::TColgp_Array2OfPnt;
        type TColgp_HArray1OfPnt = crate::t_col_gp::TColgp_HArray1OfPnt;
        type Handle_Standard_Type = crate::standard::Handle_Standard_Type;

        // Handles
        type Handle_Geom_Curve;
        pub fn IsNull(self: &Handle_Geom_Curve) -> bool;
        pub fn HandleGeomCurve_Value(curve: &Handle_Geom_Curve, u: f64) -> UniquePtr<gp_Pnt>;

        type Handle_Geom_BSplineCurve;
        pub fn IsNull(self: &Handle_Geom_BSplineCurve) -> bool;

        type Handle_Geom_BezierCurve;
        pub fn IsNull(self: &Handle_Geom_BezierCurve) -> bool;

        type Handle_Geom_TrimmedCurve;
        pub fn IsNull(self: &Handle_Geom_TrimmedCurve) -> bool;

        type Handle_Geom_Surface;
        pub fn IsNull(self: &Handle_Geom_Surface) -> bool;
        pub fn DynamicType(surface: &Handle_Geom_Surface) -> &Handle_Standard_Type;

        type Handle_Geom_BezierSurface;
        pub fn IsNull(self: &Handle_Geom_BezierSurface) -> bool;

        type Handle_Geom_Plane;
        pub fn IsNull(self: &Handle_Geom_Plane) -> bool;

        type Handle_Geom_SphericalSurface;
        pub fn IsNull(self: &Handle_Geom_SphericalSurface) -> bool;

        type Handle_Geom_ConicalSurface;
        pub fn IsNull(self: &Handle_Geom_ConicalSurface) -> bool;

        type Handle_Geom_ToroidalSurface;
        pub fn IsNull(self: &Handle_Geom_ToroidalSurface) -> bool;

        type Handle_Geom_Circle;
        pub fn IsNull(self: &Handle_Geom_Circle) -> bool;

        type Handle_Geom_CylindricalSurface;
        pub fn IsNull(self: &Handle_Geom_CylindricalSurface) -> bool;
        // End Handles

        type Geom_TrimmedCurve;

        type Geom_CylindricalSurface;
        pub fn Geom_CylindricalSurface_new(
            axis: &gp_Ax3,
            radius: f64,
        ) -> UniquePtr<Handle_Geom_CylindricalSurface>;
        pub fn cylinder_to_surface(
            cylinder_handle: &Handle_Geom_CylindricalSurface,
        ) -> UniquePtr<Handle_Geom_Surface>;

        type Geom_BezierSurface;
        pub fn Geom_BezierSurface_new(
            poles: &TColgp_Array2OfPnt,
        ) -> UniquePtr<Handle_Geom_BezierSurface>;
        pub fn bezier_to_surface(
            bezier_handle: &Handle_Geom_BezierSurface,
        ) -> UniquePtr<Handle_Geom_Surface>;

        type Geom_BezierCurve;
        #[cxx_name = "construct_unique"]
        pub fn Geom_BezierCurve_new_points(
            poles: &TColgp_HArray1OfPnt,
        ) -> UniquePtr<Geom_BezierCurve>;

        pub fn Geom_BezierCurve_to_handle(
            law: UniquePtr<Geom_BezierCurve>,
        ) -> UniquePtr<Handle_Geom_BezierCurve>;

        pub fn handle_geom_plane_location(plane: &Handle_Geom_Plane) -> &gp_Pnt;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_Plane>;

        pub fn new_HandleGeomCylindricalSurface_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_CylindricalSurface>;
        pub fn HandleGeomCylindricalSurface_Radius(cyl: &Handle_Geom_CylindricalSurface) -> f64;
        pub fn HandleGeomCylindricalSurface_Position(
            cyl: &Handle_Geom_CylindricalSurface,
        ) -> &gp_Ax3;

        pub fn new_HandleGeomSphericalSurface_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_SphericalSurface>;
        pub fn HandleGeomSphericalSurface_Radius(sphere: &Handle_Geom_SphericalSurface) -> f64;
        pub fn HandleGeomSphericalSurface_Position(
            sphere: &Handle_Geom_SphericalSurface,
        ) -> &gp_Ax3;

        pub fn new_HandleGeomConicalSurface_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_ConicalSurface>;
        pub fn HandleGeomConicalSurface_RefRadius(cone: &Handle_Geom_ConicalSurface) -> f64;
        pub fn HandleGeomConicalSurface_SemiAngle(cone: &Handle_Geom_ConicalSurface) -> f64;
        pub fn HandleGeomConicalSurface_Position(cone: &Handle_Geom_ConicalSurface) -> &gp_Ax3;

        pub fn new_HandleGeomToroidalSurface_from_HandleGeomSurface(
            geom_surface_handle: &Handle_Geom_Surface,
        ) -> UniquePtr<Handle_Geom_ToroidalSurface>;
        pub fn HandleGeomToroidalSurface_MajorRadius(torus: &Handle_Geom_ToroidalSurface) -> f64;
        pub fn HandleGeomToroidalSurface_MinorRadius(torus: &Handle_Geom_ToroidalSurface) -> f64;
        pub fn HandleGeomToroidalSurface_Position(torus: &Handle_Geom_ToroidalSurface) -> &gp_Ax3;

        pub fn new_HandleGeomCircle_from_HandleGeomCurve(
            curve: &Handle_Geom_Curve,
        ) -> UniquePtr<Handle_Geom_Circle>;
        pub fn HandleGeomCircle_Circ(circle: &Handle_Geom_Circle) -> UniquePtr<gp_Circ>;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeom_BSplineCurve(
            bspline_curve_handle: &Handle_Geom_BSplineCurve,
        ) -> UniquePtr<Handle_Geom_Curve>;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeom_BezierCurve(
            bezier_curve_handle: &Handle_Geom_BezierCurve,
        ) -> UniquePtr<Handle_Geom_Curve>;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            trimmed_curve_handle: &Handle_Geom_TrimmedCurve,
        ) -> UniquePtr<Handle_Geom_Curve>;
    }

    impl UniquePtr<Handle_Geom_Circle> {}
    impl UniquePtr<Handle_Geom_CylindricalSurface> {}
    impl UniquePtr<Handle_Geom_SphericalSurface> {}
    impl UniquePtr<Handle_Geom_ConicalSurface> {}
    impl UniquePtr<Handle_Geom_ToroidalSurface> {}
    impl UniquePtr<Handle_Geom_BezierSurface> {}
    impl UniquePtr<Handle_Geom_BezierCurve> {}
    impl UniquePtr<Handle_Geom_Plane> {}
    impl UniquePtr<Handle_Geom_BSplineCurve> {}
    impl UniquePtr<Handle_Geom_TrimmedCurve> {}
    impl UniquePtr<Handle_Geom_Surface> {}
    impl UniquePtr<Handle_Geom_Curve> {}
}
