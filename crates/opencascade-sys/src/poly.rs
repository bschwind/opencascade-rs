pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/poly.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Pnt2d = crate::gp::gp_Pnt2d;
        type gp_Dir = crate::gp::gp_Dir;
        type BRepAdaptor_Curve = crate::b_rep_adaptor::BRepAdaptor_Curve;

        // Handles
        type Handle_Poly_Triangulation;
        pub fn IsNull(self: &Handle_Poly_Triangulation) -> bool;
        #[cxx_name = "handle_try_deref"]
        pub fn Handle_Poly_Triangulation_Get(
            handle: &Handle_Poly_Triangulation,
        ) -> Result<&Poly_Triangulation>;
        // End Handles

        type Poly_Triangulation;
        #[cxx_name = "construct_unique"]
        fn Triangulation_new(
            nb_nodes: i32,
            nb_triangles: i32,
            has_uv: bool,
            has_normals: bool,
        ) -> UniquePtr<Poly_Triangulation>;
        fn NbNodes(self: &Poly_Triangulation) -> i32;
        fn NbTriangles(self: &Poly_Triangulation) -> i32;
        fn HasNormals(self: &Poly_Triangulation) -> bool;
        fn HasUVNodes(self: &Poly_Triangulation) -> bool;
        fn Triangle(self: &Poly_Triangulation, index: i32) -> &Poly_Triangle;
        fn SetTriangle(self: Pin<&mut Poly_Triangulation>, index: i32, triangle: &Poly_Triangle);
        fn SetNode(self: Pin<&mut Poly_Triangulation>, index: i32, node: &gp_Pnt);
        fn SetNormal(self: Pin<&mut Poly_Triangulation>, index: i32, dir: &gp_Dir);
        fn SetUVNode(self: Pin<&mut Poly_Triangulation>, index: i32, uv: &gp_Pnt2d);
        fn Poly_Triangulation_Normal(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Dir>;
        fn Poly_Triangulation_Node(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;
        fn Poly_Triangulation_UV(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Pnt2d>;

        type Poly_Triangle;
        #[cxx_name = "construct_unique"]
        fn Triangle_new(node1: i32, node2: i32, node3: i32) -> UniquePtr<Poly_Triangle>;
        fn Value(self: &Poly_Triangle, index: i32) -> i32;

        type Poly_Connect;
        #[cxx_name = "construct_unique"]
        fn Connect_new(triangulation: &Handle_Poly_Triangulation) -> UniquePtr<Poly_Connect>;

        pub fn Handle_Poly_Triangulation_new(
            triangulation: UniquePtr<Poly_Triangulation>,
        ) -> UniquePtr<Handle_Poly_Triangulation>;
    }

    impl UniquePtr<Handle_Poly_Triangulation> {}
}
