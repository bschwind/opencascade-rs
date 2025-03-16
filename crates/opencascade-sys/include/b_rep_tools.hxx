#include "rust/cxx.h"
#include <memory>
#include <BRepTools.hxx>
#include <TopoDS_Wire.hxx>

inline std::unique_ptr<TopoDS_Wire> outer_wire(const TopoDS_Face &face) {
  return std::unique_ptr<TopoDS_Wire>(new TopoDS_Wire(BRepTools::OuterWire(face)));
}
