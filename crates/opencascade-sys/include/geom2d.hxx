#include <Geom2d_Ellipse.hxx>
#include <Geom2d_TrimmedCurve.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<Handle_Geom2d_Ellipse> Geom2d_Ellipse_new(const gp_Ax2d &axis, double major_radius,
                                                                 double minor_radius) {
  return std::unique_ptr<Handle_Geom2d_Ellipse>(
      new opencascade::handle<Geom2d_Ellipse>(new Geom2d_Ellipse(axis, major_radius, minor_radius)));
}

inline std::unique_ptr<Handle_Geom2d_Curve> ellipse_to_HandleGeom2d_Curve(const Handle_Geom2d_Ellipse &ellipse_handle) {
  return std::unique_ptr<Handle_Geom2d_Curve>(new opencascade::handle<Geom2d_Curve>(ellipse_handle));
}

inline std::unique_ptr<Handle_Geom2d_TrimmedCurve> Geom2d_TrimmedCurve_new(const Handle_Geom2d_Curve &curve, double u1,
                                                                           double u2) {
  return std::unique_ptr<Handle_Geom2d_TrimmedCurve>(
      new opencascade::handle<Geom2d_TrimmedCurve>(new Geom2d_TrimmedCurve(curve, u1, u2)));
}

inline std::unique_ptr<Handle_Geom2d_Curve>
HandleGeom2d_TrimmedCurve_to_curve(const Handle_Geom2d_TrimmedCurve &trimmed_curve) {
  return std::unique_ptr<Handle_Geom2d_Curve>(new opencascade::handle<Geom2d_Curve>(trimmed_curve));
}

inline std::unique_ptr<gp_Pnt2d> ellipse_value(const Handle_Geom2d_Ellipse &ellipse, double u) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(ellipse->Value(u)));
}
