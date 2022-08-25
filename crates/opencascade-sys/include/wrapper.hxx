#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp.hxx>
#include <gp_Vec.hxx>
#include <GC_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Edge.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepPrimAPI_MakePrism.hxx>

// Handles
typedef opencascade::handle<Geom_Curve> HandleGeomCurve;
typedef opencascade::handle<Geom_TrimmedCurve> HandleGeomTrimmedCurve;

std::unique_ptr<HandleGeomCurve> new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve& trimmed_curve);

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z);
std::unique_ptr<gp_Vec> new_vec(double x, double y, double z);

// Line segment stuff
std::unique_ptr<GC_MakeSegment> GC_MakeSegment_point_point(const gp_Pnt& p1, const gp_Pnt& p2);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment& segment);

// Arc stuff
std::unique_ptr<GC_MakeArcOfCircle> GC_MakeArcOfCircle_point_point_point(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle& arc);

// BRepBuilderAPI stuff
std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_HandleGeomCurve(const HandleGeomCurve& geom_curve);
std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_ctor();
std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge_edge(const TopoDS_Edge& edge_1, const TopoDS_Edge& edge_2, const TopoDS_Edge& edge_3);

std::unique_ptr<BRepBuilderAPI_MakeFace> BRepBuilderAPI_MakeFace_wire(const TopoDS_Wire& wire, const Standard_Boolean only_plane);

std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape& shape, const gp_Vec& vec, const Standard_Boolean copy, const Standard_Boolean canonize);

// Geometric processor
const gp_Ax1& gp_OX();

// Shape stuff
const TopoDS_Wire& TopoDS_cast_to_wire(const TopoDS_Shape& shape);

// Transforms
std::unique_ptr<gp_Trsf> new_transform();

std::unique_ptr<BRepBuilderAPI_Transform> BRepBuilderAPI_Transform_ctor(const TopoDS_Shape& shape, const gp_Trsf& transform, const Standard_Boolean copy);
