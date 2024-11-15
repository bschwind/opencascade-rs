pub use inner::*;

#[cxx::bridge]
mod inner {

    unsafe extern "C++" {
        #[cxx_name = "IFSelect_ReturnStatus"]
        type IFSelectReturnStatus = crate::ffi::IFSelectReturnStatus;
        #[cxx_name = "TopoDS_Shape"]
        type TopoDSShape = crate::ffi::TopoDSShape;

        include!("opencascade-sys/include/wrapper.hxx");

        // Data Export
        #[cxx_name = "STEPControl_Writer"]
        type STEPControlWriter;

        #[cxx_name = "construct_unique"]
        pub fn STEPControlWriter_ctor() -> UniquePtr<STEPControlWriter>;

        pub fn transfer_shape(
            writer: Pin<&mut STEPControlWriter>,
            shape: &TopoDSShape,
        ) -> IFSelectReturnStatus;
        pub fn write_step(
            writer: Pin<&mut STEPControlWriter>,
            filename: String,
        ) -> IFSelectReturnStatus;

        #[cxx_name = "StlAPI_Writer"]
        type StlAPIWriter;

        #[cxx_name = "construct_unique"]
        pub fn StlAPIWriter_ctor() -> UniquePtr<StlAPIWriter>;

        #[cxx_name = "WriteStl"]
        pub fn write_stl(
            writer: Pin<&mut StlAPIWriter>,
            shape: &TopoDSShape,
            filename: String,
        ) -> bool;
    }
}
