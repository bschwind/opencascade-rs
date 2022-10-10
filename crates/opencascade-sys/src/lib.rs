#[cxx::bridge]
pub mod ffi {
    #[repr(u32)]
    pub enum TopAbs_ShapeEnum {
        TopAbs_COMPOUND,
        TopAbs_COMPSOLID,
        TopAbs_SOLID,
        TopAbs_SHELL,
        TopAbs_FACE,
        TopAbs_WIRE,
        TopAbs_EDGE,
        TopAbs_VERTEX,
        TopAbs_SHAPE,
    }

    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280

        // OCCT Includes
        include!("opencascade-sys/include/wrapper.hxx");

        // Handles
        type HandleStandardType;
        type HandleGeomCurve;
        type HandleGeomTrimmedCurve;
        type HandleGeomSurface;
        type HandleGeomPlane;

        pub fn DynamicType(surface: &HandleGeomSurface) -> &HandleStandardType;
        pub fn type_name(handle: &HandleStandardType) -> String;

        pub fn new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            trimmed_curve_handle: &HandleGeomTrimmedCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<HandleGeomPlane>;

        // Collections
        type TopTools_ListOfShape;
        pub fn new_list_of_shape() -> UniquePtr<TopTools_ListOfShape>;
        pub fn shape_list_append_face(list: Pin<&mut TopTools_ListOfShape>, face: &TopoDS_Face);

        // Geometry
        type Geom_TrimmedCurve;

        pub fn handle_geom_plane_location(plane: &HandleGeomPlane) -> &gp_Pnt;

        // Points
        type gp_Pnt;
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;
        pub fn X(self: &gp_Pnt) -> f64;
        pub fn Y(self: &gp_Pnt) -> f64;
        pub fn Z(self: &gp_Pnt) -> f64;
        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        type gp_Vec;
        pub fn new_vec(x: f64, y: f64, z: f64) -> UniquePtr<gp_Vec>;

        // Segments
        type GC_MakeSegment;
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;
        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Arcs
        type GC_MakeArcOfCircle;
        pub fn GC_MakeArcOfCircle_point_point_point(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
            p3: &gp_Pnt,
        ) -> UniquePtr<GC_MakeArcOfCircle>;
        pub fn GC_MakeArcOfCircle_Value(
            arc: &GC_MakeArcOfCircle,
        ) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Shapes
        type TopoDS_Shape;
        type TopoDS_Edge;
        type TopoDS_Wire;
        type TopoDS_Face;

