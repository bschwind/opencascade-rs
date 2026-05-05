#include <Standard_Type.hxx>
#include <bindings_common.hxx>

inline rust::String type_name(const Handle_Standard_Type &handle) { return std::string(handle->Name()); }
