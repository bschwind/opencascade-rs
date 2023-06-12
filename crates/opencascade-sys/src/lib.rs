#[cxx::bridge]
pub mod ffi {
    #[repr(u32)]
    #[derive(Debug)]
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

    #[repr(u32)]
    pub enum TopAbs_Orientation {
        TopAbs_FORWARD,
        TopAbs_REVERSED,
        TopAbs_INTERNAL,
        TopAbs_EXTERNAL,
    }

    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280

        // OCCT Includes
        include!("opencascade-sys/include/wrapper.hxx");

        // Runtime
        type Message_ProgressRange;

        #[cxx_name = "construct_unique"]
        pub fn Message_ProgressRange_ctor() -> UniquePtr<Message_ProgressRange>;

        // Handles
        type HandleStandardType;
        type HandleGeomCurve;
        type HandleGeomTrimmedCurve;
        type HandleGeomSurface;
        type HandleGeomPlane;
        type HandleGeom2d_Curve;
        type HandleGeom2d_Ellipse;
        type HandleGeom2d_TrimmedCurve;
        type HandleGeom_CylindricalSurface;

        pub fn DynamicType(surface: &HandleGeomSurface) -> &HandleStandardType;
        pub fn type_name(handle: &HandleStandardType) -> String;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            trimmed_curve_handle: &HandleGeomTrimmedCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<HandleGeomPlane>;

        pub fn IsNull(self: &HandleStandardType) -> bool;
        pub fn IsNull(self: &HandleGeomCurve) -> bool;
        pub fn IsNull(self: &HandleGeomTrimmedCurve) -> bool;
        pub fn IsNull(self: &HandleGeomSurface) -> bool;
        pub fn IsNull(self: &HandleGeomPlane) -> bool;
        pub fn IsNull(self: &HandleGeom2d_Curve) -> bool;
        pub fn IsNull(self: &HandleGeom2d_Ellipse) -> bool;
        pub fn IsNull(self: &HandleGeom2d_TrimmedCurve) -> bool;
        pub fn IsNull(self: &HandleGeom_CylindricalSurface) -> bool;

        pub fn HandleGeomCurve_Value(curve: &HandleGeomCurve, u: f64) -> UniquePtr<gp_Pnt>;

        // Collections
        type TopTools_ListOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_list_of_shape() -> UniquePtr<TopTools_ListOfShape>;
        pub fn shape_list_append_face(list: Pin<&mut TopTools_ListOfShape>, face: &TopoDS_Face);
        pub fn Size(self: &TopTools_ListOfShape) -> i32;

        #[cxx_name = "list_to_vector"]
        pub fn shape_list_to_vector(
            list: &TopTools_ListOfShape,
        ) -> UniquePtr<CxxVector<TopoDS_Shape>>;

        type TopTools_IndexedMapOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_indexed_map_of_shape() -> UniquePtr<TopTools_IndexedMapOfShape>;
        pub fn Extent(self: &TopTools_IndexedMapOfShape) -> i32;
        pub fn FindKey(self: &TopTools_IndexedMapOfShape, index: i32) -> &TopoDS_Shape;

        pub fn map_shapes(
            shape: &TopoDS_Shape,
            shape_type: TopAbs_ShapeEnum,
            shape_map: Pin<&mut TopTools_IndexedMapOfShape>,
        );

        type TColgp_Array1OfDir;
        #[cxx_name = "construct_unique"]
        pub fn TColgp_Array1OfDir_ctor(
            lower_bound: i32,
            upper_bound: i32,
        ) -> UniquePtr<TColgp_Array1OfDir>;
        pub fn Length(self: &TColgp_Array1OfDir) -> i32;
        pub fn TColgp_Array1OfDir_Value(
            array: &TColgp_Array1OfDir,
            index: i32,
        ) -> UniquePtr<gp_Dir>;

        // Geometry
        type Geom_TrimmedCurve;
        type Geom_CylindricalSurface;
        type Geom2d_Ellipse;
        type Geom2d_Curve;
        type Geom2d_TrimmedCurve;

        pub fn handle_geom_plane_location(plane: &HandleGeomPlane) -> &gp_Pnt;

