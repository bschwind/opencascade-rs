#include "rust/cxx.h"
#include <BOPAlgo_GlueEnum.hxx>
#include <BRepAdaptor_Curve.hxx>
#include <BRepAlgoAPI_Common.hxx>
#include <BRepAlgoAPI_Cut.hxx>
#include <BRepAlgoAPI_Fuse.hxx>
#include <BRepAlgoAPI_Section.hxx>
#include <BRepBndLib.hxx>
#include <BRepBuilderAPI_GTransform.hxx>
#include <BRepBuilderAPI_MakeEdge.hxx>
#include <BRepBuilderAPI_MakeFace.hxx>
#include <BRepBuilderAPI_MakeShapeOnMesh.hxx>
#include <BRepBuilderAPI_MakeSolid.hxx>
#include <BRepBuilderAPI_MakeVertex.hxx>
#include <BRepBuilderAPI_MakeWire.hxx>
#include <BRepBuilderAPI_Transform.hxx>
#include <BRepFilletAPI_MakeChamfer.hxx>
#include <BRepFilletAPI_MakeFillet.hxx>
#include <BRepFilletAPI_MakeFillet2d.hxx>
#include <BRepGProp.hxx>
#include <BRepGProp_Face.hxx>
#include <BRepIntCurveSurface_Inter.hxx>
#include <BRepLib.hxx>
#include <BRepLib_ToolTriangulatedShape.hxx>
#include <BRepMesh_IncrementalMesh.hxx>
#include <BRepOffsetAPI_MakeOffset.hxx>
#include <BRepOffsetAPI_MakePipe.hxx>
#include <BRepOffsetAPI_MakePipeShell.hxx>
#include <BRepOffsetAPI_MakeThickSolid.hxx>
#include <BRepOffsetAPI_ThruSections.hxx>
#include <BRepPrimAPI_MakeBox.hxx>
#include <BRepPrimAPI_MakeCone.hxx>
#include <BRepPrimAPI_MakeCylinder.hxx>
#include <BRepPrimAPI_MakePrism.hxx>
#include <BRepPrimAPI_MakeRevol.hxx>
#include <BRepPrimAPI_MakeSphere.hxx>
#include <BRepPrimAPI_MakeTorus.hxx>
#include <BRepTools.hxx>
#include <BRep_Builder.hxx>
#include <BinTools.hxx>
#include <GCE2d_MakeSegment.hxx>
#include <GCPnts_TangentialDeflection.hxx>
#include <GC_MakeArcOfCircle.hxx>
#include <GC_MakeSegment.hxx>
#include <GProp_GProps.hxx>
#include <Geom2d_Ellipse.hxx>
#include <Geom2d_TrimmedCurve.hxx>
#include <GeomAPI_Interpolate.hxx>
#include <GeomAPI_ProjectPointOnSurf.hxx>
#include <Geom_BezierCurve.hxx>
#include <Geom_BezierSurface.hxx>
#include <Geom_CylindricalSurface.hxx>
#include <IFSelect_ReturnStatus.hxx>
#include <Law_Function.hxx>
#include <Law_Interpol.hxx>
#include <NCollection_Array1.hxx>
#include <NCollection_Array2.hxx>
#include <Poly_Connect.hxx>
#include <ShapeAnalysis_FreeBounds.hxx>
#include <ShapeUpgrade_UnifySameDomain.hxx>
#include <Standard_Type.hxx>
#include <StlAPI_Writer.hxx>
#include <TColgp_Array1OfDir.hxx>
#include <TColgp_HArray1OfPnt.hxx>
#include <TopAbs_ShapeEnum.hxx>
#include <TopTools_HSequenceOfShape.hxx>
#include <TopoDS.hxx>
#include <TopoDS_Edge.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>
#include <gp.hxx>
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Circ.hxx>
#include <gp_Lin.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>

// Generic template constructor
template <typename T, typename... Args> std::unique_ptr<T> construct_unique(Args... args) {
  return std::unique_ptr<T>(new T(args...));
}

// Generic List
template <typename T> std::unique_ptr<std::vector<T>> list_to_vector(const NCollection_List<T> &list) {
  return std::unique_ptr<std::vector<T>>(new std::vector<T>(list.begin(), list.end()));
}

// Handles
typedef opencascade::handle<Standard_Type> HandleStandardType;

inline std::unique_ptr<Handle_TColgp_HArray1OfPnt>
new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(std::unique_ptr<TColgp_HArray1OfPnt> array) {
  return std::unique_ptr<Handle_TColgp_HArray1OfPnt>(new Handle_TColgp_HArray1OfPnt(array.release()));
}

// Handle stuff
template <typename T> const T &handle_try_deref(const opencascade::handle<T> &handle) {
  if (handle.IsNull()) {
    throw std::runtime_error("null handle dereference");
  }
  return *handle;
}

inline const HandleStandardType &DynamicType(const Handle_Geom_Surface &surface) { return surface->DynamicType(); }

inline rust::String type_name(const HandleStandardType &handle) { return std::string(handle->Name()); }

inline std::unique_ptr<gp_Pnt> HandleGeomCurve_Value(const Handle_Geom_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve->Value(U)));
}

inline std::unique_ptr<gp_Pnt> BRepAdaptor_Curve_value(const BRepAdaptor_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve.Value(U)));
}

// BRepLib
inline bool BRepLibBuildCurves3d(const TopoDS_Shape &shape) { return BRepLib::BuildCurves3d(shape); }

inline void MakeThickSolidByJoin(BRepOffsetAPI_MakeThickSolid &make_thick_solid, const TopoDS_Shape &shape,
                                 const TopTools_ListOfShape &closing_faces, const Standard_Real offset,
                                 const Standard_Real tolerance) {
  make_thick_solid.MakeThickSolidByJoin(shape, closing_faces, offset, tolerance);
}