        pub fn TopoDS_cast_to_wire(shape: &TopoDS_Shape) -> &TopoDS_Wire;
        pub fn TopoDS_cast_to_edge(shape: &TopoDS_Shape) -> &TopoDS_Edge;
        pub fn TopoDS_cast_to_face<'a>(shape: &'a TopoDS_Shape) -> UniquePtr<TopoDS_Face>;

        pub fn IsNull(self: &TopoDS_Shape) -> bool;
        pub fn IsEqual(self: &TopoDS_Shape, other: &TopoDS_Shape) -> bool;

        // BRepBuilder
        type BRepBuilderAPI_MakeEdge;
        type TopoDS_Vertex;
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;
        pub fn Vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDS_Vertex;
        pub fn Edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;

        type BRepBuilderAPI_MakeWire;
        pub fn BRepBuilderAPI_MakeWire_ctor() -> UniquePtr<BRepBuilderAPI_MakeWire>;
        pub fn BRepBuilderAPI_MakeWire_edge_edge_edge(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            edge_3: &TopoDS_Edge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;
        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Shape;
        pub fn Wire(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Wire;
        pub fn IsDone(self: &BRepBuilderAPI_MakeWire) -> bool;

        type BRepBuilderAPI_MakeFace;
        pub fn BRepBuilderAPI_MakeFace_wire(
            wire: &TopoDS_Wire,
            only_plane: bool,
        ) -> UniquePtr<BRepBuilderAPI_MakeFace>;
        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeFace>) -> &TopoDS_Shape;
        pub fn IsDone(self: &BRepBuilderAPI_MakeFace) -> bool;

        // Primitives
        type BRepPrimAPI_MakePrism;
        pub fn BRepPrimAPI_MakePrism_ctor(
            shape: &TopoDS_Shape,
            vec: &gp_Vec,
            copy: bool,
            canonize: bool,
        ) -> UniquePtr<BRepPrimAPI_MakePrism>;
        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakePrism>) -> &TopoDS_Shape;
        pub fn IsDone(self: &BRepPrimAPI_MakePrism) -> bool;

        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepBuilderAPI_MakeWire>, edge: &TopoDS_Edge);

        #[rust_name = "add_wire"]
        pub fn Add(self: Pin<&mut BRepBuilderAPI_MakeWire>, wire: &TopoDS_Wire);

        type BRepPrimAPI_MakeCylinder;
        pub fn BRepPrimAPI_MakeCylinder_ctor(
            coord_system: &gp_Ax2,
            radius: f64,
            height: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeCylinder>;
        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeCylinder>) -> &TopoDS_Shape;

        // Fillets
        type BRepFilletAPI_MakeFillet;
        pub fn BRepFilletAPI_MakeFillet_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeFillet>;
        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepFilletAPI_MakeFillet>, radius: f64, edge: &TopoDS_Edge);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeFillet>) -> &TopoDS_Shape;

        // Boolean Operations
        type BRepAlgoAPI_Fuse;
        pub fn BRepAlgoAPI_Fuse_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;

        // Geometric processor
        type gp_Ax1;
        type gp_Ax2;
        type gp_Dir;
        pub fn gp_OX() -> &'static gp_Ax1;
        pub fn gp_DZ() -> &'static gp_Dir;

        pub fn gp_Ax2_ctor(origin: &gp_Pnt, main_dir: &gp_Dir) -> UniquePtr<gp_Ax2>;

        // Transforms
        type gp_Trsf;
        pub fn new_transform() -> UniquePtr<gp_Trsf>;

        #[rust_name = "set_mirror_axis"]
        pub fn SetMirror(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1);

        type BRepBuilderAPI_Transform;
        pub fn BRepBuilderAPI_Transform_ctor(
            shape: &TopoDS_Shape,
            transform: &gp_Trsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPI_Transform>;
        pub fn Shape(self: Pin<&mut BRepBuilderAPI_Transform>) -> &TopoDS_Shape;

        // Topology Explorer
        type TopExp_Explorer;
        type TopAbs_ShapeEnum;
        pub fn TopExp_Explorer_ctor(
            shape: &TopoDS_Shape,
            to_find: TopAbs_ShapeEnum,
        ) -> UniquePtr<TopExp_Explorer>;
        pub fn More(self: &TopExp_Explorer) -> bool;
        pub fn Next(self: Pin<&mut TopExp_Explorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExp_Explorer) -> UniquePtr<TopoDS_Shape>;
        pub fn Current(self: &TopExp_Explorer) -> &TopoDS_Shape;

        pub fn BRep_Tool_Surface(face: &TopoDS_Face) -> UniquePtr<HandleGeomSurface>;

        // Data export
        type StlAPI_Writer;
        pub fn StlAPI_Writer_ctor() -> UniquePtr<StlAPI_Writer>;
        // pub fn Write(self: Pin<&mut StlAPI_Writer>, shape: &TopoDS_Shape, filename: &c_char) -> bool;
        pub fn write_stl(
            writer: Pin<&mut StlAPI_Writer>,
            shape: &TopoDS_Shape,
            filename: String,
        ) -> bool;

        // Triangulation
        type BRepMesh_IncrementalMesh;
        pub fn BRepMesh_IncrementalMesh_ctor(
            shape: &TopoDS_Shape,
            deflection: f64,
        ) -> UniquePtr<BRepMesh_IncrementalMesh>;
        pub fn Shape(self: &BRepMesh_IncrementalMesh) -> &TopoDS_Shape;
    }
}
