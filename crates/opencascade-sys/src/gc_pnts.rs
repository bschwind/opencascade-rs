use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type gp_Pnt = crate::ffi::gp_Pnt;
        type BRepAdaptor_Curve = crate::ffi::BRepAdaptor_Curve;

        #[cxx_name = "GCPnts_TangentialDeflection"]
        type TangentialDeflection;

        #[cxx_name = "construct_unique"]
        fn TangentialDeflection_new(
            curve: &BRepAdaptor_Curve,
            angular_deflection: f64,
            curvature_deflection: f64,
        ) -> UniquePtr<TangentialDeflection>;
        fn NbPoints(self: &TangentialDeflection) -> i32;
        fn GCPnts_TangentialDeflection_Value(
            approximator: &TangentialDeflection,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;
    }
}

impl TangentialDeflection {
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
