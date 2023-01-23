#include "rust/cxx.h"
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Section.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepFilletAPI_MakeChamfer.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepLib.hxx>
#include <BRepMesh_IncrementalMesh.hxx>
#include <BRepOffsetAPI_MakeThickSolid.hxx>
#include <BRepOffsetAPI_ThruSections.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakePrism.hxx>
#include <BRepPrimAPI_MakeRevol.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <GCE2d_MakeSegment.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <GC_MakeSegment.hxx>
#include <Geom2d_Ellipse.hxx>
#include <Geom2d_TrimmedCurve.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <Geom_Plane.hxx>
#include <Geom_Surface.hxx>
#include <Geom_TrimmedCurve.hxx>
#include <Standard_Type.hxx>
#include <StlAPI_Writer.hxx>
#include <TopAbs_ShapeEnum.hxx>
#include <TopExp_Explorer.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <gp.hxx>
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>

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

// Runtime
inline std::unique_ptr<Message_ProgressRange> Message_ProgressRange_ctor() {
  return std::unique_ptr<Message_ProgressRange>(new Message_ProgressRange());
}

// Handle stuff

inline const HandleStandardType &DynamicType(const HandleGeomSurface &surface) { return surface->DynamicType(); }

inline rust::String type_name(const HandleStandardType &handle) { return std::string(handle->Name()); }

inline std::unique_ptr<gp_Pnt> HandleGeomCurve_Value(const HandleGeomCurve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve->Value(U)));
}

inline std::unique_ptr<HandleGeomCurve>
new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve &trimmed_curve) {
  return std::unique_ptr<HandleGeomCurve>(new opencascade::handle<Geom_Curve>(trimmed_curve));
}

inline std::unique_ptr<HandleGeomPlane> new_HandleGeomPlane_from_HandleGeomSurface(const HandleGeomSurface &surface) {
  HandleGeomPlane plane_handle = opencascade::handle<Geom_Plane>::DownCast(surface);
  return std::unique_ptr<HandleGeomPlane>(new opencascade::handle<Geom_Plane>(plane_handle));
}

// Collections
inline std::unique_ptr<TopTools_ListOfShape> new_list_of_shape() {
  return std::unique_ptr<TopTools_ListOfShape>(new TopTools_ListOfShape());
}

inline void shape_list_append_face(TopTools_ListOfShape &list, const TopoDS_Face &face) { list.Append(face); }

// Geometry
inline const gp_Pnt &handle_geom_plane_location(const HandleGeomPlane &plane) { return plane->Location(); }

inline std::unique_ptr<HandleGeom_CylindricalSurface> Geom_CylindricalSurface_ctor(const gp_Ax3 &axis, double radius) {
  return std::unique_ptr<HandleGeom_CylindricalSurface>(
      new opencascade::handle<Geom_CylindricalSurface>(new Geom_CylindricalSurface(axis, radius)));
}

inline std::unique_ptr<HandleGeomSurface> cylinder_to_surface(const HandleGeom_CylindricalSurface &cylinder_handle) {
  return std::unique_ptr<HandleGeomSurface>(new opencascade::handle<Geom_Surface>(cylinder_handle));
}

inline std::unique_ptr<HandleGeom2d_Ellipse> Geom2d_Ellipse_ctor(const gp_Ax2d &axis, double major_radius,
                                                                 double minor_radius) {
  return std::unique_ptr<HandleGeom2d_Ellipse>(
      new opencascade::handle<Geom2d_Ellipse>(new Geom2d_Ellipse(axis, major_radius, minor_radius)));
}

inline std::unique_ptr<HandleGeom2d_Curve> ellipse_to_HandleGeom2d_Curve(const HandleGeom2d_Ellipse &ellipse_handle) {
  return std::unique_ptr<HandleGeom2d_Curve>(new opencascade::handle<Geom2d_Curve>(ellipse_handle));
}

