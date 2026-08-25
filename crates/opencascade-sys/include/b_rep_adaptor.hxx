#include <BRepAdaptor_Curve.hxx>
#include <bindings_common.hxx>
#include <gp_Circ.hxx>
#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>

inline std::unique_ptr<gp_Pnt> BRepAdaptor_Curve_value(const BRepAdaptor_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve.Value(U)));
}

inline void BRepAdaptor_Curve_D1(const BRepAdaptor_Curve &curve, const Standard_Real U, gp_Pnt &P, gp_Vec &V1) {
  curve.D1(U, P, V1);
}

inline std::unique_ptr<gp_Circ> BRepAdaptor_Curve_Circle(const BRepAdaptor_Curve &curve) {
  return std::unique_ptr<gp_Circ>(new gp_Circ(curve.Circle()));
}
