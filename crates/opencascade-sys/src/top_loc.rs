use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/top_loc.hxx");

        type gp_Trsf = crate::gp::gp_Trsf;

        type TopLoc_Location;
        #[cxx_name = "construct_unique"]
        fn Location_new() -> UniquePtr<TopLoc_Location>;

        #[cxx_name = "construct_unique"]
        fn Location_from_transform(transform: &gp_Trsf) -> UniquePtr<TopLoc_Location>;

        fn TopLoc_Location_Transformation(location: &TopLoc_Location) -> UniquePtr<gp_Trsf>;
    }
}

impl TopLoc_Location {
    pub fn new() -> UniquePtr<Self> {
        Location_new()
    }

    pub fn from_transform(transform: &gp_Trsf) -> UniquePtr<TopLoc_Location> {
        Location_from_transform(transform)
    }

    pub fn transform(&self) -> UniquePtr<gp_Trsf> {
        TopLoc_Location_Transformation(self)
    }
}
