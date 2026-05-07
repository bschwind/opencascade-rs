pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_bnd_lib.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type Bnd_Box = crate::bnd::Bnd_Box;

        type BRepBndLib;
        #[Self = "BRepBndLib"]
        pub fn Add(shape: &TopoDS_Shape, bb: Pin<&mut Bnd_Box>, use_triangulation: bool);
    }
}
