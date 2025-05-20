use cxx::UniquePtr;
pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;

        #[cxx_name = "BRepMesh_IncrementalMesh"]
        type IncrementalMesh;

        #[cxx_name = "construct_unique"]
        fn IncrementalMesh_new(shape: &TopoDS_Shape, deflection: f64)
            -> UniquePtr<IncrementalMesh>;

        fn Shape(self: &IncrementalMesh) -> &TopoDS_Shape;
        fn IsDone(self: &IncrementalMesh) -> bool;
    }
}

impl IncrementalMesh {
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
