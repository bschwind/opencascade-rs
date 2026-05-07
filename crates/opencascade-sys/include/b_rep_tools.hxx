#include <BRepTools.hxx>
#include <BRep_Builder.hxx>
#include <TopoDS_Wire.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<TopoDS_Wire> outer_wire(const TopoDS_Face &face) {
  return std::unique_ptr<TopoDS_Wire>(new TopoDS_Wire(BRepTools::OuterWire(face)));
}

inline bool write_brep_text(const TopoDS_Shape &shape, rust::String path) {
  return BRepTools::Write(shape, path.c_str());
}

inline std::unique_ptr<TopoDS_Shape> read_brep_text(rust::String path) {
  BRep_Builder builder;
  auto shape = std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape());

  if (BRepTools::Read(*shape, path.c_str(), builder)) {
    return shape;
  } else {
    return std::unique_ptr<TopoDS_Shape>(nullptr);
  }
}
