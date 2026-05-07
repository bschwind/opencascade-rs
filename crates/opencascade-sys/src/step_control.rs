pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/step_control.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type IFSelect_ReturnStatus = crate::if_select::IFSelect_ReturnStatus;
        type Message_ProgressRange = crate::message::Message_ProgressRange;

        type STEPControl_Reader;
        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Reader_new() -> UniquePtr<STEPControl_Reader>;
        pub fn read_step(
            reader: Pin<&mut STEPControl_Reader>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
        pub fn TransferRoots(
            self: Pin<&mut STEPControl_Reader>,
            progress: &Message_ProgressRange,
        ) -> i32;
        pub fn one_shape_step(reader: &STEPControl_Reader) -> UniquePtr<TopoDS_Shape>;

        type STEPControl_Writer;
        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Writer_new() -> UniquePtr<STEPControl_Writer>;
        // TODO(bschwind) - Expose the full list of parameters here.
        pub fn transfer_shape(
            writer: Pin<&mut STEPControl_Writer>,
            shape: &TopoDS_Shape,
        ) -> IFSelect_ReturnStatus;
        pub fn write_step(
            writer: Pin<&mut STEPControl_Writer>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
    }
}