// Shape stuff
inline const TopoDS_Vertex &TopoDS_cast_to_vertex(const TopoDS_Shape &shape) { return TopoDS::Vertex(shape); }
inline const TopoDS_Edge &TopoDS_cast_to_edge(const TopoDS_Shape &shape) { return TopoDS::Edge(shape); }
inline const TopoDS_Wire &TopoDS_cast_to_wire(const TopoDS_Shape &shape) { return TopoDS::Wire(shape); }
inline const TopoDS_Face &TopoDS_cast_to_face(const TopoDS_Shape &shape) { return TopoDS::Face(shape); }
inline const TopoDS_Shell &TopoDS_cast_to_shell(const TopoDS_Shape &shape) { return TopoDS::Shell(shape); }
inline const TopoDS_Solid &TopoDS_cast_to_solid(const TopoDS_Shape &shape) { return TopoDS::Solid(shape); }
inline const TopoDS_Compound &TopoDS_cast_to_compound(const TopoDS_Shape &shape) { return TopoDS::Compound(shape); }

inline const TopoDS_Shape &cast_vertex_to_shape(const TopoDS_Vertex &vertex) { return vertex; }
inline const TopoDS_Shape &cast_edge_to_shape(const TopoDS_Edge &edge) { return edge; }
inline const TopoDS_Shape &cast_wire_to_shape(const TopoDS_Wire &wire) { return wire; }
inline const TopoDS_Shape &cast_face_to_shape(const TopoDS_Face &face) { return face; }
inline const TopoDS_Shape &cast_shell_to_shape(const TopoDS_Shell &shell) { return shell; }
inline const TopoDS_Shape &cast_solid_to_shape(const TopoDS_Solid &solid) { return solid; }
inline const TopoDS_Shape &cast_compound_to_shape(const TopoDS_Compound &compound) { return compound; }

// Compound shapes
inline std::unique_ptr<TopoDS_Shape> TopoDS_Compound_as_shape(std::unique_ptr<TopoDS_Compound> compound) {
  return compound;
}

inline std::unique_ptr<TopoDS_Shape> TopoDS_Shell_as_shape(std::unique_ptr<TopoDS_Shell> shell) { return shell; }

inline const TopoDS_Builder &BRep_Builder_upcast_to_topods_builder(const BRep_Builder &builder) { return builder; }

// Transforms
inline std::unique_ptr<Handle_Geom_Surface> BRep_Tool_Surface(const TopoDS_Face &face) {
  return std::unique_ptr<Handle_Geom_Surface>(new opencascade::handle<Geom_Surface>(BRep_Tool::Surface(face)));
}

inline std::unique_ptr<Handle_Geom_Curve> BRep_Tool_Curve(const TopoDS_Edge &edge, Standard_Real &first,
                                                          Standard_Real &last) {
  return std::unique_ptr<Handle_Geom_Curve>(new opencascade::handle<Geom_Curve>(BRep_Tool::Curve(edge, first, last)));
}

inline std::unique_ptr<gp_Pnt> BRep_Tool_Pnt(const TopoDS_Vertex &vertex) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(BRep_Tool::Pnt(vertex)));
}

inline std::unique_ptr<TopoDS_Face> BRepIntCurveSurface_Inter_face(const BRepIntCurveSurface_Inter &intersector) {
  return std::unique_ptr<TopoDS_Face>(new TopoDS_Face(intersector.Face()));
}

inline std::unique_ptr<gp_Pnt> BRepIntCurveSurface_Inter_point(const BRepIntCurveSurface_Inter &intersector) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(intersector.Pnt()));
}

// Fillets
inline std::unique_ptr<TopoDS_Edge> BRepFilletAPI_MakeFillet2d_add_fillet(BRepFilletAPI_MakeFillet2d &make_fillet,
                                                                          const TopoDS_Vertex &vertex,
                                                                          Standard_Real radius) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddFillet(vertex, radius)));
}

// Chamfers
inline std::unique_ptr<TopoDS_Edge>
BRepFilletAPI_MakeFillet2d_add_chamfer(BRepFilletAPI_MakeFillet2d &make_fillet, const TopoDS_Edge &edge1,
                                       const TopoDS_Edge &edge2, const Standard_Real dist1, const Standard_Real dist2) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddChamfer(edge1, edge2, dist1, dist2)));
}

inline std::unique_ptr<TopoDS_Edge>
BRepFilletAPI_MakeFillet2d_add_chamfer_angle(BRepFilletAPI_MakeFillet2d &make_fillet, const TopoDS_Edge &edge,
                                             const TopoDS_Vertex &vertex, const Standard_Real dist,
                                             const Standard_Real angle) {
  return std::unique_ptr<TopoDS_Edge>(new TopoDS_Edge(make_fillet.AddChamfer(edge, vertex, dist, angle)));
}

inline std::unique_ptr<Handle_Poly_Triangulation> BRep_Tool_Triangulation(const TopoDS_Face &face,
                                                                          TopLoc_Location &location) {
  return std::unique_ptr<Handle_Poly_Triangulation>(
      new opencascade::handle<Poly_Triangulation>(BRep_Tool::Triangulation(face, location)));
}

inline void compute_normals(const TopoDS_Face &face, const Handle(Poly_Triangulation) & triangulation) {
  BRepLib_ToolTriangulatedShape::ComputeNormals(face, triangulation);
}

// BRep Algo API
inline std::unique_ptr<BRepAlgoAPI_BuilderAlgo>
cast_section_to_builderalgo(std::unique_ptr<BRepAlgoAPI_Section> section) {
  return section;
}
// namespace BRepAlgoAPI
