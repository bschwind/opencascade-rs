pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/bin_tools.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;

        #[cxx_name = "write_brep_bin"]
        pub fn write(shape: &TopoDS_Shape, path: String) -> bool;
        #[cxx_name = "read_brep_bin"]
        pub fn read(path: String) -> UniquePtr<TopoDS_Shape>;
    }
}
