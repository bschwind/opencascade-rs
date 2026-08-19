#include <Geom_BSplineCurve.hxx>
#include <Geom_BezierCurve.hxx>
#include <Geom_BezierSurface.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom_Plane.hxx>
#include <Geom_Surface.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<RustHandle_Geom_CylindricalSurface> Geom_CylindricalSurface_new(const gp_Ax3 &axis,
                                                                                       double radius) {
  return std::unique_ptr<RustHandle_Geom_CylindricalSurface>(
      new opencascade::handle<Geom_CylindricalSurface>(new Geom_CylindricalSurface(axis, radius)));
}

inline std::unique_ptr<RustHandle_Geom_Surface>
cylinder_to_surface(const RustHandle_Geom_CylindricalSurface &cylinder_handle) {
  return std::unique_ptr<RustHandle_Geom_Surface>(new opencascade::handle<Geom_Surface>(cylinder_handle));
}

inline std::unique_ptr<RustHandle_Geom_BezierSurface> Geom_BezierSurface_new(const TColgp_Array2OfPnt &poles) {
  return std::unique_ptr<RustHandle_Geom_BezierSurface>(
      new opencascade::handle<Geom_BezierSurface>(new Geom_BezierSurface(poles)));
}

inline const gp_Pnt &handle_geom_plane_location(const RustHandle_Geom_Plane &plane) { return plane->Location(); }

inline std::unique_ptr<RustHandle_Geom_BezierCurve>
Geom_BezierCurve_to_handle(std::unique_ptr<Geom_BezierCurve> bezier_curve) {
  return std::unique_ptr<RustHandle_Geom_BezierCurve>(new RustHandle_Geom_BezierCurve(bezier_curve.release()));
}

inline std::unique_ptr<RustHandle_Geom_Surface> bezier_to_surface(const RustHandle_Geom_BezierSurface &bezier_handle) {
  return std::unique_ptr<RustHandle_Geom_Surface>(new opencascade::handle<Geom_Surface>(bezier_handle));
}

inline std::unique_ptr<RustHandle_Geom_Plane>
new_HandleGeomPlane_from_HandleGeomSurface(const RustHandle_Geom_Surface &surface) {
  RustHandle_Geom_Plane plane_handle = opencascade::handle<Geom_Plane>::DownCast(surface);
  return std::unique_ptr<RustHandle_Geom_Plane>(new opencascade::handle<Geom_Plane>(plane_handle));
}

inline std::unique_ptr<gp_Pnt> HandleGeomCurve_Value(const RustHandle_Geom_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve->Value(U)));
}

inline const RustHandle_Standard_Type &DynamicType(const RustHandle_Geom_Surface &surface) {
  return surface->DynamicType();
}
