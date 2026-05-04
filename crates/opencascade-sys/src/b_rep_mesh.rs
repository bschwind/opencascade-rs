use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/b_rep_mesh.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;

        type BRepMesh_IncrementalMesh;

        #[cxx_name = "construct_unique"]
        fn IncrementalMesh_new(
            shape: &TopoDS_Shape,
            deflection: f64,
        ) -> UniquePtr<BRepMesh_IncrementalMesh>;

        fn Shape(self: &BRepMesh_IncrementalMesh) -> &TopoDS_Shape;
        fn IsDone(self: &BRepMesh_IncrementalMesh) -> bool;
    }
}

impl BRepMesh_IncrementalMesh {
    pub fn new(shape: &TopoDS_Shape, deflection: f64) -> UniquePtr<Self> {
        IncrementalMesh_new(shape, deflection)
    }

    pub fn shape(&self) -> &TopoDS_Shape {
        self.Shape()
    }

    pub fn is_done(&self) -> bool {
        self.IsDone()
    }
}
