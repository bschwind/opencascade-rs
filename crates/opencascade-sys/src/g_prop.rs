use cxx::UniquePtr;

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

impl GProp_GProps {
    pub fn new() -> cxx::UniquePtr<Self> {
        GProps_new()
    }

    pub fn mass(&self) -> f64 {
        self.Mass()
    }

    pub fn static_moments(&self, lx: &mut f64, ly: &mut f64, lz: &mut f64) {
        self.StaticMoments(lx, ly, lz)
    }

    pub fn moment_of_inertia(&self, axis: &gp_Ax1) -> f64 {
        self.MomentOfInertia(axis)
    }

    pub fn radius_of_gyration(&self, axis: &gp_Ax1) -> f64 {
        self.RadiusOfGyration(axis)
    }

    pub fn center_of_mass(&self) -> UniquePtr<gp_Pnt> {
        GProp_GProps_CentreOfMass(self)
    }
}
