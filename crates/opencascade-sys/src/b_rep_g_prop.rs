pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_g_prop.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;
        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Vec = crate::gp::gp_Vec;
        type GProp_GProps = crate::g_prop::GProp_GProps;

        type BRepGProp;
        #[Self = "BRepGProp"]
        fn LinearProperties(
            shape: &TopoDS_Shape,
            props: Pin<&mut GProp_GProps>,
            skip_shared: bool,
            use_triangulation: bool,
        );
        #[Self = "BRepGProp"]
        fn SurfaceProperties(
            shape: &TopoDS_Shape,
            props: Pin<&mut GProp_GProps>,
            skip_shared: bool,
            use_triangulation: bool,
        );
        #[Self = "BRepGProp"]
        fn VolumeProperties(
            shape: &TopoDS_Shape,
            props: Pin<&mut GProp_GProps>,
            only_closed: bool,
            skip_shared: bool,
            use_triangulation: bool,
        );

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