        pub fn Geom_CylindricalSurface_ctor(
            axis: &gp_Ax3,
            radius: f64,
        ) -> UniquePtr<HandleGeom_CylindricalSurface>;
        pub fn cylinder_to_surface(
            cylinder_handle: &HandleGeom_CylindricalSurface,
        ) -> UniquePtr<HandleGeomSurface>;

        pub fn Geom2d_Ellipse_ctor(
            axis: &gp_Ax2d,
            major_radius: f64,
            minor_radius: f64,
        ) -> UniquePtr<HandleGeom2d_Ellipse>;
        pub fn ellipse_to_HandleGeom2d_Curve(
            ellipse: &HandleGeom2d_Ellipse,
        ) -> UniquePtr<HandleGeom2d_Curve>;
        pub fn Geom2d_TrimmedCurve_ctor(
            curve_handle: &HandleGeom2d_Curve,
            u1: f64,
            u2: f64,
        ) -> UniquePtr<HandleGeom2d_TrimmedCurve>;
        pub fn HandleGeom2d_TrimmedCurve_to_curve(
            trimmed_curve: &HandleGeom2d_TrimmedCurve,
        ) -> UniquePtr<HandleGeom2d_Curve>;

        pub fn ellipse_value(ellipse: &HandleGeom2d_Ellipse, u: f64) -> UniquePtr<gp_Pnt2d>;

        // Points
        type gp_Pnt;
        type gp_Pnt2d;

        #[cxx_name = "construct_unique"]
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;

        pub fn X(self: &gp_Pnt) -> f64;
        pub fn Y(self: &gp_Pnt) -> f64;
        pub fn Z(self: &gp_Pnt) -> f64;
        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        #[cxx_name = "construct_unique"]
        pub fn new_point_2d(x: f64, y: f64) -> UniquePtr<gp_Pnt2d>;

        pub fn X(self: &gp_Pnt2d) -> f64;
        pub fn Y(self: &gp_Pnt2d) -> f64;
        pub fn Distance(self: &gp_Pnt2d, other: &gp_Pnt2d) -> f64;

        type gp_Vec;

        #[cxx_name = "construct_unique"]
        pub fn new_vec(x: f64, y: f64, z: f64) -> UniquePtr<gp_Vec>;

        pub fn X(self: &gp_Vec) -> f64;
        pub fn Y(self: &gp_Vec) -> f64;
        pub fn Z(self: &gp_Vec) -> f64;

        // Segments
        type GC_MakeSegment;
        type GCE2d_MakeSegment;

        #[cxx_name = "construct_unique"]
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;

        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;
        pub fn GCE2d_MakeSegment_point_point(
            p1: &gp_Pnt2d,
            p2: &gp_Pnt2d,
        ) -> UniquePtr<HandleGeom2d_TrimmedCurve>;

        // Arcs
        type GC_MakeArcOfCircle;

        #[cxx_name = "construct_unique"]
        pub fn GC_MakeArcOfCircle_point_point_point(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
            p3: &gp_Pnt,
        ) -> UniquePtr<GC_MakeArcOfCircle>;

        pub fn GC_MakeArcOfCircle_Value(
            arc: &GC_MakeArcOfCircle,
        ) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Shapes
        type TopoDS_Vertex;
        type TopoDS_Edge;
        type TopoDS_Wire;
        type TopoDS_Face;
        type TopoDS_Shell;
        type TopoDS_Solid;
        type TopoDS_Shape;

        pub fn cast_wire_to_shape(wire: &TopoDS_Wire) -> &TopoDS_Shape;
        pub fn cast_face_to_shape(wire: &TopoDS_Face) -> &TopoDS_Shape;
        pub fn cast_solid_to_shape(wire: &TopoDS_Solid) -> &TopoDS_Shape;
        pub fn cast_compound_to_shape(wire: &TopoDS_Compound) -> &TopoDS_Shape;

