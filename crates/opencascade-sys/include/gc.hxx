#include <GCE2d_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <GC_MakeSegment.hxx>
#include <bindings_common.hxx>

inline std::unique_ptr<Handle_Geom_TrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment &segment) {
  return std::unique_ptr<Handle_Geom_TrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(segment.Value()));
}

inline std::unique_ptr<Handle_Geom2d_TrimmedCurve> GCE2d_MakeSegment_point_point(const gp_Pnt2d &p1,
                                                                                 const gp_Pnt2d &p2) {
  return std::unique_ptr<Handle_Geom2d_TrimmedCurve>(
      new opencascade::handle<Geom2d_TrimmedCurve>(GCE2d_MakeSegment(p1, p2)));
}

inline std::unique_ptr<Handle_Geom_TrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle &arc) {
  return std::unique_ptr<Handle_Geom_TrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(arc.Value()));
}
