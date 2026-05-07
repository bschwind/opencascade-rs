pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/stl_api.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        type StlAPI_Writer;
        #[cxx_name = "construct_unique"]
        fn StlAPI_Writer_new() -> UniquePtr<StlAPI_Writer>;
        fn write_stl(
            writer: Pin<&mut StlAPI_Writer>,
            shape: &TopoDS_Shape,
            filename: String,
        ) -> bool;
    }
}
