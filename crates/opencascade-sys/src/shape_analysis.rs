pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/shape_analysis.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;
        type Handle_TopTools_HSequenceOfShape = crate::ffi::Handle_TopTools_HSequenceOfShape;

        #[cxx_name = "ShapeAnalysis_FreeBounds"]
        type FreeBounds;

        pub fn connect_edges_to_wires(
            edges: Pin<&mut Handle_TopTools_HSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut Handle_TopTools_HSequenceOfShape>,
        );
    }
}
