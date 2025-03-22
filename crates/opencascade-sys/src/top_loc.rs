use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type gp_Trsf = crate::ffi::gp_Trsf;

        #[cxx_name = "TopLoc_Location"]
        type Location;
        #[cxx_name = "construct_unique"]
        fn Location_new() -> UniquePtr<Location>;

        #[cxx_name = "construct_unique"]
        fn Location_from_transform(transform: &gp_Trsf) -> UniquePtr<Location>;

        fn TopLoc_Location_Transformation(location: &Location) -> UniquePtr<gp_Trsf>;
    }
}

impl Location {
    pub fn new() -> UniquePtr<Self> {
        Location_new()
    }

    pub fn from_transform(transform: &gp_Trsf) -> UniquePtr<Location> {
        Location_from_transform(transform)
    }

    pub fn transform(&self) -> UniquePtr<gp_Trsf> {
        TopLoc_Location_Transformation(self)
    }
}
