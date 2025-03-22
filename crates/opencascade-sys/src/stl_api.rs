use cxx::UniquePtr;
pub use inner::*;
use std::pin::Pin;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/wrapper.hxx");

        type TopoDS_Shape = crate::ffi::TopoDS_Shape;

        #[cxx_name = "StlAPI_Writer"]
        type Writer;

        #[cxx_name = "construct_unique"]
        fn Writer_new() -> UniquePtr<Writer>;

        // fn Write(self: Pin<&mut Writer>, shape: &TopoDS_Shape, filename: &c_char) -> bool;
        fn write_stl(writer: Pin<&mut Writer>, shape: &TopoDS_Shape, filename: String) -> bool;
    }
}

impl Writer {
    pub fn new() -> UniquePtr<Self> {
        Writer_new()
    }

    pub fn write_stl(self: Pin<&mut Writer>, shape: &TopoDS_Shape, filename: String) -> bool {
        write_stl(self, shape, filename)
    }
}
