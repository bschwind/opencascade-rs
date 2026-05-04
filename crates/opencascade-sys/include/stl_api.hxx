#include <StlAPI.hxx>
#include <StlAPI_Writer.hxx>
#include <bindings_common.hxx>

inline bool write_stl(StlAPI_Writer &writer, const TopoDS_Shape &theShape, rust::String theFileName) {
  return writer.Write(theShape, theFileName.c_str());
}
