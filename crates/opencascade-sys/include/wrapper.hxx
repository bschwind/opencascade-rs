#include "rust/cxx.h"
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp.hxx>
#include <gp_Vec.hxx>
#include <GC_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <GCE2d_MakeSegment.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <Geom_Surface.hxx>
#include <Geom_Plane.hxx>
#include <Geom2d_Ellipse.hxx>
#include <Geom2d_TrimmedCurve.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Shape.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopAbs_ShapeEnum.hxx>
#include <TopExp_Explorer.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Section.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepFilletAPI_MakeChamfer.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepLib.hxx>
#include <BRepMesh_IncrementalMesh.hxx>
#include <BRepPrimAPI_MakePrism.hxx>
#include <BRepPrimAPI_MakeRevol.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepOffsetAPI_MakeThickSolid.hxx>
#include <BRepOffsetAPI_ThruSections.hxx>
#include <StlAPI_Writer.hxx>
#include <Standard_Type.hxx>

// Handles
typedef opencascade::handle<Standard_Type> HandleStandardType;
typedef opencascade::handle<Geom_Curve> HandleGeomCurve;
typedef opencascade::handle<Geom_TrimmedCurve> HandleGeomTrimmedCurve;
typedef opencascade::handle<Geom_Surface> HandleGeomSurface;
typedef opencascade::handle<Geom_Plane> HandleGeomPlane;
typedef opencascade::handle<Geom2d_Curve> HandleGeom2d_Curve;
typedef opencascade::handle<Geom2d_Ellipse> HandleGeom2d_Ellipse;
typedef opencascade::handle<Geom2d_TrimmedCurve> HandleGeom2d_TrimmedCurve;
typedef opencascade::handle<Geom_CylindricalSurface> HandleGeom_CylindricalSurface;

const HandleStandardType& DynamicType(const HandleGeomSurface& surface);
rust::String type_name(const HandleStandardType& handle);

// General Shape Stuff
std::unique_ptr<TopoDS_Shape> new_shape(const TopoDS_Shape& shape);

// Collections
std::unique_ptr<TopTools_ListOfShape> new_list_of_shape();
void shape_list_append_face(TopTools_ListOfShape& list, const TopoDS_Face& face);

// Geometry
const gp_Pnt& handle_geom_plane_location(const HandleGeomPlane& plane);
std::unique_ptr<HandleGeom_CylindricalSurface> Geom_CylindricalSurface_ctor(const gp_Ax3& axis, double radius);
std::unique_ptr<HandleGeomSurface> cylinder_to_surface(const HandleGeom_CylindricalSurface& cylinder_handle);
std::unique_ptr<HandleGeom2d_Ellipse> Geom2d_Ellipse_ctor(const gp_Ax2d& axis, double major_radius, double minor_radius);
std::unique_ptr<HandleGeom2d_Curve> ellipse_to_HandleGeom2d_Curve(const HandleGeom2d_Ellipse& ellipse_handle);
std::unique_ptr<HandleGeom2d_TrimmedCurve> Geom2d_TrimmedCurve_ctor(const HandleGeom2d_Curve& curve, double u1, double u2);
std::unique_ptr<HandleGeom2d_Curve> HandleGeom2d_TrimmedCurve_to_curve(const HandleGeom2d_TrimmedCurve& trimmed_curve);
std::unique_ptr<gp_Pnt2d> ellipse_value(const HandleGeom2d_Ellipse& ellipse, double u);


std::unique_ptr<HandleGeomCurve> new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve& trimmed_curve);
std::unique_ptr<HandleGeomPlane> new_HandleGeomPlane_from_HandleGeomSurface(const HandleGeomSurface& surface);

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z);
std::unique_ptr<gp_Pnt2d> new_point_2d(double x, double y);
std::unique_ptr<gp_Vec> new_vec(double x, double y, double z);

// Line segment stuff
std::unique_ptr<GC_MakeSegment> GC_MakeSegment_point_point(const gp_Pnt& p1, const gp_Pnt& p2);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment& segment);
std::unique_ptr<HandleGeom2d_TrimmedCurve> GCE2d_MakeSegment_point_point(const gp_Pnt2d& p1, const gp_Pnt2d& p2);

// Arc stuff
std::unique_ptr<GC_MakeArcOfCircle> GC_MakeArcOfCircle_point_point_point(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3);
std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle& arc);

// BRepBuilderAPI stuff
std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_HandleGeomCurve(const HandleGeomCurve& geom_curve);
std::unique_ptr<BRepBuilderAPI_MakeEdge> BRepBuilderAPI_MakeEdge_CurveSurface2d(const HandleGeom2d_Curve& curve_handle, const HandleGeomSurface& surface_handle);
std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_ctor();
std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge(const TopoDS_Edge& edge_1, const TopoDS_Edge& edge_2);
std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge_edge(const TopoDS_Edge& edge_1, const TopoDS_Edge& edge_2, const TopoDS_Edge& edge_3);

std::unique_ptr<BRepBuilderAPI_MakeFace> BRepBuilderAPI_MakeFace_wire(const TopoDS_Wire& wire, const Standard_Boolean only_plane);

