#include <BinTools.hxx>
#include <TopoDS_Shape.hxx>
#include <bindings_common.hxx>

inline bool write_brep_bin(const TopoDS_Shape &shape, rust::String path) {
  return BinTools::Write(shape, path.c_str());
}

inline std::unique_ptr<TopoDS_Shape> read_brep_bin(rust::String path) {
  auto shape = std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape());

  if (BinTools::Read(*shape, path.c_str())) {
    return shape;
  } else {
    return std::unique_ptr<TopoDS_Shape>(nullptr);
  }
}
