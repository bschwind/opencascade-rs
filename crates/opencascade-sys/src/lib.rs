#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280

        // OCCT Includes
        include!("opencascade-sys/OCCT/src/gp/gp_Pnt.hxx");
        include!("opencascade-sys/OCCT/src/gp/gp.hxx");
        include!("opencascade-sys/OCCT/src/GC/GC_MakeSegment.hxx");
        include!("opencascade-sys/OCCT/src/GC/GC_MakeArcOfCircle.hxx");
        include!("opencascade-sys/OCCT/src/Geom/Geom_TrimmedCurve.hxx");
        include!("opencascade-sys/OCCT/src/Standard/Standard_Handle.hxx");
        include!("opencascade-sys/OCCT/src/TopoDS/TopoDS_Edge.hxx");
        include!("opencascade-sys/OCCT/src/BRepBuilderAPI/BRepBuilderAPI_MakeEdge.hxx");
        include!("opencascade-sys/OCCT/src/BRepBuilderAPI/BRepBuilderAPI_MakeWire.hxx");

        include!("opencascade-sys/include/wrapper.hxx");

        // Handles
        type HandleGeomCurve;
        type HandleGeomTrimmedCurve;

        pub fn new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            trimmed_curve_handle: &HandleGeomTrimmedCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        // Geometry
        type Geom_TrimmedCurve;

        // Points
        type gp_Pnt;
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;
        pub fn Y(self: &gp_Pnt) -> f64;
        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        // Segments
        type GC_MakeSegment;
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;
        pub fn new_segment(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<Geom_TrimmedCurve>;
        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Arcs
        type GC_MakeArcOfCircle;
        pub fn GC_MakeArcOfCircle_point_point_point(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
            p3: &gp_Pnt,
        ) -> UniquePtr<GC_MakeArcOfCircle>;
        pub fn new_arc_of_circle(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
            p3: &gp_Pnt,
        ) -> UniquePtr<Geom_TrimmedCurve>;
        pub fn GC_MakeArcOfCircle_Value(
            arc: &GC_MakeArcOfCircle,
        ) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Shapes
        type TopoDS_Shape;
        type TopoDS_Edge;
        pub fn make_edge(geom_curve: &Geom_TrimmedCurve) -> UniquePtr<TopoDS_Edge>;

        // BRepBuilder
        type BRepBuilderAPI_MakeEdge;
        type TopoDS_Vertex;
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;
        pub fn Vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDS_Vertex;
        pub fn Edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;

        type BRepBuilderAPI_MakeWire;
        pub fn BRepBuilderAPI_MakeWire_edge_edge_edge(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            edge_3: &TopoDS_Edge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;

        // Geometric processor
        type gp_Ax1;
        pub fn gp_OX() -> &'static gp_Ax1;
    }
}
