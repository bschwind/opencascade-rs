pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/iges_control.hxx");

        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type IFSelect_ReturnStatus = crate::if_select::IFSelect_ReturnStatus;
        type Message_ProgressRange = crate::message::Message_ProgressRange;

        type IGESControl_Reader;
        #[cxx_name = "construct_unique"]
        pub fn IGESControl_Reader_new() -> UniquePtr<IGESControl_Reader>;
        pub fn read_iges(
            reader: Pin<&mut IGESControl_Reader>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
        pub fn TransferRoots(
            self: Pin<&mut IGESControl_Reader>,
            progress: &Message_ProgressRange,
        ) -> i32;
        pub fn one_shape_iges(reader: &IGESControl_Reader) -> UniquePtr<TopoDS_Shape>;

        type IGESControl_Writer;
        #[cxx_name = "construct_unique"]
        pub fn IGESControl_Writer_new() -> UniquePtr<IGESControl_Writer>;
        pub fn AddShape(
            self: Pin<&mut IGESControl_Writer>,
            shape: &TopoDS_Shape,
            progress: &Message_ProgressRange,
        ) -> bool;
        pub fn ComputeModel(self: Pin<&mut IGESControl_Writer>);
        pub fn write_iges(writer: Pin<&mut IGESControl_Writer>, filename: String) -> bool;
    }
}
