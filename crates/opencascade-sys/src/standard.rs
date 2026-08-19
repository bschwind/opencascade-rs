pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/standard.hxx");

        #[cxx_name = "RustHandle_Standard_Type"]
        type Handle_Standard_Type;
        pub fn type_name(handle: &Handle_Standard_Type) -> String;
        pub fn IsNull(self: &Handle_Standard_Type) -> bool;
    }
}
