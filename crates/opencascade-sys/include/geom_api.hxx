#include <GeomAPI_Interpolate.hxx>
#include <GeomAPI_ProjectPointOnSurf.hxx>
#include <Geom_BSplineCurve.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<Handle_Geom_BSplineCurve> GeomAPI_Interpolate_Curve(const GeomAPI_Interpolate &interpolate) {
  return std::unique_ptr<Handle_Geom_BSplineCurve>(new opencascade::handle<Geom_BSplineCurve>(interpolate.Curve()));
}
