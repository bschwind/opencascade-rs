pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/shape_upgrade.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;

        #[cxx_name = "ShapeUpgrade_UnifySameDomain"]
        type UnifySameDomain;

        #[cxx_name = "construct_unique"]
        pub fn UnifySameDomain_new(
            shape: &TopoDS_Shape,
            unify_edges: bool,
            unify_faces: bool,
            concat_b_splines: bool,
        ) -> UniquePtr<UnifySameDomain>;

        #[cxx_name = "AllowInternalEdges"]
        pub fn allow_internal_edges(self: Pin<&mut UnifySameDomain>, allow: bool);

        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut UnifySameDomain>);

        #[cxx_name = "Shape"]
        pub fn shape(self: &UnifySameDomain) -> &TopoDS_Shape;
    }
}

impl UnifySameDomain {
    pub fn new(
        shape: &TopoDS_Shape,
        unify_edges: bool,
        unify_faces: bool,
        concat_b_splines: bool,
    ) -> cxx::UniquePtr<Self> {
        UnifySameDomain_new(shape, unify_edges, unify_faces, concat_b_splines)
    }
}