// Primitives
std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape& shape, const gp_Vec& vec, const Standard_Boolean copy, const Standard_Boolean canonize);
std::unique_ptr<BRepPrimAPI_MakeRevol> BRepPrimAPI_MakeRevol_ctor(const TopoDS_Shape& shape, const gp_Ax1& axis, const Standard_Real angle, const Standard_Boolean copy);
std::unique_ptr<BRepPrimAPI_MakeCylinder> BRepPrimAPI_MakeCylinder_ctor(const gp_Ax2& coord_system, const Standard_Real radius, const Standard_Real height);
std::unique_ptr<BRepPrimAPI_MakeBox> BRepPrimAPI_MakeBox_ctor(const gp_Pnt& point, double dx, double dy, double dz);
std::unique_ptr<BRepPrimAPI_MakeSphere> BRepPrimAPI_MakeSphere_ctor(double r);

// BRepLib
bool BRepLibBuildCurves3d(const TopoDS_Shape& shape);

// Boolean operations
std::unique_ptr<BRepAlgoAPI_Fuse> BRepAlgoAPI_Fuse_ctor(const TopoDS_Shape& shape_1, const TopoDS_Shape& shape_2);
std::unique_ptr<BRepAlgoAPI_Cut> BRepAlgoAPI_Cut_ctor(const TopoDS_Shape& shape_1, const TopoDS_Shape& shape_2);
std::unique_ptr<BRepAlgoAPI_Section> BRepAlgoAPI_Section_ctor(const TopoDS_Shape& shape_1, const TopoDS_Shape& shape_2);

// Fillets
std::unique_ptr<BRepFilletAPI_MakeFillet> BRepFilletAPI_MakeFillet_ctor(const TopoDS_Shape& shape);

// Chamfers
std::unique_ptr<BRepFilletAPI_MakeChamfer> BRepFilletAPI_MakeChamfer_ctor(const TopoDS_Shape& shape);

// Solids
std::unique_ptr<BRepOffsetAPI_MakeThickSolid> BRepOffsetAPI_MakeThickSolid_ctor();
void MakeThickSolidByJoin(
    BRepOffsetAPI_MakeThickSolid& make_thick_solid,
    const TopoDS_Shape& S,
    const TopTools_ListOfShape& ClosingFaces,
    const Standard_Real Offset,
    const Standard_Real Tol
);

// Lofting
std::unique_ptr<BRepOffsetAPI_ThruSections> BRepOffsetAPI_ThruSections_ctor(bool is_solid);

// Geometric processor
const gp_Ax1& gp_OX();
const gp_Ax1& gp_OY();
const gp_Ax1& gp_OZ();
const gp_Dir& gp_DZ();

std::unique_ptr<gp_Ax2> gp_Ax2_ctor(const gp_Pnt& origin, const gp_Dir& main_dir);
std::unique_ptr<gp_Ax3> gp_Ax3_from_gp_Ax2(const gp_Ax2& axis);
std::unique_ptr<gp_Dir2d> gp_Dir2d_ctor(double x, double y);
std::unique_ptr<gp_Ax2d> gp_Ax2d_ctor(const gp_Pnt2d& point, const gp_Dir2d& dir);

// Shape stuff
const TopoDS_Wire& TopoDS_cast_to_wire(const TopoDS_Shape& shape);
const TopoDS_Edge& TopoDS_cast_to_edge(const TopoDS_Shape& shape);
std::unique_ptr<TopoDS_Face> TopoDS_cast_to_face(const TopoDS_Shape& shape);

// Compound Shapes
std::unique_ptr<TopoDS_Shape> TopoDS_Compound_as_shape(std::unique_ptr<TopoDS_Compound> compound);
std::unique_ptr<TopoDS_Compound> TopoDS_Compound_ctor();
std::unique_ptr<BRep_Builder> BRep_Builder_ctor();
const TopoDS_Builder& BRep_Builder_upcast_to_topods_builder(const BRep_Builder& builder);

// Transforms
std::unique_ptr<gp_Trsf> new_transform();

std::unique_ptr<BRepBuilderAPI_Transform> BRepBuilderAPI_Transform_ctor(const TopoDS_Shape& shape, const gp_Trsf& transform, const Standard_Boolean copy);

// Topology Explorer
std::unique_ptr<TopExp_Explorer> TopExp_Explorer_ctor(const TopoDS_Shape& shape, const TopAbs_ShapeEnum to_find);
std::unique_ptr<HandleGeomSurface> BRep_Tool_Surface(const TopoDS_Face& face);
std::unique_ptr<TopoDS_Shape> ExplorerCurrentShape(const TopExp_Explorer& explorer);

// Data export
std::unique_ptr<StlAPI_Writer> StlAPI_Writer_ctor();
bool write_stl(StlAPI_Writer& writer, const TopoDS_Shape& theShape, rust::String theFileName);

// Triangulation
std::unique_ptr<BRepMesh_IncrementalMesh> BRepMesh_IncrementalMesh_ctor(const TopoDS_Shape& shape, double deflection);