        pub fn TopoDS_cast_to_vertex(shape: &TopoDS_Shape) -> &TopoDS_Vertex;
        pub fn TopoDS_cast_to_wire(shape: &TopoDS_Shape) -> &TopoDS_Wire;
        pub fn TopoDS_cast_to_edge(shape: &TopoDS_Shape) -> &TopoDS_Edge;
        pub fn TopoDS_cast_to_face(shape: &TopoDS_Shape) -> &TopoDS_Face;
        pub fn TopoDS_cast_to_solid(shape: &TopoDS_Shape) -> &TopoDS_Solid;
        pub fn TopoDS_cast_to_compound(shape: &TopoDS_Shape) -> &TopoDS_Compound;

        #[cxx_name = "Move"]
        pub fn translate(
            self: Pin<&mut TopoDS_Shape>,
            position: &TopLoc_Location,
            raise_exception: bool,
        );

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Vertex_to_owned(shape: &TopoDS_Vertex) -> UniquePtr<TopoDS_Vertex>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Edge_to_owned(shape: &TopoDS_Edge) -> UniquePtr<TopoDS_Edge>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Wire_to_owned(shape: &TopoDS_Wire) -> UniquePtr<TopoDS_Wire>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Face_to_owned(shape: &TopoDS_Face) -> UniquePtr<TopoDS_Face>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Shell_to_owned(shape: &TopoDS_Shell) -> UniquePtr<TopoDS_Shell>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Solid_to_owned(shape: &TopoDS_Solid) -> UniquePtr<TopoDS_Solid>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Compound_to_owned(shape: &TopoDS_Compound) -> UniquePtr<TopoDS_Compound>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Shape_to_owned(shape: &TopoDS_Shape) -> UniquePtr<TopoDS_Shape>;

        pub fn IsNull(self: &TopoDS_Shape) -> bool;
        pub fn IsEqual(self: &TopoDS_Shape, other: &TopoDS_Shape) -> bool;
        pub fn ShapeType(self: &TopoDS_Shape) -> TopAbs_ShapeEnum;

        type TopAbs_Orientation;
        pub fn Orientation(self: &TopoDS_Shape) -> TopAbs_Orientation;
        pub fn Orientation(self: &TopoDS_Face) -> TopAbs_Orientation;

        // Compound Shapes
        type TopoDS_Compound;
        pub fn TopoDS_Compound_as_shape(
            compound: UniquePtr<TopoDS_Compound>,
        ) -> UniquePtr<TopoDS_Shape>;

        type BRep_Builder;
        type TopoDS_Builder;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Compound_ctor() -> UniquePtr<TopoDS_Compound>;

        #[cxx_name = "construct_unique"]
        pub fn BRep_Builder_ctor() -> UniquePtr<BRep_Builder>;

        pub fn BRep_Builder_upcast_to_topods_builder(builder: &BRep_Builder) -> &TopoDS_Builder;
        pub fn MakeCompound(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Compound>);
        pub fn Add(self: &TopoDS_Builder, shape: Pin<&mut TopoDS_Shape>, compound: &TopoDS_Shape);

        // BRepBuilder
        type BRepBuilderAPI_MakeVertex;
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeVertex_gp_Pnt(
            point: &gp_Pnt,
        ) -> UniquePtr<BRepBuilderAPI_MakeVertex>;

        pub fn Vertex(self: Pin<&mut BRepBuilderAPI_MakeVertex>) -> &TopoDS_Vertex;

        type BRepBuilderAPI_MakeEdge;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_CurveSurface2d(
            curve_handle: &HandleGeom2d_Curve,
            surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        pub fn Vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDS_Vertex;
        pub fn Edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_MakeEdge>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_MakeEdge) -> bool;

