#include <bindings_common.hxx>
#include <GCPnts_TangentialDeflection.hxx>
#include <BRepAdaptor_Curve.hxx>

inline std::unique_ptr<gp_Pnt> GCPnts_TangentialDeflection_Value(const GCPnts_TangentialDeflection &approximator,
                                                                 Standard_Integer i) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(approximator.Value(i)));
}