inline std::unique_ptr<HandleGeom2d_TrimmedCurve> Geom2d_TrimmedCurve_ctor(const HandleGeom2d_Curve &curve, double u1,
                                                                           double u2) {
  return std::unique_ptr<HandleGeom2d_TrimmedCurve>(
      new opencascade::handle<Geom2d_TrimmedCurve>(new Geom2d_TrimmedCurve(curve, u1, u2)));
}

inline std::unique_ptr<HandleGeom2d_Curve>
HandleGeom2d_TrimmedCurve_to_curve(const HandleGeom2d_TrimmedCurve &trimmed_curve) {
  return std::unique_ptr<HandleGeom2d_Curve>(new opencascade::handle<Geom2d_Curve>(trimmed_curve));
}

inline std::unique_ptr<gp_Pnt2d> ellipse_value(const HandleGeom2d_Ellipse &ellipse, double u) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(ellipse->Value(u)));
}

// Point stuff
inline std::unique_ptr<gp_Pnt> new_point(double x, double y, double z) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(x, y, z));
}

inline std::unique_ptr<gp_Pnt2d> new_point_2d(double x, double y) {
  return std::unique_ptr<gp_Pnt2d>(new gp_Pnt2d(x, y));
}

inline std::unique_ptr<gp_Vec> new_vec(double x, double y, double z) {
  return std::unique_ptr<gp_Vec>(new gp_Vec(x, y, z));
}

// Segment Stuff
inline std::unique_ptr<GC_MakeSegment> GC_MakeSegment_point_point(const gp_Pnt &p1, const gp_Pnt &p2) {
  return std::unique_ptr<GC_MakeSegment>(new GC_MakeSegment(p1, p2));
}

inline std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeSegment_Value(const GC_MakeSegment &segment) {
  return std::unique_ptr<HandleGeomTrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(segment.Value()));
}

inline std::unique_ptr<HandleGeom2d_TrimmedCurve> GCE2d_MakeSegment_point_point(const gp_Pnt2d &p1,
                                                                                const gp_Pnt2d &p2) {
  return std::unique_ptr<HandleGeom2d_TrimmedCurve>(
      new opencascade::handle<Geom2d_TrimmedCurve>(GCE2d_MakeSegment(p1, p2)));
}

// Arc stuff
inline std::unique_ptr<GC_MakeArcOfCircle> GC_MakeArcOfCircle_point_point_point(const gp_Pnt &p1, const gp_Pnt &p2,
                                                                                const gp_Pnt &p3) {
  return std::unique_ptr<GC_MakeArcOfCircle>(new GC_MakeArcOfCircle(p1, p2, p3));
}

inline std::unique_ptr<HandleGeomTrimmedCurve> GC_MakeArcOfCircle_Value(const GC_MakeArcOfCircle &arc) {
  return std::unique_ptr<HandleGeomTrimmedCurve>(new opencascade::handle<Geom_TrimmedCurve>(arc.Value()));
}

// BRepBuilderAPI stuff
inline std::unique_ptr<BRepBuilderAPI_MakeEdge>
BRepBuilderAPI_MakeEdge_HandleGeomCurve(const HandleGeomCurve &geom_curve) {
  return std::unique_ptr<BRepBuilderAPI_MakeEdge>(new BRepBuilderAPI_MakeEdge(geom_curve));
}

inline std::unique_ptr<BRepBuilderAPI_MakeEdge>
BRepBuilderAPI_MakeEdge_CurveSurface2d(const HandleGeom2d_Curve &curve_handle,
                                       const HandleGeomSurface &surface_handle) {
  return std::unique_ptr<BRepBuilderAPI_MakeEdge>(new BRepBuilderAPI_MakeEdge(curve_handle, surface_handle));
}

inline std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_ctor() {
  return std::unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire());
}

inline std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge_edge(const TopoDS_Edge &edge_1,
                                                                                       const TopoDS_Edge &edge_2,
                                                                                       const TopoDS_Edge &edge_3) {
  return std::unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire(edge_1, edge_2, edge_3));
}

