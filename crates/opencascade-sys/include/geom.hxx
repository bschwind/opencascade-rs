#include <Geom_BSplineCurve.hxx>
#include <Geom_BezierCurve.hxx>
#include <Geom_BezierSurface.hxx>
#include <Geom_Circle.hxx>
#include <Geom_ConicalSurface.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom_Plane.hxx>
#include <Geom_SphericalSurface.hxx>
#include <Geom_Surface.hxx>
#include <Geom_ToroidalSurface.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <bindings_common.hxx>
#include <gp_Ax3.hxx>
#include <gp_Circ.hxx>

inline std::unique_ptr<Handle_Geom_CylindricalSurface> Geom_CylindricalSurface_new(const gp_Ax3 &axis, double radius) {
  return std::unique_ptr<Handle_Geom_CylindricalSurface>(
      new opencascade::handle<Geom_CylindricalSurface>(new Geom_CylindricalSurface(axis, radius)));
}

inline std::unique_ptr<Handle_Geom_Surface> cylinder_to_surface(const Handle_Geom_CylindricalSurface &cylinder_handle) {
  return std::unique_ptr<Handle_Geom_Surface>(new opencascade::handle<Geom_Surface>(cylinder_handle));
}

inline std::unique_ptr<Handle_Geom_BezierSurface> Geom_BezierSurface_new(const TColgp_Array2OfPnt &poles) {
  return std::unique_ptr<Handle_Geom_BezierSurface>(
      new opencascade::handle<Geom_BezierSurface>(new Geom_BezierSurface(poles)));
}

inline const gp_Pnt &handle_geom_plane_location(const Handle_Geom_Plane &plane) { return plane->Location(); }

inline std::unique_ptr<Handle_Geom_BezierCurve>
Geom_BezierCurve_to_handle(std::unique_ptr<Geom_BezierCurve> bezier_curve) {
  return std::unique_ptr<Handle_Geom_BezierCurve>(new Handle_Geom_BezierCurve(bezier_curve.release()));
}

inline std::unique_ptr<Handle_Geom_Surface> bezier_to_surface(const Handle_Geom_BezierSurface &bezier_handle) {
  return std::unique_ptr<Handle_Geom_Surface>(new opencascade::handle<Geom_Surface>(bezier_handle));
}

inline std::unique_ptr<Handle_Geom_Plane>
new_HandleGeomPlane_from_HandleGeomSurface(const Handle_Geom_Surface &surface) {
  Handle_Geom_Plane plane_handle = opencascade::handle<Geom_Plane>::DownCast(surface);
  return std::unique_ptr<Handle_Geom_Plane>(new opencascade::handle<Geom_Plane>(plane_handle));
}

inline std::unique_ptr<gp_Pnt> HandleGeomCurve_Value(const Handle_Geom_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve->Value(U)));
}

inline const Handle_Standard_Type &DynamicType(const Handle_Geom_Surface &surface) { return surface->DynamicType(); }

inline std::unique_ptr<Handle_Geom_Circle> new_HandleGeomCircle_from_HandleGeomCurve(const Handle_Geom_Curve &curve) {
  Handle_Geom_Circle circle_handle = opencascade::handle<Geom_Circle>::DownCast(curve);
  return std::unique_ptr<Handle_Geom_Circle>(new opencascade::handle<Geom_Circle>(circle_handle));
}

inline std::unique_ptr<gp_Circ> HandleGeomCircle_Circ(const Handle_Geom_Circle &circle) {
  return std::unique_ptr<gp_Circ>(new gp_Circ(circle->Circ()));
}

// CylindricalSurface downcast + accessors

inline std::unique_ptr<Handle_Geom_CylindricalSurface>
new_HandleGeomCylindricalSurface_from_HandleGeomSurface(const Handle_Geom_Surface &surface) {
  Handle_Geom_CylindricalSurface handle = opencascade::handle<Geom_CylindricalSurface>::DownCast(surface);
  return std::unique_ptr<Handle_Geom_CylindricalSurface>(new opencascade::handle<Geom_CylindricalSurface>(handle));
}

inline double HandleGeomCylindricalSurface_Radius(const Handle_Geom_CylindricalSurface &cyl) { return cyl->Radius(); }

inline const gp_Ax3 &HandleGeomCylindricalSurface_Position(const Handle_Geom_CylindricalSurface &cyl) {
  return cyl->Position();
}

// SphericalSurface downcast + accessors

inline std::unique_ptr<Handle_Geom_SphericalSurface>
new_HandleGeomSphericalSurface_from_HandleGeomSurface(const Handle_Geom_Surface &surface) {
  Handle_Geom_SphericalSurface handle = opencascade::handle<Geom_SphericalSurface>::DownCast(surface);
  return std::unique_ptr<Handle_Geom_SphericalSurface>(new opencascade::handle<Geom_SphericalSurface>(handle));
}

inline double HandleGeomSphericalSurface_Radius(const Handle_Geom_SphericalSurface &sphere) { return sphere->Radius(); }

inline const gp_Ax3 &HandleGeomSphericalSurface_Position(const Handle_Geom_SphericalSurface &sphere) {
  return sphere->Position();
}

// ConicalSurface downcast + accessors

inline std::unique_ptr<Handle_Geom_ConicalSurface>
new_HandleGeomConicalSurface_from_HandleGeomSurface(const Handle_Geom_Surface &surface) {
  Handle_Geom_ConicalSurface handle = opencascade::handle<Geom_ConicalSurface>::DownCast(surface);
  return std::unique_ptr<Handle_Geom_ConicalSurface>(new opencascade::handle<Geom_ConicalSurface>(handle));
}

inline double HandleGeomConicalSurface_RefRadius(const Handle_Geom_ConicalSurface &cone) { return cone->RefRadius(); }

inline double HandleGeomConicalSurface_SemiAngle(const Handle_Geom_ConicalSurface &cone) { return cone->SemiAngle(); }

inline const gp_Ax3 &HandleGeomConicalSurface_Position(const Handle_Geom_ConicalSurface &cone) {
  return cone->Position();
}

// ToroidalSurface downcast + accessors

inline std::unique_ptr<Handle_Geom_ToroidalSurface>
new_HandleGeomToroidalSurface_from_HandleGeomSurface(const Handle_Geom_Surface &surface) {
  Handle_Geom_ToroidalSurface handle = opencascade::handle<Geom_ToroidalSurface>::DownCast(surface);
  return std::unique_ptr<Handle_Geom_ToroidalSurface>(new opencascade::handle<Geom_ToroidalSurface>(handle));
}

inline double HandleGeomToroidalSurface_MajorRadius(const Handle_Geom_ToroidalSurface &torus) {
  return torus->MajorRadius();
}

inline double HandleGeomToroidalSurface_MinorRadius(const Handle_Geom_ToroidalSurface &torus) {
  return torus->MinorRadius();
}

inline const gp_Ax3 &HandleGeomToroidalSurface_Position(const Handle_Geom_ToroidalSurface &torus) {
  return torus->Position();
}
