#include <ShapeAnalysis.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <bindings_common.hxx>

inline void connect_edges_to_wires(Handle_TopTools_HSequenceOfShape &edges, const Standard_Real toler,
                                   const Standard_Boolean shared, Handle_TopTools_HSequenceOfShape &wires) {
  ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges, toler, shared, wires);
}
