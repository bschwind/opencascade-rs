use std::pin::Pin;

pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_g_prop.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type gp_Pnt = crate::ffi::gp_Pnt;
        type gp_Vec = crate::ffi::gp_Vec;
        type GProp_GProps = crate::g_prop::GProp_GProps;

        fn BRepGProp_LinearProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);
        fn BRepGProp_SurfaceProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);
        fn BRepGProp_VolumeProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);

        type BRepGProp;
        type BRepGProp_Face;

        #[cxx_name = "construct_unique"]
        fn BRepGProp_Face_new(face: &TopoDS_Face) -> UniquePtr<BRepGProp_Face>;
        fn Normal(
            self: &BRepGProp_Face,
            u: f64,
            v: f64,
            point: Pin<&mut gp_Pnt>,
            normal: Pin<&mut gp_Vec>,
        );
    }
}

impl BRepGProp {
    pub fn linear_properties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>) {
        inner::BRepGProp_LinearProperties(shape, props)
    }

    pub fn surface_properties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>) {
        inner::BRepGProp_SurfaceProperties(shape, props)
    }

    pub fn volume_properties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>) {
        inner::BRepGProp_VolumeProperties(shape, props)
    }
}

impl BRepGProp_Face {
    pub fn new(face: &TopoDS_Face) -> cxx::UniquePtr<BRepGProp_Face> {
        BRepGProp_Face_new(face)
    }

    pub fn normal(&self, u: f64, v: f64, point: Pin<&mut gp_Pnt>, normal: Pin<&mut gp_Vec>) {
        self.Normal(u, v, point, normal);
    }
}
