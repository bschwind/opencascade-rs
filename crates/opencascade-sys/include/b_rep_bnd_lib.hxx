#include <bindings_common.hxx>
#include <BRepBndLib.hxx>

inline void BRepBndLib_Add(const TopoDS_Shape &shape, Bnd_Box &box, const Standard_Boolean useTriangulation) {
  BRepBndLib::Add(shape, box, useTriangulation);
}
