pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/g_prop.hxx");

        type gp_Ax1 = crate::gp::gp_Ax1;
        type gp_Pnt = crate::gp::gp_Pnt;

        type GProp_GProps;
        #[cxx_name = "construct_unique"]
        fn GProps_new() -> UniquePtr<GProp_GProps>;
        fn Mass(self: &GProp_GProps) -> f64;
        fn StaticMoments(self: &GProp_GProps, lx: &mut f64, ly: &mut f64, lz: &mut f64);
        fn MomentOfInertia(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
        fn RadiusOfGyration(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
        fn GProp_GProps_CentreOfMass(props: &GProp_GProps) -> UniquePtr<gp_Pnt>;
    }
}