inline std::unique_ptr<BRepBuilderAPI_MakeWire> BRepBuilderAPI_MakeWire_edge_edge(const TopoDS_Edge &edge_1,
                                                                                  const TopoDS_Edge &edge_2) {
  return std::unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire(edge_1, edge_2));
}

inline std::unique_ptr<BRepBuilderAPI_MakeFace> BRepBuilderAPI_MakeFace_wire(const TopoDS_Wire &wire,
                                                                             const Standard_Boolean only_plane) {
  return std::unique_ptr<BRepBuilderAPI_MakeFace>(new BRepBuilderAPI_MakeFace(wire, only_plane));
}

// Primitives
inline std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape &shape, const gp_Vec &vec,
                                                                         const Standard_Boolean copy,
                                                                         const Standard_Boolean canonize) {
  return std::unique_ptr<BRepPrimAPI_MakePrism>(new BRepPrimAPI_MakePrism(shape, vec, copy, canonize));
}

inline std::unique_ptr<BRepPrimAPI_MakeRevol> BRepPrimAPI_MakeRevol_ctor(const TopoDS_Shape &shape, const gp_Ax1 &axis,
                                                                         const Standard_Real angle,
                                                                         const Standard_Boolean copy) {
  return std::unique_ptr<BRepPrimAPI_MakeRevol>(new BRepPrimAPI_MakeRevol(shape, axis, angle, copy));
}

inline std::unique_ptr<BRepPrimAPI_MakeCylinder>
BRepPrimAPI_MakeCylinder_ctor(const gp_Ax2 &coord_system, const Standard_Real radius, const Standard_Real height) {
  return std::unique_ptr<BRepPrimAPI_MakeCylinder>(new BRepPrimAPI_MakeCylinder(coord_system, radius, height));
}

inline std::unique_ptr<BRepPrimAPI_MakeBox> BRepPrimAPI_MakeBox_ctor(const gp_Pnt &point, double dx, double dy,
                                                                     double dz) {
  return std::unique_ptr<BRepPrimAPI_MakeBox>(new BRepPrimAPI_MakeBox(point, dx, dy, dz));
}

inline std::unique_ptr<BRepPrimAPI_MakeSphere> BRepPrimAPI_MakeSphere_ctor(double r) {
  return std::unique_ptr<BRepPrimAPI_MakeSphere>(new BRepPrimAPI_MakeSphere(r));
}

// BRepLib
inline bool BRepLibBuildCurves3d(const TopoDS_Shape &shape) { return BRepLib::BuildCurves3d(shape); }

// Boolean operations
inline std::unique_ptr<BRepAlgoAPI_Fuse> BRepAlgoAPI_Fuse_ctor(const TopoDS_Shape &shape_1,
                                                               const TopoDS_Shape &shape_2) {
  return std::unique_ptr<BRepAlgoAPI_Fuse>(new BRepAlgoAPI_Fuse(shape_1, shape_2));
}

inline std::unique_ptr<BRepAlgoAPI_Cut> BRepAlgoAPI_Cut_ctor(const TopoDS_Shape &shape_1, const TopoDS_Shape &shape_2) {
  return std::unique_ptr<BRepAlgoAPI_Cut>(new BRepAlgoAPI_Cut(shape_1, shape_2));
}

inline std::unique_ptr<BRepAlgoAPI_Common> BRepAlgoAPI_Common_ctor(const TopoDS_Shape &shape_1,
                                                                   const TopoDS_Shape &shape_2) {
  return std::unique_ptr<BRepAlgoAPI_Common>(new BRepAlgoAPI_Common(shape_1, shape_2));
}

inline std::unique_ptr<BRepAlgoAPI_Section> BRepAlgoAPI_Section_ctor(const TopoDS_Shape &shape_1,
                                                                     const TopoDS_Shape &shape_2) {
  return std::unique_ptr<BRepAlgoAPI_Section>(new BRepAlgoAPI_Section(shape_1, shape_2));
}

