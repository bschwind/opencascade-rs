#include "rust/cxx.h"
#include <gp_Ax2.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp.hxx>
#include <gp_Vec.hxx>
#include <GC_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <Geom_Surface.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopAbs_ShapeEnum.hxx>
#include <TopExp_Explorer.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepMesh_IncrementalMesh.hxx>
#include <BRepPrimAPI_MakePrism.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <StlAPI_Writer.hxx>

// Handles
typedef opencascade::handle<Geom_Curve> HandleGeomCurve;
typedef opencascade::handle<Geom_TrimmedCurve> HandleGeomTrimmedCurve;
typedef opencascade::handle<Geom_Surface> HandleGeomSurface;

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

// Primitives
std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape& shape, const gp_Vec& vec, const Standard_Boolean copy, const Standard_Boolean canonize);
std::unique_ptr<BRepPrimAPI_MakeCylinder> BRepPrimAPI_MakeCylinder_ctor(const gp_Ax2& coord_system, const Standard_Real radius, const Standard_Real height);

// Boolean operations
std::unique_ptr<BRepAlgoAPI_Fuse> BRepAlgoAPI_Fuse_ctor(const TopoDS_Shape& shape_1, const TopoDS_Shape& shape_2);

// Fillets
std::unique_ptr<BRepFilletAPI_MakeFillet> BRepFilletAPI_MakeFillet_ctor(const TopoDS_Shape& shape);

// Geometric processor
const gp_Ax1& gp_OX();
const gp_Dir& gp_DZ();

std::unique_ptr<gp_Ax2> gp_Ax2_ctor(const gp_Pnt& origin, const gp_Dir& main_dir);

// Shape stuff
const TopoDS_Wire& TopoDS_cast_to_wire(const TopoDS_Shape& shape);
const TopoDS_Edge& TopoDS_cast_to_edge(const TopoDS_Shape& shape);
const TopoDS_Face& TopoDS_cast_to_face(const TopoDS_Shape& shape);

// Transforms
std::unique_ptr<gp_Trsf> new_transform();

std::unique_ptr<BRepBuilderAPI_Transform> BRepBuilderAPI_Transform_ctor(const TopoDS_Shape& shape, const gp_Trsf& transform, const Standard_Boolean copy);

// Topology Explorer
std::unique_ptr<TopExp_Explorer> TopExp_Explorer_ctor(const TopoDS_Shape& shape, const TopAbs_ShapeEnum to_find);
std::unique_ptr<HandleGeomSurface> BRep_Tool_Surface(const TopoDS_Face& face);

// Data export
std::unique_ptr<StlAPI_Writer> StlAPI_Writer_ctor();
bool write_stl(StlAPI_Writer& writer, const TopoDS_Shape& theShape, rust::String theFileName);

// Triangulation
std::unique_ptr<BRepMesh_IncrementalMesh> BRepMesh_IncrementalMesh_ctor(const TopoDS_Shape& shape, double deflection);
