#include <TopTools_HSequenceOfShape.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedMapOfShape.hxx>
#include <TopTools_ListOfShape.hxx>
#include <TopoDS_Face.hxx>
#include <bindings_common.hxx>

// MSVC only: same reasoning
// as Handle_Poly_Triangulation_deref in poly.hxx.
inline const TopTools_HSequenceOfShape &
Handle_TopTools_HSequenceOfShape_deref(const Handle_TopTools_HSequenceOfShape &handle) {
  return handle_try_deref<TopTools_HSequenceOfShape>(handle);
}

inline std::unique_ptr<Handle_TopTools_HSequenceOfShape> new_Handle_TopTools_HSequenceOfShape() {
  auto sequence = new TopTools_HSequenceOfShape();
  auto handle = new Handle_TopTools_HSequenceOfShape(sequence);

  return std::unique_ptr<Handle_TopTools_HSequenceOfShape>(handle);
}

inline void TopTools_HSequenceOfShape_append(Handle_TopTools_HSequenceOfShape &handle, const TopoDS_Shape &shape) {
  handle->Append(shape);
}

inline Standard_Integer TopTools_HSequenceOfShape_length(const Handle_TopTools_HSequenceOfShape &handle) {
  return handle->Length();
}

inline const TopoDS_Shape &TopTools_HSequenceOfShape_value(const Handle_TopTools_HSequenceOfShape &handle,
                                                           Standard_Integer index) {
  return handle->Value(index);
}
