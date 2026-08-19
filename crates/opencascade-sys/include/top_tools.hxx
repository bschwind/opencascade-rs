#include <TopTools_HSequenceOfShape.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedMapOfShape.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS_Face.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<RustHandle_TopTools_HSequenceOfShape> new_Handle_TopTools_HSequenceOfShape() {
  auto sequence = new TopTools_HSequenceOfShape();
  auto handle = new opencascade::handle<TopTools_HSequenceOfShape>(sequence);

  return std::unique_ptr<RustHandle_TopTools_HSequenceOfShape>(handle);
}

inline void TopTools_HSequenceOfShape_append(RustHandle_TopTools_HSequenceOfShape &handle, const TopoDS_Shape &shape) {
  handle->Append(shape);
}

inline Standard_Integer TopTools_HSequenceOfShape_length(const RustHandle_TopTools_HSequenceOfShape &handle) {
  return handle->Length();
}

inline const TopoDS_Shape &TopTools_HSequenceOfShape_value(const RustHandle_TopTools_HSequenceOfShape &handle,
                                                           Standard_Integer index) {
  return handle->Value(index);
}
