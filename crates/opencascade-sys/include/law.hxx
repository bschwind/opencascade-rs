#include <Law_Function.hxx>
#include <Law_Interpol.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<Handle_Law_Function> Law_Function_to_handle(std::unique_ptr<Law_Function> law_function) {
  return std::unique_ptr<Handle_Law_Function>(new Handle_Law_Function(law_function.release()));
}

inline std::unique_ptr<Law_Function> Law_Interpol_into_Law_Function(std::unique_ptr<Law_Interpol> law_interpol) {
  return std::unique_ptr<Law_Function>(law_interpol.release());
}
