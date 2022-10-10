#include <wrapper.hxx>

// Handle stuff

const HandleStandardType& DynamicType(const HandleGeomSurface& surface) {
  return surface->DynamicType();
}

rust::String type_name(const HandleStandardType& handle) {
  return std::string(handle->Name());
}

std::unique_ptr<HandleGeomCurve> new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(const HandleGeomTrimmedCurve& trimmed_curve) {
  return std::unique_ptr<HandleGeomCurve>(new opencascade::handle<Geom_Curve>(trimmed_curve));
}

std::unique_ptr<HandleGeomPlane> new_HandleGeomPlane_from_HandleGeomSurface(const HandleGeomSurface& surface) {
  HandleGeomPlane plane_handle = opencascade::handle<Geom_Plane>::DownCast(surface);
  return std::unique_ptr<HandleGeomPlane>(new opencascade::handle<Geom_Plane>(plane_handle));
}

// Collections
std::unique_ptr<TopTools_ListOfShape> new_list_of_shape() {
  return std::unique_ptr<TopTools_ListOfShape>(new TopTools_ListOfShape());
}

void shape_list_append_face(TopTools_ListOfShape& list, const TopoDS_Face& face) {
  list.Append(face);
}

// Geometry
const gp_Pnt& handle_geom_plane_location(const HandleGeomPlane& plane) {
  return plane->Location();
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

// Primitives
std::unique_ptr<BRepPrimAPI_MakePrism> BRepPrimAPI_MakePrism_ctor(const TopoDS_Shape& shape, const gp_Vec& vec, const Standard_Boolean copy, const Standard_Boolean canonize) {
  return std::unique_ptr<BRepPrimAPI_MakePrism>(new BRepPrimAPI_MakePrism(shape, vec, copy, canonize));
}

std::unique_ptr<BRepPrimAPI_MakeCylinder> BRepPrimAPI_MakeCylinder_ctor(const gp_Ax2& coord_system, const Standard_Real radius, const Standard_Real height) {
  return std::unique_ptr<BRepPrimAPI_MakeCylinder>(new BRepPrimAPI_MakeCylinder(coord_system, radius, height));
}

// Boolean operations
std::unique_ptr<BRepAlgoAPI_Fuse> BRepAlgoAPI_Fuse_ctor(const TopoDS_Shape& shape_1, const TopoDS_Shape& shape_2) {
  return std::unique_ptr<BRepAlgoAPI_Fuse>(new BRepAlgoAPI_Fuse(shape_1, shape_2));
}

// Fillets
std::unique_ptr<BRepFilletAPI_MakeFillet> BRepFilletAPI_MakeFillet_ctor(const TopoDS_Shape& shape) {
  return std::unique_ptr<BRepFilletAPI_MakeFillet>(new BRepFilletAPI_MakeFillet(shape));
}

// Geometric processing
const gp_Ax1& gp_OX() {
  return gp::OX();
}

const gp_Dir& gp_DZ() {
  return gp::DZ();
}

std::unique_ptr<gp_Ax2> gp_Ax2_ctor(const gp_Pnt& origin, const gp_Dir& main_dir) {
  return std::unique_ptr<gp_Ax2>(new gp_Ax2(origin, main_dir));
}

// Shape stuff
const TopoDS_Wire& TopoDS_cast_to_wire(const TopoDS_Shape& shape) {
  return TopoDS::Wire(shape);
}

const TopoDS_Edge& TopoDS_cast_to_edge(const TopoDS_Shape& shape) {
  return TopoDS::Edge(shape);
}

const TopoDS_Face& TopoDS_cast_to_face(const TopoDS_Shape& shape) {
  return TopoDS::Face(shape);
}

// Transforms
std::unique_ptr<gp_Trsf> new_transform() {
  return std::unique_ptr<gp_Trsf>(new gp_Trsf());
}

std::unique_ptr<BRepBuilderAPI_Transform> BRepBuilderAPI_Transform_ctor(const TopoDS_Shape& shape, const gp_Trsf& transform, const Standard_Boolean copy) {
  return std::unique_ptr<BRepBuilderAPI_Transform>(new BRepBuilderAPI_Transform(shape, transform, copy));
}

// Topology Explorer
std::unique_ptr<TopExp_Explorer> TopExp_Explorer_ctor(const TopoDS_Shape& shape, const TopAbs_ShapeEnum to_find) {
  return std::unique_ptr<TopExp_Explorer>(new TopExp_Explorer(shape, to_find));
}

std::unique_ptr<HandleGeomSurface> BRep_Tool_Surface(const TopoDS_Face& face) {
  return std::unique_ptr<HandleGeomSurface>(new opencascade::handle<Geom_Surface>(BRep_Tool::Surface(face)));
}

// Data export
std::unique_ptr<StlAPI_Writer> StlAPI_Writer_ctor() {
  return std::unique_ptr<StlAPI_Writer>(new StlAPI_Writer());
}

bool write_stl(StlAPI_Writer& writer, const TopoDS_Shape& theShape, rust::String theFileName) {
  return writer.Write(theShape, theFileName.c_str());
}

// Triangulation
std::unique_ptr<BRepMesh_IncrementalMesh> BRepMesh_IncrementalMesh_ctor(const TopoDS_Shape& shape, double deflection) {
  return std::unique_ptr<BRepMesh_IncrementalMesh>(new BRepMesh_IncrementalMesh(shape, deflection));
}