        type BRepBuilderAPI_MakeWire;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_ctor() -> UniquePtr<BRepBuilderAPI_MakeWire>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_edge_edge(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_edge_edge_edge(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            edge_3: &TopoDS_Edge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;

        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Shape;
        pub fn Wire(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Wire;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_MakeWire>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_MakeWire) -> bool;

        type BRepBuilderAPI_MakeFace;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeFace_wire(
            wire: &TopoDS_Wire,
            only_plane: bool,
        ) -> UniquePtr<BRepBuilderAPI_MakeFace>;

        pub fn Face(self: &BRepBuilderAPI_MakeFace) -> &TopoDS_Face;
        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeFace>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_MakeFace>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_MakeFace) -> bool;

        // BRepAdaptor
        type BRepAdaptor_Curve;

        #[cxx_name = "construct_unique"]
        pub fn BRepAdaptor_Curve_ctor(edge: &TopoDS_Edge) -> UniquePtr<BRepAdaptor_Curve>;
        pub fn FirstParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn LastParameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn BRepAdaptor_Curve_value(curve: &BRepAdaptor_Curve, u: f64) -> UniquePtr<gp_Pnt>;

        // Primitives
        type BRepPrimAPI_MakePrism;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakePrism_ctor(
            shape: &TopoDS_Shape,
            vec: &gp_Vec,
            copy: bool,
            canonize: bool,
        ) -> UniquePtr<BRepPrimAPI_MakePrism>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakePrism>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakePrism>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakePrism) -> bool;

