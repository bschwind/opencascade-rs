use std::pin::Pin;

pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;
        type HandleTopTools_HSequenceOfShape = crate::ffi::HandleTopTools_HSequenceOfShape;

        #[cxx_name = "ShapeAnalysis_FreeBounds"]
        type FreeBounds;

        fn connect_edges_to_wires(
            edges: Pin<&mut HandleTopTools_HSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut HandleTopTools_HSequenceOfShape>,
        );
    }
}

impl FreeBounds {
    pub fn connect_edges_to_wires(
        edges: Pin<&mut HandleTopTools_HSequenceOfShape>,
        tolerance: f64,
        shared: bool,
        wires: Pin<&mut HandleTopTools_HSequenceOfShape>,
    ) {
        connect_edges_to_wires(edges, tolerance, shared, wires);
    }
}
