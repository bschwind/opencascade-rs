#include <gp_Pnt.hxx>
#include <GC_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <TopoDS_Edge.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>

// Handles
// typedef opencascade::handle<Geom_Curve> GeomCurveHandle;

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z);

// Line segment stuff
std::unique_ptr<Geom_TrimmedCurve> new_segment(const gp_Pnt& p1, const gp_Pnt& p2);

// Arc stuff
std::unique_ptr<Geom_TrimmedCurve> new_arc_of_circle(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);

// Topo stuff
std::unique_ptr<TopoDS_Edge> make_edge(const Geom_TrimmedCurve &geom_curve);
