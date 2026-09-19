#include <ShapeAnalysis.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <bindings_common.hxx>

// MSVC only: same reasoning
// as the other wrappers here - a static-method function pointer doesn't
// exact-match `Handle_TopTools_HSequenceOfShape` against OCCT's own
// `Handle(TopTools_HSequenceOfShape)&`-typed parameters.
inline void ShapeAnalysis_FreeBounds_ConnectEdgesToWires(Handle_TopTools_HSequenceOfShape &edges,
                                                          Standard_Real tolerance, Standard_Boolean shared,
                                                          Handle_TopTools_HSequenceOfShape &wires) {
  ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges, tolerance, shared, wires);
}
