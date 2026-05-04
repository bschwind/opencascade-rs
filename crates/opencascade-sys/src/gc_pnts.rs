use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/gc_pnts.hxx");

        type gp_Pnt = crate::ffi::gp_Pnt;
        type BRepAdaptor_Curve = crate::ffi::BRepAdaptor_Curve;

        type GCPnts_TangentialDeflection;

        #[cxx_name = "construct_unique"]
        fn TangentialDeflection_new(
            curve: &BRepAdaptor_Curve,
            angular_deflection: f64,
            curvature_deflection: f64,
        ) -> UniquePtr<GCPnts_TangentialDeflection>;
        fn NbPoints(self: &GCPnts_TangentialDeflection) -> i32;
        fn GCPnts_TangentialDeflection_Value(
            approximator: &GCPnts_TangentialDeflection,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;
    }
}

impl GCPnts_TangentialDeflection {
    pub fn new(
        curve: &BRepAdaptor_Curve,
        angular_deflection: f64,
        curvature_deflection: f64,
    ) -> cxx::UniquePtr<Self> {
        TangentialDeflection_new(curve, angular_deflection, curvature_deflection)
    }

    pub fn num_points(&self) -> i32 {
        self.NbPoints()
    }

    pub fn value(&self, index: i32) -> UniquePtr<gp_Pnt> {
        GCPnts_TangentialDeflection_Value(self, index)
    }
}
