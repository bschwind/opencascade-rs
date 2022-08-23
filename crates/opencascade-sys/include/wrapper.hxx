#include <gp_Pnt.hxx>
#include <gp.hxx>
#include <GC_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <TopoDS_Edge.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>

// Handles
// typedef opencascade::handle<Geom_Curve> GeomCurveHandle;
typedef opencascade::handle<Geom_Curve> HandleGeomCurve;
typedef opencascade::handle<Geom_TrimmedCurve> HandleGeomTrimmedCurve;

std::unique_ptr<HandleGeomCurve> new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve& trimmed_curve);

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z);

// Line segment stuff
std::unique_ptr<Geom_TrimmedCurve> new_segment(const gp_Pnt& p1, const gp_Pnt& p2);
std::unique_ptr<GC_MakeSegment> GC_MakeSegment_point_point(const gp_Pnt& p1, const gp_Pnt& p2);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment& segment);

// Arc stuff
std::unique_ptr<Geom_TrimmedCurve> new_arc_of_circle(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);
std::unique_ptr<GC_MakeArcOfCircle> GC_MakeArcOfCircle_point_point_point(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle& arc);

// Topo stuff
std::unique_ptr<TopoDS_Edge> make_edge(const Geom_TrimmedCurve& geom_curve);

// BRepBuilderAPI stuff
std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_HandleGeomCurve(const HandleGeomCurve& geom_curve);
// std::unique_ptr<TopoDS_Edge> BRepBuilderAPI_MakeEdge_Edge(BRepBuilderAPI_MakeEdge& make_edge);
// TopoDS_Edge& BRepBuilderAPI_MakeEdge_Edge(BRepBuilderAPI_MakeEdge& make_edge);

std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge_edge(const TopoDS_Edge& edge_1, const TopoDS_Edge& edge_2, const TopoDS_Edge& edge_3);

// Geometric processor
const gp_Ax1& gp_OX();
