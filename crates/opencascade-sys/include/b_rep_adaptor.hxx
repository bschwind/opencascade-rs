#include <BRepAdaptor_Curve.hxx>
#include <bindings_common.hxx>
#include <gp_Pnt.hxx>

inline std::unique_ptr<gp_Pnt> BRepAdaptor_Curve_value(const BRepAdaptor_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve.Value(U)));
}