// Fillets
inline std::unique_ptr<BRepFilletAPI_MakeFillet> BRepFilletAPI_MakeFillet_ctor(const TopoDS_Shape &shape) {
  return std::unique_ptr<BRepFilletAPI_MakeFillet>(new BRepFilletAPI_MakeFillet(shape));
}

// Chamfers
inline std::unique_ptr<BRepFilletAPI_MakeChamfer> BRepFilletAPI_MakeChamfer_ctor(const TopoDS_Shape &shape) {
  return std::unique_ptr<BRepFilletAPI_MakeChamfer>(new BRepFilletAPI_MakeChamfer(shape));
}

// Solids
inline std::unique_ptr<BRepOffsetAPI_MakeThickSolid> BRepOffsetAPI_MakeThickSolid_ctor() {
  return std::unique_ptr<BRepOffsetAPI_MakeThickSolid>(new BRepOffsetAPI_MakeThickSolid());
}

inline void MakeThickSolidByJoin(BRepOffsetAPI_MakeThickSolid &make_thick_solid, const TopoDS_Shape &shape,
                                 const TopTools_ListOfShape &closing_faces, const Standard_Real offset,
                                 const Standard_Real tolerance) {
  make_thick_solid.MakeThickSolidByJoin(shape, closing_faces, offset, tolerance);
}

// Lofting
inline std::unique_ptr<BRepOffsetAPI_ThruSections> BRepOffsetAPI_ThruSections_ctor(bool is_solid) {
  return std::unique_ptr<BRepOffsetAPI_ThruSections>(new BRepOffsetAPI_ThruSections(is_solid));
}

// Geometric processing
inline const gp_Ax1 &gp_OX() { return gp::OX(); }
inline const gp_Ax1 &gp_OY() { return gp::OY(); }
inline const gp_Ax1 &gp_OZ() { return gp::OZ(); }

inline const gp_Dir &gp_DZ() { return gp::DZ(); }

inline std::unique_ptr<gp_Ax2> gp_Ax2_ctor(const gp_Pnt &origin, const gp_Dir &main_dir) {
  return std::unique_ptr<gp_Ax2>(new gp_Ax2(origin, main_dir));
}

inline std::unique_ptr<gp_Ax3> gp_Ax3_from_gp_Ax2(const gp_Ax2 &axis) {
  return std::unique_ptr<gp_Ax3>(new gp_Ax3(axis));
}

inline std::unique_ptr<gp_Dir2d> gp_Dir2d_ctor(double x, double y) {
  return std::unique_ptr<gp_Dir2d>(new gp_Dir2d(x, y));
}

inline std::unique_ptr<gp_Ax2d> gp_Ax2d_ctor(const gp_Pnt2d &point, const gp_Dir2d &dir) {
  return std::unique_ptr<gp_Ax2d>(new gp_Ax2d(point, dir));
}

// Shape stuff
inline const TopoDS_Vertex &TopoDS_cast_to_vertex(const TopoDS_Shape &shape) { return TopoDS::Vertex(shape); }

inline const TopoDS_Wire &TopoDS_cast_to_wire(const TopoDS_Shape &shape) { return TopoDS::Wire(shape); }

inline const TopoDS_Edge &TopoDS_cast_to_edge(const TopoDS_Shape &shape) { return TopoDS::Edge(shape); }

inline const TopoDS_Face &TopoDS_cast_to_face(const TopoDS_Shape &shape) { return TopoDS::Face(shape); }

inline std::unique_ptr<TopoDS_Shape> TopoDS_Shape_to_owned(const TopoDS_Shape &shape) {
  return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(shape));
}

inline std::unique_ptr<TopoDS_Vertex> TopoDS_Vertex_to_owned(const TopoDS_Vertex &vertex) {
  return std::unique_ptr<TopoDS_Vertex>(new TopoDS_Vertex(vertex));
}

