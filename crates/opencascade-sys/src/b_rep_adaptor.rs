pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_adaptor.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Vec = crate::gp::gp_Vec;
        type gp_Circ = crate::gp::gp_Circ;
        type GeomAbs_CurveType = crate::geom_abs::GeomAbs_CurveType;
        type TopoDS_Edge = crate::topo_ds::TopoDS_Edge;

        type BRepAdaptor_Curve;
        #[cxx_name = "construct_unique"]
        pub fn BRepAdaptor_Curve_new(edge: &TopoDS_Edge) -> UniquePtr<BRepAdaptor_Curve>;
        pub fn FirstParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn LastParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn BRepAdaptor_Curve_value(curve: &BRepAdaptor_Curve, u: f64) -> UniquePtr<gp_Pnt>;
        pub fn BRepAdaptor_Curve_D1(
            curve: &BRepAdaptor_Curve,
            u: f64,
            point: Pin<&mut gp_Pnt>,
            vec: Pin<&mut gp_Vec>,
        );
        pub fn BRepAdaptor_Curve_Circle(curve: &BRepAdaptor_Curve) -> UniquePtr<gp_Circ>;
        pub fn GetType(self: &BRepAdaptor_Curve) -> GeomAbs_CurveType;
    }
}
