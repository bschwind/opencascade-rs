use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type gp_Pnt = crate::ffi::gp_Pnt;
        type gp_Pnt2d = crate::ffi::gp_Pnt2d;
        type HandlePoly_Triangulation = crate::ffi::HandlePoly_Triangulation;
        type gp_Dir = crate::ffi::gp_Dir;
        type BRepAdaptor_Curve = crate::ffi::BRepAdaptor_Curve;

        #[cxx_name = "Poly_Triangulation"]
        type Triangulation;
        #[cxx_name = "construct_unique"]
        fn Triangulation_new(
            nb_nodes: i32,
            nb_triangles: i32,
            has_uv: bool,
            has_normals: bool,
        ) -> UniquePtr<Triangulation>;
        fn NbNodes(self: &Triangulation) -> i32;
        fn NbTriangles(self: &Triangulation) -> i32;
        fn HasNormals(self: &Triangulation) -> bool;
        fn HasUVNodes(self: &Triangulation) -> bool;
        fn Triangle(self: &Triangulation, index: i32) -> &Triangle;
        fn SetTriangle(self: Pin<&mut Triangulation>, index: i32, triangle: &Triangle);
        fn SetNode(self: Pin<&mut Triangulation>, index: i32, node: &gp_Pnt);
        fn SetNormal(self: Pin<&mut Triangulation>, index: i32, dir: &gp_Dir);
        fn SetUVNode(self: Pin<&mut Triangulation>, index: i32, uv: &gp_Pnt2d);
        fn Poly_Triangulation_Normal(
            triangulation: &Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Dir>;
        fn Poly_Triangulation_Node(triangulation: &Triangulation, index: i32) -> UniquePtr<gp_Pnt>;
        fn Poly_Triangulation_UV(triangulation: &Triangulation, index: i32) -> UniquePtr<gp_Pnt2d>;

        #[cxx_name = "Poly_Triangle"]
        type Triangle;
        #[cxx_name = "construct_unique"]
        fn Triangle_new(node1: i32, node2: i32, node3: i32) -> UniquePtr<Triangle>;
        fn Value(self: &Triangle, index: i32) -> i32;

        #[cxx_name = "Poly_Connect"]
        type Connect;
        #[cxx_name = "construct_unique"]
        fn Connect_new(triangulation: &HandlePoly_Triangulation) -> UniquePtr<Connect>;
    }
}

impl Triangulation {
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