        type BRepPrimAPI_MakeRevol;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeRevol_ctor(
            shape: &TopoDS_Shape,
            axis: &gp_Ax1,
            angle: f64,
            copy: bool,
        ) -> UniquePtr<BRepPrimAPI_MakeRevol>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeRevol>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeRevol>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeRevol) -> bool;

        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepBuilderAPI_MakeWire>, edge: &TopoDS_Edge);

        #[rust_name = "add_wire"]
        pub fn Add(self: Pin<&mut BRepBuilderAPI_MakeWire>, wire: &TopoDS_Wire);

        type BRepPrimAPI_MakeCylinder;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeCylinder_ctor(
            coord_system: &gp_Ax2,
            radius: f64,
            height: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeCylinder>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeCylinder>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeCylinder>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeCylinder) -> bool;

        type BRepPrimAPI_MakeBox;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeBox_ctor(
            point: &gp_Pnt,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeBox>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeBox>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeBox) -> bool;

        type BRepPrimAPI_MakeSphere;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeSphere_ctor(r: f64) -> UniquePtr<BRepPrimAPI_MakeSphere>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeSphere>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeSphere>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeSphere) -> bool;

        // BRepLib
        pub fn BRepLibBuildCurves3d(shape: &TopoDS_Shape) -> bool;

        // Fillets
        type BRepFilletAPI_MakeFillet;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeFillet_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeFillet>;

        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepFilletAPI_MakeFillet>, radius: f64, edge: &TopoDS_Edge);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeFillet>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepFilletAPI_MakeFillet>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepFilletAPI_MakeFillet) -> bool;

        type BRepFilletAPI_MakeFillet2d;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeFillet2d_ctor(
            face: &TopoDS_Face,
        ) -> UniquePtr<BRepFilletAPI_MakeFillet2d>;

        pub fn BRepFilletAPI_MakeFillet2d_add_fillet(
            make_fillet: Pin<&mut BRepFilletAPI_MakeFillet2d>,
            vertex: &TopoDS_Vertex,
            radius: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        pub fn Build(self: Pin<&mut BRepFilletAPI_MakeFillet2d>, progress: &Message_ProgressRange);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeFillet2d>) -> &TopoDS_Shape;
        pub fn IsDone(self: &BRepFilletAPI_MakeFillet2d) -> bool;

        // Chamfers
        type BRepFilletAPI_MakeChamfer;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeChamfer_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeChamfer>;

        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepFilletAPI_MakeChamfer>, distance: f64, edge: &TopoDS_Edge);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeChamfer>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepFilletAPI_MakeChamfer>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepFilletAPI_MakeChamfer) -> bool;

        // Solids
        type BRepOffsetAPI_MakeThickSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeThickSolid_ctor() -> UniquePtr<BRepOffsetAPI_MakeThickSolid>;

        pub fn MakeThickSolidByJoin(
            make_thick_solid: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            shape: &TopoDS_Shape,
            closing_faces: &TopTools_ListOfShape,
            offset: f64,
            tolerance: f64,
        );
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakeThickSolid>) -> &TopoDS_Shape;
        pub fn Build(
            self: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            progress: &Message_ProgressRange,
        );
        pub fn IsDone(self: &BRepOffsetAPI_MakeThickSolid) -> bool;

        // Lofting
        type BRepOffsetAPI_ThruSections;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_ThruSections_ctor(
            is_solid: bool,
        ) -> UniquePtr<BRepOffsetAPI_ThruSections>;

        pub fn AddWire(self: Pin<&mut BRepOffsetAPI_ThruSections>, wire: &TopoDS_Wire);
        pub fn CheckCompatibility(self: Pin<&mut BRepOffsetAPI_ThruSections>, check: bool);
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_ThruSections>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepOffsetAPI_ThruSections>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepOffsetAPI_ThruSections) -> bool;

        // Boolean Operations
        type BRepAlgoAPI_Fuse;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Fuse_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Fuse>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Fuse) -> bool;

        type BRepAlgoAPI_Cut;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Cut_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Cut>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Cut>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Cut) -> bool;
        pub fn Generated<'a>(
            self: Pin<&'a mut BRepAlgoAPI_Cut>,
            shape: &'a TopoDS_Shape,
        ) -> &'a TopTools_ListOfShape;
        pub fn SectionEdges<'a>(self: Pin<&'a mut BRepAlgoAPI_Cut>) -> &'a TopTools_ListOfShape;

        type BRepAlgoAPI_Common;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Common_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Common>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Common>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Common) -> bool;

        type BRepAlgoAPI_Section;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Section_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Section>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Section>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Section) -> bool;

        // Geometric processor
        type gp_Ax1;
        type gp_Ax2;
        type gp_Ax3;
        type gp_Dir;
        type gp_Dir2d;
        type gp_Ax2d;
        pub fn gp_OX() -> &'static gp_Ax1;
        pub fn gp_OY() -> &'static gp_Ax1;
        pub fn gp_OZ() -> &'static gp_Ax1;
        pub fn gp_DZ() -> &'static gp_Dir;

        pub fn X(self: &gp_Dir) -> f64;
        pub fn Y(self: &gp_Dir) -> f64;
        pub fn Z(self: &gp_Dir) -> f64;

        #[cxx_name = "construct_unique"]
        pub fn gp_Ax1_ctor(origin: &gp_Pnt, main_dir: &gp_Dir) -> UniquePtr<gp_Ax1>;

        #[cxx_name = "construct_unique"]
        pub fn gp_Ax2_ctor(origin: &gp_Pnt, main_dir: &gp_Dir) -> UniquePtr<gp_Ax2>;

        #[cxx_name = "construct_unique"]
        pub fn gp_Ax3_from_gp_Ax2(axis: &gp_Ax2) -> UniquePtr<gp_Ax3>;

        #[cxx_name = "construct_unique"]
        pub fn gp_Dir_ctor(x: f64, y: f64, z: f64) -> UniquePtr<gp_Dir>;

        #[cxx_name = "construct_unique"]
        pub fn gp_Dir2d_ctor(x: f64, y: f64) -> UniquePtr<gp_Dir2d>;

        #[cxx_name = "construct_unique"]
        pub fn gp_Ax2d_ctor(point: &gp_Pnt2d, dir: &gp_Dir2d) -> UniquePtr<gp_Ax2d>;

        // Geometry Querying
        type GeomAPI_ProjectPointOnSurf;

        #[cxx_name = "construct_unique"]
        pub fn GeomAPI_ProjectPointOnSurf_ctor(
            origin: &gp_Pnt,
            surface: &HandleGeomSurface,
        ) -> UniquePtr<GeomAPI_ProjectPointOnSurf>;
        pub fn LowerDistanceParameters(self: &GeomAPI_ProjectPointOnSurf, u: &mut f64, v: &mut f64);

        // Transforms
        type gp_Trsf;

        #[cxx_name = "construct_unique"]
        pub fn new_transform() -> UniquePtr<gp_Trsf>;

        #[rust_name = "set_mirror_axis"]
        pub fn SetMirror(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1);
        pub fn SetRotation(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1, angle: f64);
        pub fn SetScale(self: Pin<&mut gp_Trsf>, point: &gp_Pnt, scale: f64);
        pub fn SetTranslation(self: Pin<&mut gp_Trsf>, point1: &gp_Pnt, point2: &gp_Pnt);

        #[cxx_name = "SetTranslationPart"]
        pub fn set_translation_vec(self: Pin<&mut gp_Trsf>, translation: &gp_Vec);

        type BRepBuilderAPI_Transform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_Transform_ctor(
            shape: &TopoDS_Shape,
            transform: &gp_Trsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPI_Transform>;

        pub fn Shape(self: Pin<&mut BRepBuilderAPI_Transform>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_Transform>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_Transform) -> bool;

        // Topology Explorer
        type TopExp_Explorer;
        type TopAbs_ShapeEnum;

        #[cxx_name = "construct_unique"]
        pub fn TopExp_Explorer_ctor(
            shape: &TopoDS_Shape,
            to_find: TopAbs_ShapeEnum,
        ) -> UniquePtr<TopExp_Explorer>;

        pub fn More(self: &TopExp_Explorer) -> bool;
        pub fn Next(self: Pin<&mut TopExp_Explorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExp_Explorer) -> UniquePtr<TopoDS_Shape>;
        pub fn Current(self: &TopExp_Explorer) -> &TopoDS_Shape;

        pub fn BRep_Tool_Surface(face: &TopoDS_Face) -> UniquePtr<HandleGeomSurface>;
        pub fn BRep_Tool_Curve(
            edge: &TopoDS_Edge,
            first: &mut f64,
            last: &mut f64,
        ) -> UniquePtr<HandleGeomCurve>;
        pub fn BRep_Tool_Pnt(vertex: &TopoDS_Vertex) -> UniquePtr<gp_Pnt>;
        pub fn BRep_Tool_Triangulation(
            face: &TopoDS_Face,
            location: Pin<&mut TopLoc_Location>,
        ) -> UniquePtr<Handle_Poly_Triangulation>;

        // BRepFeat
        type BRepFeat_MakeCylindricalHole;
        pub fn BRepFeat_MakeCylindricalHole_ctor() -> UniquePtr<BRepFeat_MakeCylindricalHole>;
        pub fn Init(
            self: Pin<&mut BRepFeat_MakeCylindricalHole>,
            shape: &TopoDS_Shape,
            axis: &gp_Ax1,
        );
        pub fn Perform(self: Pin<&mut BRepFeat_MakeCylindricalHole>, radius: f64);
        pub fn Build(self: Pin<&mut BRepFeat_MakeCylindricalHole>);
        pub fn Shape(self: &BRepFeat_MakeCylindricalHole) -> &TopoDS_Shape;

        // Data export
        type StlAPI_Writer;

        #[cxx_name = "construct_unique"]
        pub fn StlAPI_Writer_ctor() -> UniquePtr<StlAPI_Writer>;

        // pub fn Write(self: Pin<&mut StlAPI_Writer>, shape: &TopoDS_Shape, filename: &c_char) -> bool;
        pub fn write_stl(
            writer: Pin<&mut StlAPI_Writer>,
            shape: &TopoDS_Shape,
            filename: String,
        ) -> bool;

        // Triangulation
        type BRepMesh_IncrementalMesh;

        #[cxx_name = "construct_unique"]
        pub fn BRepMesh_IncrementalMesh_ctor(
            shape: &TopoDS_Shape,
            deflection: f64,
        ) -> UniquePtr<BRepMesh_IncrementalMesh>;

        pub fn Shape(self: &BRepMesh_IncrementalMesh) -> &TopoDS_Shape;
        pub fn IsDone(self: &BRepMesh_IncrementalMesh) -> bool;

        type TopLoc_Location;
        #[cxx_name = "construct_unique"]
        pub fn TopLoc_Location_ctor() -> UniquePtr<TopLoc_Location>;

        #[cxx_name = "construct_unique"]
        pub fn TopLoc_Location_from_transform(transform: &gp_Trsf) -> UniquePtr<TopLoc_Location>;

        type Handle_Poly_Triangulation;
        pub fn IsNull(self: &Handle_Poly_Triangulation) -> bool;
        #[cxx_name = "handle_try_deref"]
        pub fn Handle_Poly_Triangulation_Get(
            handle: &Handle_Poly_Triangulation,
        ) -> Result<&Poly_Triangulation>;

        type Poly_Triangulation;
        pub fn NbNodes(self: &Poly_Triangulation) -> i32;
        pub fn NbTriangles(self: &Poly_Triangulation) -> i32;
        pub fn HasNormals(self: &Poly_Triangulation) -> bool;
        pub fn HasUVNodes(self: &Poly_Triangulation) -> bool;
        pub fn Triangle(self: &Poly_Triangulation, index: i32) -> &Poly_Triangle;
        pub fn Poly_Triangulation_Normal(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Dir>;
        pub fn Poly_Triangulation_Node(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;
        pub fn Poly_Triangulation_UV(
            triangulation: &Poly_Triangulation,
            index: i32,
        ) -> UniquePtr<gp_Pnt2d>;

        type Poly_Triangle;
        pub fn Value(self: &Poly_Triangle, index: i32) -> i32;

        type Poly_Connect;
        #[cxx_name = "construct_unique"]
        pub fn Poly_Connect_ctor(
            triangulation: &Handle_Poly_Triangulation,
        ) -> UniquePtr<Poly_Connect>;

        type StdPrs_ToolTriangulatedShape;
        #[cxx_name = "construct_unique"]
        pub fn StdPrs_ToolTriangulatedShape_ctor() -> UniquePtr<StdPrs_ToolTriangulatedShape>;
        pub fn triangulated_shape_normal(
            face: &TopoDS_Face,
            poly_connect: Pin<&mut Poly_Connect>,
            normals: Pin<&mut TColgp_Array1OfDir>,
        );

        // Edge approximation
        type GCPnts_TangentialDeflection;

        #[cxx_name = "construct_unique"]
        pub fn GCPnts_TangentialDeflection_ctor(
            curve: &BRepAdaptor_Curve,
            angular_deflection: f64,
            curvature_deflection: f64,
        ) -> UniquePtr<GCPnts_TangentialDeflection>;
        pub fn NbPoints(self: &GCPnts_TangentialDeflection) -> i32;
        pub fn GCPnts_TangentialDeflection_Value(
            approximator: &GCPnts_TangentialDeflection,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;

        // Shape Properties
        type GProp_GProps;
        #[cxx_name = "construct_unique"]
        pub fn GProp_GProps_ctor() -> UniquePtr<GProp_GProps>;
        pub fn Mass(self: &GProp_GProps) -> f64;
        pub fn StaticMoments(self: &GProp_GProps, lx: &mut f64, ly: &mut f64, lz: &mut f64);
        pub fn MomentOfInertia(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
        pub fn RadiusOfGyration(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
        pub fn GProp_GProps_CentreOfMass(props: &GProp_GProps) -> UniquePtr<gp_Pnt>;

        pub fn BRepGProp_LinearProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);
        pub fn BRepGProp_SurfaceProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);
        pub fn BRepGProp_VolumeProperties(shape: &TopoDS_Shape, props: Pin<&mut GProp_GProps>);

        type BRepGProp_Face;

        #[cxx_name = "construct_unique"]
        pub fn BRepGProp_Face_ctor(face: &TopoDS_Face) -> UniquePtr<BRepGProp_Face>;
        pub fn Normal(
            self: &BRepGProp_Face,
            u: f64,
            v: f64,
            point: Pin<&mut gp_Pnt>,
            normal: Pin<&mut gp_Vec>,
        );

        // BRepTools
        pub fn outer_wire(face: &TopoDS_Face) -> UniquePtr<TopoDS_Wire>;

        // Cleaning
        type ShapeUpgrade_UnifySameDomain;

        #[cxx_name = "construct_unique"]
        pub fn ShapeUpgrade_UnifySameDomain_ctor(
            shape: &TopoDS_Shape,
            unify_edges: bool,
            unify_faces: bool,
            concat_b_splines: bool,
        ) -> UniquePtr<ShapeUpgrade_UnifySameDomain>;
        pub fn AllowInternalEdges(self: Pin<&mut ShapeUpgrade_UnifySameDomain>, allow: bool);
        pub fn Build(self: Pin<&mut ShapeUpgrade_UnifySameDomain>);
        pub fn Shape(self: &ShapeUpgrade_UnifySameDomain) -> &TopoDS_Shape;
    }
}
