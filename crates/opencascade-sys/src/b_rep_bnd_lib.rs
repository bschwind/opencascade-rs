pub use inner::*;
use std::pin::Pin;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_bnd_lib.hxx");

        type gp_Pnt = crate::ffi::gp_Pnt;
        type TopoDS_Shape = crate::ffi::TopoDS_Shape;

        type Bnd_Box = crate::bnd::Bnd_Box;
        type BRepBndLib;

        pub fn BRepBndLib_Add(shape: &TopoDS_Shape, bb: Pin<&mut Bnd_Box>, use_triangulation: bool);
    }
}

pub fn add(shape: &TopoDS_Shape, bb: Pin<&mut Bnd_Box>, use_triangulation: bool) {
    BRepBndLib_Add(shape, bb, use_triangulation)
}
