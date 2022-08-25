#include <wrapper.hxx>

// Handle stuff
std::unique_ptr<HandleGeomCurve> new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve& trimmed_curve) {
  return std::unique_ptr<HandleGeomCurve>(new opencascade::handle<Geom_Curve>(trimmed_curve));
}

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(x, y, z));
}

std::unique_ptr<gp_Vec> new_vec(double x, double y, double z) {
  return std::unique_ptr<gp_Vec>(new gp_Vec(x, y, z));
}

// Segment Stuff
std::unique_ptr<GC_MakeSegment> GC_MakeSegment_point_point(const gp_Pnt& p1, const gp_Pnt& p2) {
  return std::unique_ptr<GC_MakeSegment>(new GC_MakeSegment(p1, p2));
}

std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment& segment) {
  return std::unique_ptr<HandleGeomTrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(segment.Value()));
}

// Arc stuff
std::unique_ptr<GC_MakeArcOfCircle> GC_MakeArcOfCircle_point_point_point(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3) {
  return std::unique_ptr<GC_MakeArcOfCircle>(new GC_MakeArcOfCircle(p1, p2, p3));
}

std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle& arc) {
  return std::unique_ptr<HandleGeomTrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(arc.Value()));
}

// BRepBuilderAPI stuff
std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_HandleGeomCurve(const HandleGeomCurve &geom_curve) {
  return std::unique_ptr<BRepBuilderAPI_MakeEdge>(new BRepBuilderAPI_MakeEdge(geom_curve));
}

std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_ctor() {
  return std::unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire());
}

std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge_edge(const TopoDS_Edge& edge_1, const TopoDS_Edge& edge_2, const TopoDS_Edge& edge_3) {
  return std::unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire(edge_1, edge_2, edge_3));
}

std::unique_ptr<BRepBuilderAPI_MakeFace> BRepBuilderAPI_MakeFace_wire(const TopoDS_Wire& wire, const Standard_Boolean only_plane) {
  return std::unique_ptr<BRepBuilderAPI_MakeFace>(new BRepBuilderAPI_MakeFace(wire, only_plane));
}

std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape& shape, const gp_Vec& vec, const Standard_Boolean copy, const Standard_Boolean canonize) {
  return std::unique_ptr<BRepPrimAPI_MakePrism>(new BRepPrimAPI_MakePrism(shape, vec, copy, canonize));
}

// Geometric processing
const gp_Ax1& gp_OX() {
  return gp::OX();
}

// Shape stuff
const TopoDS_Wire& TopoDS_cast_to_wire(const TopoDS_Shape& shape) {
  return TopoDS::Wire(shape);
}

// Transforms
std::unique_ptr<gp_Trsf> new_transform() {
  return std::unique_ptr<gp_Trsf>(new gp_Trsf());
}

std::unique_ptr<BRepBuilderAPI_Transform> BRepBuilderAPI_Transform_ctor(const TopoDS_Shape& shape, const gp_Trsf& transform, const Standard_Boolean copy) {
  return std::unique_ptr<BRepBuilderAPI_Transform>(new BRepBuilderAPI_Transform(shape, transform, copy));
}
