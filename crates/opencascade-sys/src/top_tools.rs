pub use inner::*;

#[cxx::bridge]
mod inner {
    unsafe extern "C++" {
        include!("opencascade-sys/include/top_tools.hxx");

        type TopAbs_ShapeEnum = crate::top_abs::TopAbs_ShapeEnum;
        type TopoDS_Shape = crate::topo_ds::TopoDS_Shape;
        type TopoDS_Face = crate::topo_ds::TopoDS_Face;

        // Handles
        type Handle_TopTools_HSequenceOfShape;
        pub fn IsNull(self: &Handle_TopTools_HSequenceOfShape) -> bool;
        // End Handles

        type TopTools_ListOfShape;
        #[cxx_name = "construct_unique"]
        pub fn new_list_of_shape() -> UniquePtr<TopTools_ListOfShape>;
        pub fn Append(
            self: Pin<&mut TopTools_ListOfShape>,
            face: &TopoDS_Shape,
        ) -> Pin<&mut TopoDS_Shape>;
        pub fn Size(self: &TopTools_ListOfShape) -> i32;

        type TopTools_IndexedMapOfShape;
        #[cxx_name = "construct_unique"]
        pub fn new_indexed_map_of_shape() -> UniquePtr<TopTools_IndexedMapOfShape>;
        pub fn Extent(self: &TopTools_IndexedMapOfShape) -> i32;
        pub fn FindKey(self: &TopTools_IndexedMapOfShape, index: i32) -> &TopoDS_Shape;

        type TopTools_IndexedDataMapOfShapeListOfShape;
        #[cxx_name = "construct_unique"]
        pub fn new_indexed_data_map_of_shape_list_of_shape(
        ) -> UniquePtr<TopTools_IndexedDataMapOfShapeListOfShape>;
        pub fn Extent(self: &TopTools_IndexedDataMapOfShapeListOfShape) -> i32;
        pub fn FindKey(
            self: &TopTools_IndexedDataMapOfShapeListOfShape,
            index: i32,
        ) -> &TopoDS_Shape;
        pub fn FindFromIndex(
            self: &TopTools_IndexedDataMapOfShapeListOfShape,
            index: i32,
        ) -> &TopTools_ListOfShape;
        pub fn FindIndex(
            self: &TopTools_IndexedDataMapOfShapeListOfShape,
            shape: &TopoDS_Shape,
        ) -> i32;
        pub fn FindFromKey<'a>(
            self: &'a TopTools_IndexedDataMapOfShapeListOfShape,
            shape: &'a TopoDS_Shape,
        ) -> &'a TopTools_ListOfShape;

        type TopTools_HSequenceOfShape;
        pub fn Length(self: &TopTools_HSequenceOfShape) -> i32;

        pub fn new_Handle_TopTools_HSequenceOfShape() -> UniquePtr<Handle_TopTools_HSequenceOfShape>;
        pub fn TopTools_HSequenceOfShape_append(
            handle: Pin<&mut Handle_TopTools_HSequenceOfShape>,
            shape: &TopoDS_Shape,
        );

        pub fn TopTools_HSequenceOfShape_length(handle: &Handle_TopTools_HSequenceOfShape) -> i32;
        pub fn TopTools_HSequenceOfShape_value(
            handle: &Handle_TopTools_HSequenceOfShape,
            index: i32,
        ) -> &TopoDS_Shape;

        #[cxx_name = "handle_try_deref"]
        pub fn HandleTopTools_HSequenceOfShape_Get(
            handle: &Handle_TopTools_HSequenceOfShape,
        ) -> Result<&TopTools_HSequenceOfShape>;
    }

    impl UniquePtr<Handle_TopTools_HSequenceOfShape> {}
}
