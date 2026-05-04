use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/poly.hxx");

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Pnt2d = crate::gp::gp_Pnt2d;
        type Handle_Poly_Triangulation = crate::ffi::Handle_Poly_Triangulation;
        type gp_Dir = crate::gp::gp_Dir;
        type BRepAdaptor_Curve = crate::b_rep_adaptor::BRepAdaptor_Curve;

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

        pub fn Handle_Poly_Triangulation_ctor(
            triangulation: UniquePtr<Poly_Triangulation>,
        ) -> UniquePtr<Handle_Poly_Triangulation>;
    }
}

impl Poly_Triangulation {
    pub fn new(
        nb_nodes: i32,
        nb_triangles: i32,
        has_uv: bool,
        has_normals: bool,
    ) -> UniquePtr<Self> {
        Triangulation_new(nb_nodes, nb_triangles, has_uv, has_normals)
    }

    pub fn num_nodes(&self) -> i32 {
        self.NbNodes()
    }

    pub fn num_triangles(&self) -> i32 {
        self.NbTriangles()
    }

    pub fn node(&self, index: i32) -> UniquePtr<gp_Pnt> {
        Poly_Triangulation_Node(self, index)
    }

    pub fn uv(&self, index: i32) -> UniquePtr<gp_Pnt2d> {
        Poly_Triangulation_UV(self, index)
    }

    pub fn normal(&self, index: i32) -> UniquePtr<gp_Dir> {
        Poly_Triangulation_Normal(self, index)
    }
}
