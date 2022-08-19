#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280
        include!("opencascade-sys/OCCT/src/gp/gp_Pnt.hxx");
        include!("opencascade-sys/include/wrapper.hxx");

        type gp_Pnt;
        pub fn make_gp_Pnt() -> UniquePtr<gp_Pnt>;
        pub fn Y(&self) -> f64;
    }
}