inline std::unique_ptr<TopoDS_Wire> TopoDS_Wire_to_owned(const TopoDS_Wire &wire) {
  return std::unique_ptr<TopoDS_Wire>(new TopoDS_Wire(wire));
}

inline std::unique_ptr<TopoDS_Edge> TopoDS_Edge_to_owned(const TopoDS_Edge &edge) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(edge));
}

inline std::unique_ptr<TopoDS_Face> TopoDS_Face_to_owned(const TopoDS_Face &face) {
  return std::unique_ptr<TopoDS_Face>(new TopoDS_Face(face));
}

// Compound shapes
inline std::unique_ptr<TopoDS_Shape> TopoDS_Compound_as_shape(std::unique_ptr<TopoDS_Compound> compound) {
  return compound;
}

inline std::unique_ptr<TopoDS_Compound> TopoDS_Compound_ctor() {
  return std::unique_ptr<TopoDS_Compound>(new TopoDS_Compound());
}

inline std::unique_ptr<BRep_Builder> BRep_Builder_ctor() { return std::unique_ptr<BRep_Builder>(new BRep_Builder()); }

inline const TopoDS_Builder &BRep_Builder_upcast_to_topods_builder(const BRep_Builder &builder) { return builder; }

// Transforms
inline std::unique_ptr<gp_Trsf> new_transform() { return std::unique_ptr<gp_Trsf>(new gp_Trsf()); }

inline std::unique_ptr<BRepBuilderAPI_Transform>
BRepBuilderAPI_Transform_ctor(const TopoDS_Shape &shape, const gp_Trsf &transform, const Standard_Boolean copy) {
  return std::unique_ptr<BRepBuilderAPI_Transform>(new BRepBuilderAPI_Transform(shape, transform, copy));
}

// Topology Explorer
inline std::unique_ptr<TopExp_Explorer> TopExp_Explorer_ctor(const TopoDS_Shape &shape,
                                                             const TopAbs_ShapeEnum to_find) {
  return std::unique_ptr<TopExp_Explorer>(new TopExp_Explorer(shape, to_find));
}

inline std::unique_ptr<HandleGeomSurface> BRep_Tool_Surface(const TopoDS_Face &face) {
  return std::unique_ptr<HandleGeomSurface>(new opencascade::handle<Geom_Surface>(BRep_Tool::Surface(face)));
}

inline std::unique_ptr<HandleGeomCurve> BRep_Tool_Curve(const TopoDS_Edge &edge, Standard_Real &first,
                                                        Standard_Real &last) {
  return std::unique_ptr<HandleGeomCurve>(new opencascade::handle<Geom_Curve>(BRep_Tool::Curve(edge, first, last)));
}

inline std::unique_ptr<gp_Pnt> BRep_Tool_Pnt(const TopoDS_Vertex &vertex) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(BRep_Tool::Pnt(vertex)));
}

inline std::unique_ptr<TopoDS_Shape> ExplorerCurrentShape(const TopExp_Explorer &explorer) {
  return std::unique_ptr<TopoDS_Shape>(new TopoDS_Shape(explorer.Current()));
}

// Data export
inline std::unique_ptr<StlAPI_Writer> StlAPI_Writer_ctor() {
  return std::unique_ptr<StlAPI_Writer>(new StlAPI_Writer());
}

inline bool write_stl(StlAPI_Writer &writer, const TopoDS_Shape &theShape, rust::String theFileName) {
  return writer.Write(theShape, theFileName.c_str());
}

// Triangulation
inline std::unique_ptr<BRepMesh_IncrementalMesh> BRepMesh_IncrementalMesh_ctor(const TopoDS_Shape &shape,
                                                                               double deflection) {
  return std::unique_ptr<BRepMesh_IncrementalMesh>(new BRepMesh_IncrementalMesh(shape, deflection));
}
