pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/bnd.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;

        // Describes a bounding box in 3D space.
        type Bnd_Box;
        #[cxx_name = "construct_unique"]
        fn Bnd_Box_new() -> UniquePtr<Bnd_Box>;
        fn IsVoid(self: &Bnd_Box) -> bool;
        fn Get(
            self: &Bnd_Box,
            xMin: &mut f64,
            yMin: &mut f64,
            zMin: &mut f64,
            xMax: &mut f64,
            yMax: &mut f64,
            zMax: &mut f64,
        );
        fn Bnd_Box_CornerMin(b: &Bnd_Box) -> UniquePtr<gp_Pnt>;
        fn Bnd_Box_CornerMax(b: &Bnd_Box) -> UniquePtr<gp_Pnt>;
        fn GetGap(self: &Bnd_Box) -> f64;
        fn Set(self: Pin<&mut Bnd_Box>, p: &gp_Pnt);
        fn SetGap(self: Pin<&mut Bnd_Box>, gap: f64);
    }
}
