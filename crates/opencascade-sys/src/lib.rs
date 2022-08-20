#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280
        include!("opencascade-sys/OCCT/src/gp/gp_Pnt.hxx");
        include!("opencascade-sys/OCCT/src/GC/GC_MakeSegment.hxx");
        include!("opencascade-sys/OCCT/src/GC/GC_MakeArcOfCircle.hxx");
        include!("opencascade-sys/include/wrapper.hxx");

        type gp_Pnt;
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;
        pub fn Y(self: &gp_Pnt) -> f64;
        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        type GC_MakeSegment;
        pub fn new_segment(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;

        type GC_MakeArcOfCircle;
        pub fn new_arc_of_circle(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
            p3: &gp_Pnt,
        ) -> UniquePtr<GC_MakeArcOfCircle>;
    }
}
