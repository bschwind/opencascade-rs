#include <BRepLib.hxx>
#include <BRepLib_ToolTriangulatedShape.hxx>
#include <TopoDS_Shape.hxx>
#include <bindings_common.hxx>

// MSVC only: `Handle_Poly_Triangulation` is a distinct subclass of
// `opencascade::handle<Poly_Triangulation>` there (see
// Standard_Handle.hxx), so a function pointer to
// `BRepLib_ToolTriangulatedShape::ComputeNormals` (declared with
// `Handle(Poly_Triangulation)`, i.e. the base type) doesn't exact-match a
// `Handle_Poly_Triangulation` parameter. An inline wrapper sidesteps the
// function-pointer binding entirely; the implicit conversion at the call
// site is fine either way.
inline void BRepLib_ToolTriangulatedShape_ComputeNormals(const TopoDS_Face &face,
                                                          const Handle_Poly_Triangulation &triangulation) {
  BRepLib_ToolTriangulatedShape::ComputeNormals(face, triangulation);
}
