use std::pin::Pin;

pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;
        type TopoDS_Face = crate::ffi::TopoDS_Face;
        type gp_Ax1 = crate::ffi::gp_Ax1;
        type gp_Pnt = crate::ffi::gp_Pnt;
        type gp_Vec = crate::ffi::gp_Vec;

        #[cxx_name = "GProp_GProps"]
        type GProps = crate::g_prop::GProps;

        fn BRepGProp_LinearProperties(shape: &TopoDS_Shape, props: Pin<&mut GProps>);
        fn BRepGProp_SurfaceProperties(shape: &TopoDS_Shape, props: Pin<&mut GProps>);
        fn BRepGProp_VolumeProperties(shape: &TopoDS_Shape, props: Pin<&mut GProps>);

        type BRepGProp;
        #[cxx_name = "BRepGProp_Face"]
        type Face;

        #[cxx_name = "construct_unique"]
        fn Face_new(face: &TopoDS_Face) -> UniquePtr<Face>;
        fn Normal(self: &Face, u: f64, v: f64, point: Pin<&mut gp_Pnt>, normal: Pin<&mut gp_Vec>);
    }
}

impl BRepGProp {
    pub fn linear_properties(shape: &TopoDS_Shape, props: Pin<&mut GProps>) {
        inner::BRepGProp_LinearProperties(shape, props)
    }

    pub fn surface_properties(shape: &TopoDS_Shape, props: Pin<&mut GProps>) {
        inner::BRepGProp_SurfaceProperties(shape, props)
    }

    pub fn volume_properties(shape: &TopoDS_Shape, props: Pin<&mut GProps>) {
        inner::BRepGProp_VolumeProperties(shape, props)
    }
}

impl Face {
    pub fn new(face: &TopoDS_Face) -> cxx::UniquePtr<Face> {
        Face_new(face)
    }

    pub fn normal(&self, u: f64, v: f64, point: Pin<&mut gp_Pnt>, normal: Pin<&mut gp_Vec>) {
        self.Normal(u, v, point, normal);
    }
}
