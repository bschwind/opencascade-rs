#[cxx::bridge]
pub mod ffi {
    #[repr(u32)]
    #[derive(Debug)]
    #[cxx_name = "TopAbs_ShapeEnum"]
    pub enum TopAbsShapeEnum {
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
    #[cxx_name = "TopAbs_Orientation"]
    pub enum TopAbsOrientation {
        TopAbs_FORWARD,
        TopAbs_REVERSED,
        TopAbs_INTERNAL,
        TopAbs_EXTERNAL,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[cxx_name = "IFSelect_ReturnStatus"]
    pub enum IFSelectReturnStatus {
        IFSelect_RetVoid,
        IFSelect_RetDone,
        IFSelect_RetError,
        IFSelect_RetFail,
        IFSelect_RetStop,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[cxx_name = "BOPAlgo_GlueEnum"]
    pub enum BOPAlgoGlueEnum {
        BOPAlgo_GlueOff,
        BOPAlgo_GlueShift,
        BOPAlgo_GlueFull,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[cxx_name = "GeomAbs_CurveType"]
    pub enum GeomAbsCurveType {
        GeomAbs_Line,
        GeomAbs_Circle,
        GeomAbs_Ellipse,
        GeomAbs_Hyperbola,
        GeomAbs_Parabola,
        GeomAbs_BezierCurve,
        GeomAbs_BSplineCurve,
        GeomAbs_OffsetCurve,
        GeomAbs_OtherCurve,
    }

    #[repr(u32)]
    #[derive(Debug)]
    #[cxx_name = "GeomAbs_JoinType"]
    pub enum GeomAbsJoinType {
        GeomAbs_Arc,
        GeomAbs_Tangent,
        GeomAbs_Intersection,
    }

    unsafe extern "C++" {
        // https://github.com/dtolnay/cxx/issues/280

        // OCCT Includes
        include!("opencascade-sys/include/wrapper.hxx");

        // Runtime
        #[cxx_name = "Message_ProgressRange"]
        type MessageProgressRange;

        #[cxx_name = "construct_unique"]
        pub fn MessageProgressRange_ctor() -> UniquePtr<MessageProgressRange>;

        // Handles
        type HandleStandardType;
        type HandleGeomCurve;
        type HandleGeomBSplineCurve;
        type HandleGeomTrimmedCurve;
        type HandleGeomSurface;
        type HandleGeomBezierSurface;
        type HandleGeomPlane;
        type HandleGeom2dCurve;
        type HandleGeom2dEllipse;
        type HandleGeom2dTrimmedCurve;
        type HandleGeomCylindricalSurface;
        type Handle_TopToolsHSequenceOfShape;
        type HandleLawFunction;

        type Handle_TColgpHArray1OfPnt;
        pub fn new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(
            array: UniquePtr<TColgpHArray1OfPnt>,
        ) -> UniquePtr<Handle_TColgpHArray1OfPnt>;

        #[cxx_name = "DynamicType"]
        pub fn dynamic_type(surface: &HandleGeomSurface) -> &HandleStandardType;
        #[cxx_name = "TypeName"]
        pub fn type_name(handle: &HandleStandardType) -> String;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeom_BSplineCurve(
            bspline_curve_handle: &HandleGeomBSplineCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        #[cxx_name = "construct_unique"]
        pub fn new_HandleGeomCurve_from_HandleGeomTrimmedCurve(
            trimmed_curve_handle: &HandleGeomTrimmedCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<HandleGeomPlane>;

        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleStandardType) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomCurve) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomTrimmedCurve) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomSurface) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomBezierSurface) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomPlane) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeom2dCurve) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeom2dEllipse) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeom2dTrimmedCurve) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandleGeomCylindricalSurface) -> bool;
        #[cxx_name = "IsNull"]
        pub fn is_null(self: &Handle_TopToolsHSequenceOfShape) -> bool;

        pub fn HandleGeomCurve_Value(curve: &HandleGeomCurve, u: f64) -> UniquePtr<gp_Pnt>;

        // Collections
        #[cxx_name = "TopTools_ListOfShape"]
        type TopToolsListOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_list_of_shape() -> UniquePtr<TopToolsListOfShape>;
        pub fn shape_list_append_face(list: Pin<&mut TopToolsListOfShape>, face: &TopoDS_Face);
        #[cxx_name = "Size"]
        pub fn size(self: &TopToolsListOfShape) -> i32;

        #[cxx_name = "list_to_vector"]
        pub fn shape_list_to_vector(
            list: &TopToolsListOfShape,
        ) -> UniquePtr<CxxVector<TopoDS_Shape>>;

        #[cxx_name = "TopTools_IndexedMapOfShape"]
        type TopToolsIndexedMapOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_indexed_map_of_shape() -> UniquePtr<TopToolsIndexedMapOfShape>;
        #[cxx_name = "Extent"]
        pub fn extent(self: &TopToolsIndexedMapOfShape) -> i32;
        #[cxx_name = "FindKey"]
        pub fn find_key(self: &TopToolsIndexedMapOfShape, index: i32) -> &TopoDS_Shape;

        pub fn map_shapes(
            shape: &TopoDS_Shape,
            shape_type: TopAbsShapeEnum,
            shape_map: Pin<&mut TopToolsIndexedMapOfShape>,
        );

        #[cxx_name = "TopTools_IndexedDataMapOfShapeListOfShape"]
        type TopToolsIndexedDataMapOfShapeListOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_indexed_data_map_of_shape_list_of_shape(
        ) -> UniquePtr<TopToolsIndexedDataMapOfShapeListOfShape>;
        #[cxx_name = "Extent"]
        pub fn extent(self: &TopToolsIndexedDataMapOfShapeListOfShape) -> i32;
        #[cxx_name = "FindKey"]
        pub fn find_key(
            self: &TopToolsIndexedDataMapOfShapeListOfShape,
            index: i32,
        ) -> &TopoDS_Shape;
        #[cxx_name = "FindFromIndex"]
        pub fn find_from_index(
            self: &TopToolsIndexedDataMapOfShapeListOfShape,
            index: i32,
        ) -> &TopToolsListOfShape;
        #[cxx_name = "FindIndex"]
        pub fn find_index(
            self: &TopToolsIndexedDataMapOfShapeListOfShape,
            shape: &TopoDS_Shape,
        ) -> i32;
        #[cxx_name = "FindFromKey"]
        pub fn find_from_key<'a>(
            self: &'a TopToolsIndexedDataMapOfShapeListOfShape,
            shape: &'a TopoDS_Shape,
        ) -> &'a TopToolsListOfShape;

        pub fn map_shapes_and_ancestors(
            shape: &TopoDS_Shape,
            parent_type: TopAbsShapeEnum,
            child_type: TopAbsShapeEnum,
            shape_data_map: Pin<&mut TopToolsIndexedDataMapOfShapeListOfShape>,
        );
        pub fn map_shapes_and_unique_ancestors(
            shape: &TopoDS_Shape,
            parent_type: TopAbsShapeEnum,
            child_type: TopAbsShapeEnum,
            shape_data_map: Pin<&mut TopToolsIndexedDataMapOfShapeListOfShape>,
        );

        #[cxx_name = "TColgp_Array1OfDir"]
        type TColgpArray1OfDir;

        #[cxx_name = "construct_unique"]
        pub fn TColgpArray1OfDir_ctor(
            lower_bound: i32,
            upper_bound: i32,
        ) -> UniquePtr<TColgpArray1OfDir>;
        #[cxx_name = "Length"]
        pub fn length(self: &TColgpArray1OfDir) -> i32;
        pub fn TColgpArray1OfDir_Value(array: &TColgpArray1OfDir, index: i32) -> UniquePtr<gp_Dir>;

        #[cxx_name = "TColgp_Array1OfPnt2d"]
        type TColgpArray1OfPnt2d;
        #[cxx_name = "construct_unique"]
        pub fn TColgpArray1OfPnt2d_ctor(
            lower_bound: i32,
            upper_bound: i32,
        ) -> UniquePtr<TColgpArray1OfPnt2d>;
        #[cxx_name = "Length"]
        pub fn length(self: &TColgpArray1OfPnt2d) -> i32;
        pub fn TColgpArray1OfPnt2d_Value(
            array: &TColgpArray1OfPnt2d,
            index: i32,
        ) -> UniquePtr<gp_Pnt2d>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut TColgpArray1OfPnt2d>, index: i32, item: &gp_Pnt2d);

        #[cxx_name = "TColgp_Array2OfPnt"]
        type TColgpArray2OfPnt;
        #[cxx_name = "construct_unique"]
        pub fn TColgpArray2OfPnt_ctor(
            row_lower: i32,
            row_upper: i32,
            column_lower: i32,
            column_upper: i32,
        ) -> UniquePtr<TColgpArray2OfPnt>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut TColgpArray2OfPnt>, row: i32, column: i32, item: &gp_Pnt);

        #[cxx_name = "TColgp_HArray1OfPnt"]
        type TColgpHArray1OfPnt;
        #[cxx_name = "construct_unique"]
        pub fn TColgpHArray1OfPnt_ctor(
            lower_bound: i32,
            upper_bound: i32,
        ) -> UniquePtr<TColgpHArray1OfPnt>;
        #[cxx_name = "Length"]
        pub fn length(self: &TColgpHArray1OfPnt) -> i32;
        pub fn TColgpHArray1OfPnt_Value(
            array: &TColgpHArray1OfPnt,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut TColgpHArray1OfPnt>, index: i32, item: &gp_Pnt);

        #[cxx_name = "TopTools_HSequenceOfShape"]
        type TopToolsHSequenceOfShape;

        #[cxx_name = "Length"]
        pub fn length(self: &TopToolsHSequenceOfShape) -> i32;

        pub fn new_Handle_TopToolsHSequenceOfShape() -> UniquePtr<Handle_TopToolsHSequenceOfShape>;
        pub fn TopToolsHSequenceOfShape_append(
            handle: Pin<&mut Handle_TopToolsHSequenceOfShape>,
            shape: &TopoDS_Shape,
        );

        pub fn TopToolsHSequenceOfShape_length(handle: &Handle_TopToolsHSequenceOfShape) -> i32;
        pub fn TopToolsHSequenceOfShape_value(
            handle: &Handle_TopToolsHSequenceOfShape,
            index: i32,
        ) -> &TopoDS_Shape;

        #[cxx_name = "handle_try_deref"]
        pub fn Handle_TopToolsHSequenceOfShape_Get(
            handle: &Handle_TopToolsHSequenceOfShape,
        ) -> Result<&TopToolsHSequenceOfShape>;

        // Law Function
        #[cxx_name = "Law_Function"]
        type LawFunction;

        pub fn LawFunction_to_handle(law: UniquePtr<LawFunction>) -> UniquePtr<HandleLawFunction>;

        // Law Interpol
        #[cxx_name = "Law_Interpol"]
        type LawInterpol;

        #[cxx_name = "construct_unique"]
        pub fn LawInterpol_ctor() -> UniquePtr<LawInterpol>;
        pub fn LawInterpol_into_LawFunction(
            interpol: UniquePtr<LawInterpol>,
        ) -> UniquePtr<LawFunction>;
        #[cxx_name = "Set"]
        pub fn set(self: Pin<&mut LawInterpol>, array: &TColgpArray1OfPnt2d, periodic: bool);

        // Geometry
        #[cxx_name = "Geom_TrimmedCurve"]
        type GeomTrimmedCurve;
        #[cxx_name = "Geom_CylindricalSurface"]
        type GeomCylindricalSurface;
        #[cxx_name = "Geom_BezierSurface"]
        type GeomBezierSurface;
        #[cxx_name = "Geom2d_Ellipse"]
        type Geom2dEllipse;
        #[cxx_name = "Geom2d_Curve"]
        type Geom2dCurve;
        #[cxx_name = "Geom2d_TrimmedCurve"]
        type Geom2dTrimmedCurve;

        pub fn handle_geom_plane_location(plane: &HandleGeomPlane) -> &gp_Pnt;

        pub fn GeomCylindricalSurface_ctor(
            axis: &gp_Ax3,
            radius: f64,
        ) -> UniquePtr<HandleGeomCylindricalSurface>;
        pub fn cylinder_to_surface(
            cylinder_handle: &HandleGeomCylindricalSurface,
        ) -> UniquePtr<HandleGeomSurface>;

        pub fn GeomBezierSurface_ctor(
            poles: &TColgpArray2OfPnt,
        ) -> UniquePtr<HandleGeomBezierSurface>;
        pub fn bezier_to_surface(
            bezier_handle: &HandleGeomBezierSurface,
        ) -> UniquePtr<HandleGeomSurface>;

        pub fn Geom2dEllipse_ctor(
            axis: &gp_Ax2d,
            major_radius: f64,
            minor_radius: f64,
        ) -> UniquePtr<HandleGeom2dEllipse>;
        pub fn ellipse_to_HandleGeom2dCurve(
            ellipse: &HandleGeom2dEllipse,
        ) -> UniquePtr<HandleGeom2dCurve>;
        pub fn Geom2dTrimmedCurve_ctor(
            curve_handle: &HandleGeom2dCurve,
            u1: f64,
            u2: f64,
        ) -> UniquePtr<HandleGeom2dTrimmedCurve>;
        pub fn HandleGeom2dTrimmedCurve_to_curve(
            trimmed_curve: &HandleGeom2dTrimmedCurve,
        ) -> UniquePtr<HandleGeom2dCurve>;

        pub fn ellipse_value(ellipse: &HandleGeom2dEllipse, u: f64) -> UniquePtr<gp_Pnt2d>;

        // Points
        type gp_Pnt;
        type gp_Pnt2d;

        #[cxx_name = "construct_unique"]
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;

        #[cxx_name = "X"]
        pub fn x(self: &gp_Pnt) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &gp_Pnt) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &gp_Pnt) -> f64;
        #[cxx_name = "Distance"]
        pub fn distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;
        #[cxx_name = "Transform"]
        pub fn transform(self: Pin<&mut gp_Pnt>, transform: &gp_Trsf);

        #[cxx_name = "construct_unique"]
        pub fn new_point_2d(x: f64, y: f64) -> UniquePtr<gp_Pnt2d>;

        #[cxx_name = "X"]
        pub fn x(self: &gp_Pnt2d) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &gp_Pnt2d) -> f64;
        #[cxx_name = "Distance"]
        pub fn distance(self: &gp_Pnt2d, other: &gp_Pnt2d) -> f64;

        type gp_Vec;

        #[cxx_name = "construct_unique"]
        pub fn new_vec(x: f64, y: f64, z: f64) -> UniquePtr<gp_Vec>;

        #[cxx_name = "X"]
        pub fn x(self: &gp_Vec) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &gp_Vec) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &gp_Vec) -> f64;

        // Edge types
        #[cxx_name = "GeomAbs_CurveType"]
        type GeomAbsCurveType;

        // Segments
        type GC_MakeSegment;
        type GCE2d_MakeSegment;

        #[cxx_name = "construct_unique"]
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;

        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;
        pub fn GCE2d_MakeSegment_point_point(
            p1: &gp_Pnt2d,
            p2: &gp_Pnt2d,
        ) -> UniquePtr<HandleGeom2dTrimmedCurve>;

        // Lines
        type gp_Lin;

        #[cxx_name = "construct_unique"]
        pub fn gp_Lin_ctor(point: &gp_Pnt, dir: &gp_Dir) -> UniquePtr<gp_Lin>;

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

        // Circles
        type gp_Circ;

        #[cxx_name = "construct_unique"]
        pub fn gp_Circ_ctor(axis: &gp_Ax2, radius: f64) -> UniquePtr<gp_Circ>;

        // Shapes
        type TopoDS_Vertex;
        type TopoDS_Edge;
        type TopoDS_Wire;
        type TopoDS_Face;
        type TopoDS_Shell;
        type TopoDS_Solid;
        type TopoDS_Shape;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Face_ctor() -> UniquePtr<TopoDS_Face>;

        pub fn cast_vertex_to_shape(wire: &TopoDS_Vertex) -> &TopoDS_Shape;
        pub fn cast_edge_to_shape(wire: &TopoDS_Edge) -> &TopoDS_Shape;
        pub fn cast_wire_to_shape(wire: &TopoDS_Wire) -> &TopoDS_Shape;
        pub fn cast_face_to_shape(wire: &TopoDS_Face) -> &TopoDS_Shape;
        pub fn cast_shell_to_shape(wire: &TopoDS_Shell) -> &TopoDS_Shape;
        pub fn cast_solid_to_shape(wire: &TopoDS_Solid) -> &TopoDS_Shape;
        pub fn cast_compound_to_shape(wire: &TopoDS_Compound) -> &TopoDS_Shape;

        pub fn TopoDS_cast_to_vertex(shape: &TopoDS_Shape) -> &TopoDS_Vertex;
        pub fn TopoDS_cast_to_wire(shape: &TopoDS_Shape) -> &TopoDS_Wire;
        pub fn TopoDS_cast_to_edge(shape: &TopoDS_Shape) -> &TopoDS_Edge;
        pub fn TopoDS_cast_to_face(shape: &TopoDS_Shape) -> &TopoDS_Face;
        pub fn TopoDS_cast_to_shell(shape: &TopoDS_Shape) -> &TopoDS_Shell;
        pub fn TopoDS_cast_to_solid(shape: &TopoDS_Shape) -> &TopoDS_Solid;
        pub fn TopoDS_cast_to_compound(shape: &TopoDS_Shape) -> &TopoDS_Compound;

        #[cxx_name = "Move"]
        pub fn translate(
            self: Pin<&mut TopoDS_Shape>,
            position: &TopLoc_Location,
            raise_exception: bool,
        );

        #[cxx_name = "Location"]
        pub fn set_global_translation(
            self: Pin<&mut TopoDS_Shape>,
            translation: &TopLoc_Location,
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

        #[cxx_name = "IsNull"]
        pub fn is_null(self: &TopoDS_Shape) -> bool;
        #[cxx_name = "IsEqual"]
        pub fn is_equal(self: &TopoDS_Shape, other: &TopoDS_Shape) -> bool;
        #[cxx_name = "ShapeType"]
        pub fn shape_type(self: &TopoDS_Shape) -> TopAbsShapeEnum;

        #[cxx_name = "TopAbs_Orientation"]
        type TopAbsOrientation;
        #[cxx_name = "Orientation"]
        pub fn orientation(self: &TopoDS_Shape) -> TopAbsOrientation;
        #[cxx_name = "Orientation"]
        pub fn orientation(self: &TopoDS_Face) -> TopAbsOrientation;

        // Compound Shapes
        type TopoDS_Compound;
        pub fn TopoDS_Compound_as_shape(
            compound: UniquePtr<TopoDS_Compound>,
        ) -> UniquePtr<TopoDS_Shape>;

        pub fn TopoDS_Shell_as_shape(shell: UniquePtr<TopoDS_Shell>) -> UniquePtr<TopoDS_Shape>;

        type BRep_Builder;
        type TopoDS_Builder;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Compound_ctor() -> UniquePtr<TopoDS_Compound>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Shell_ctor() -> UniquePtr<TopoDS_Shell>;

        #[cxx_name = "construct_unique"]
        pub fn BRep_Builder_ctor() -> UniquePtr<BRep_Builder>;

        pub fn BRep_Builder_upcast_to_topods_builder(builder: &BRep_Builder) -> &TopoDS_Builder;
        #[cxx_name = "MakeCompound"]
        pub fn make_compound(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Compound>);
        #[cxx_name = "MakeShell"]
        pub fn make_shell(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Shell>);
        #[cxx_name = "Add"]
        pub fn add(self: &TopoDS_Builder, shape: Pin<&mut TopoDS_Shape>, compound: &TopoDS_Shape);

        // BRepBuilder
        type BRepBuilderAPI_MakeVertex;
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeVertex_gp_Pnt(
            point: &gp_Pnt,
        ) -> UniquePtr<BRepBuilderAPI_MakeVertex>;

        #[cxx_name = "Vertex"]
        pub fn vertex(self: Pin<&mut BRepBuilderAPI_MakeVertex>) -> &TopoDS_Vertex;

        type BRepBuilderAPI_MakeEdge;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_circle(
            circle: &gp_Circ,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_gp_Pnt_gp_Pnt(
            p1: &gp_Pnt,
            p2: &gp_Pnt,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_CurveSurface2d(
            curve_handle: &HandleGeom2dCurve,
            surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "Vertex1"]
        pub fn vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDS_Vertex;
        #[cxx_name = "Edge"]
        pub fn edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeEdge>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeEdge) -> bool;

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

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Shape;
        #[cxx_name = "Wire"]
        pub fn wire(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDS_Wire;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeWire>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeWire) -> bool;

        type BRepBuilderAPI_MakeFace;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeFace_wire(
            wire: &TopoDS_Wire,
            only_plane: bool,
        ) -> UniquePtr<BRepBuilderAPI_MakeFace>;
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeFace_surface(
            surface: &HandleGeomSurface,
            edge_tolerance: f64,
        ) -> UniquePtr<BRepBuilderAPI_MakeFace>;

        #[cxx_name = "Face"]
        pub fn face(self: &BRepBuilderAPI_MakeFace) -> &TopoDS_Face;
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_MakeFace>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeFace>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeFace) -> bool;

        // BRepAdaptor
        type BRepAdaptor_Curve;

        #[cxx_name = "construct_unique"]
        pub fn BRepAdaptor_Curve_ctor(edge: &TopoDS_Edge) -> UniquePtr<BRepAdaptor_Curve>;
        #[cxx_name = "FirstParameter"]
        pub fn first_parameter(self: &BRepAdaptor_Curve) -> f64;
        #[cxx_name = "LastParameter"]
        pub fn last_parameter(self: &BRepAdaptor_Curve) -> f64;
        pub fn BRepAdaptor_Curve_value(curve: &BRepAdaptor_Curve, u: f64) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "GetType"]
        pub fn get_type(self: &BRepAdaptor_Curve) -> GeomAbsCurveType;

        // Primitives
        type BRepPrimAPI_MakePrism;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakePrism_ctor(
            shape: &TopoDS_Shape,
            vec: &gp_Vec,
            copy: bool,
            canonize: bool,
        ) -> UniquePtr<BRepPrimAPI_MakePrism>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakePrism>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakePrism>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakePrism) -> bool;

        type BRepFeat_MakeDPrism;

        #[cxx_name = "construct_unique"]
        pub fn BRepFeat_MakeDPrism_ctor(
            shape: &TopoDS_Shape,
            profile_base: &TopoDS_Face,
            sketch_base: &TopoDS_Face,
            angle: f64,
            fuse: i32, // 0 = subtractive, 1 = additive
            modify: bool,
        ) -> UniquePtr<BRepFeat_MakeDPrism>;

        #[cxx_name = "Perform"]
        pub fn perform_until_face(self: Pin<&mut BRepFeat_MakeDPrism>, until: &TopoDS_Shape);

        #[cxx_name = "Perform"]
        pub fn perform_with_height(self: Pin<&mut BRepFeat_MakeDPrism>, height: f64);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFeat_MakeDPrism>) -> &TopoDS_Shape;

        type BRepPrimAPI_MakeRevol;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeRevol_ctor(
            shape: &TopoDS_Shape,
            axis: &gp_Ax1,
            angle: f64,
            copy: bool,
        ) -> UniquePtr<BRepPrimAPI_MakeRevol>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeRevol>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeRevol>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeRevol) -> bool;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepBuilderAPI_MakeWire>, edge: &TopoDS_Edge);

        #[rust_name = "add_wire"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepBuilderAPI_MakeWire>, wire: &TopoDS_Wire);

        type BRepPrimAPI_MakeCylinder;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeCylinder_ctor(
            coord_system: &gp_Ax2,
            radius: f64,
            height: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeCylinder>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeCylinder>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeCylinder>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeCylinder) -> bool;

        type BRepPrimAPI_MakeBox;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeBox_ctor(
            point: &gp_Pnt,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeBox>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeBox>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeBox) -> bool;

        type BRepPrimAPI_MakeSphere;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeSphere_ctor(
            axis: &gp_Ax2,
            r: f64,
            angle_1: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeSphere>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeSphere>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeSphere>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeSphere) -> bool;

        type BRepPrimAPI_MakeCone;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeCone_ctor(
            axis: &gp_Ax2,
            r1: f64,
            r2: f64,
            h: f64,
            angle: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeCone>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeCone>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeCone>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeCone) -> bool;

        type BRepPrimAPI_MakeTorus;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeTorus_ctor(
            axis: &gp_Ax2,
            r1: f64,
            r2: f64,
            angle_1: f64,
            angle_2: f64,
            angle_3: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeTorus>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeTorus>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeTorus>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeTorus) -> bool;

        // BRepLib
        pub fn BRepLibBuildCurves3d(shape: &TopoDS_Shape) -> bool;

        // Fillets
        type BRepFilletAPI_MakeFillet;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeFillet_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeFillet>;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepFilletAPI_MakeFillet>, radius: f64, edge: &TopoDS_Edge);

        #[rust_name = "variable_add_edge"]
        pub fn Add(
            self: Pin<&mut BRepFilletAPI_MakeFillet>,
            radius_values: &TColgpArray1OfPnt2d,
            edge: &TopoDS_Edge,
        );

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPI_MakeFillet>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPI_MakeFillet>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPI_MakeFillet) -> bool;

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
        pub fn BRepFilletAPI_MakeFillet2d_add_chamfer(
            make_fillet: Pin<&mut BRepFilletAPI_MakeFillet2d>,
            edge1: &TopoDS_Edge,
            edge2: &TopoDS_Edge,
            distance1: f64,
            distance2: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        pub fn BRepFilletAPI_MakeFillet2d_add_chamfer_angle(
            make_fillet: Pin<&mut BRepFilletAPI_MakeFillet2d>,
            edge: &TopoDS_Edge,
            vertex: &TopoDS_Vertex,
            distance: f64,
            angle: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPI_MakeFillet2d>, progress: &MessageProgressRange);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPI_MakeFillet2d>) -> &TopoDS_Shape;
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPI_MakeFillet2d) -> bool;

        // Chamfers
        type BRepFilletAPI_MakeChamfer;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeChamfer_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeChamfer>;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepFilletAPI_MakeChamfer>, distance: f64, edge: &TopoDS_Edge);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPI_MakeChamfer>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPI_MakeChamfer>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPI_MakeChamfer) -> bool;

        // Offset
        type BRepOffsetAPI_MakeOffset;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_face_ctor(
            face: &TopoDS_Face,
            join: GeomAbsJoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_wire_ctor(
            wire: &TopoDS_Wire,
            join: GeomAbsJoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;

        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut BRepOffsetAPI_MakeOffset>, offset: f64, alt: f64);

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPI_MakeOffset>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPI_MakeOffset>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPI_MakeOffset) -> bool;

        #[cxx_name = "GeomAbs_JoinType"]
        type GeomAbsJoinType;

        // Solids
        type BRepOffsetAPI_MakeThickSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeThickSolid_ctor() -> UniquePtr<BRepOffsetAPI_MakeThickSolid>;

        pub fn MakeThickSolidByJoin(
            make_thick_solid: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            shape: &TopoDS_Shape,
            closing_faces: &TopToolsListOfShape,
            offset: f64,
            tolerance: f64,
        );
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPI_MakeThickSolid>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepOffsetAPI_MakeThickSolid>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPI_MakeThickSolid) -> bool;

        // Sweeps
        type BRepOffsetAPI_MakePipe;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipe_ctor(
            spine: &TopoDS_Wire,
            profile: &TopoDS_Shape,
        ) -> UniquePtr<BRepOffsetAPI_MakePipe>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPI_MakePipe>) -> &TopoDS_Shape;

        // Sweeps with a law function
        type BRepOffsetAPI_MakePipeShell;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipeShell_ctor(
            spine: &TopoDS_Wire,
        ) -> UniquePtr<BRepOffsetAPI_MakePipeShell>;

        #[cxx_name = "SetMode"]
        pub fn set_mode(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, is_frenet: bool);

        pub fn Add(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            with_contact: bool,
            with_correction: bool,
        );

        pub fn SetLaw(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            law: &HandleLawFunction,
            with_contact: bool,
            with_correction: bool,
        );

        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, progress: &MessageProgressRange);
        #[cxx_name = "MakeSolid"]
        pub fn make_solid(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> bool;
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> &TopoDS_Shape;

        // Lofting
        type BRepOffsetAPI_ThruSections;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_ThruSections_ctor(
            is_solid: bool,
        ) -> UniquePtr<BRepOffsetAPI_ThruSections>;

        #[cxx_name = "AddWire"]
        pub fn add_wire(self: Pin<&mut BRepOffsetAPI_ThruSections>, wire: &TopoDS_Wire);
        #[cxx_name = "CheckCompatibility"]
        pub fn check_compatibility(self: Pin<&mut BRepOffsetAPI_ThruSections>, check: bool);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPI_ThruSections>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPI_ThruSections>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPI_ThruSections) -> bool;

        // Boolean Operations
        type BRepAlgoAPI_Fuse;
        #[cxx_name = "BOPAlgo_GlueEnum"]
        type BOPAlgoGlueEnum;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Fuse_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPI_Fuse>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPI_Fuse) -> bool;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopToolsListOfShape;
        #[cxx_name = "SetGlue"]
        pub fn set_glue(self: Pin<&mut BRepAlgoAPI_Fuse>, glue: BOPAlgoGlueEnum);

        type BRepAlgoAPI_Cut;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Cut_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Cut>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPI_Cut>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPI_Cut) -> bool;
        pub fn Generated<'a>(
            self: Pin<&'a mut BRepAlgoAPI_Cut>,
            shape: &'a TopoDS_Shape,
        ) -> &'a TopToolsListOfShape;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopToolsListOfShape;

        type BRepAlgoAPI_Common;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Common_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Common>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPI_Common>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPI_Common) -> bool;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopToolsListOfShape;

        type BRepAlgoAPI_Section;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Section_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Section>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPI_Section>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPI_Section) -> bool;

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

        #[cxx_name = "X"]
        pub fn x(self: &gp_Dir) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &gp_Dir) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &gp_Dir) -> f64;

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

        // Geometry Interpolation
        type GeomAPI_Interpolate;

        #[cxx_name = "construct_unique"]
        pub fn GeomAPI_Interpolate_ctor(
            points: &Handle_TColgpHArray1OfPnt,
            periodic: bool,
            tolerance: f64,
        ) -> UniquePtr<GeomAPI_Interpolate>;

        pub fn Load(
            self: Pin<&mut GeomAPI_Interpolate>,
            initial_tangent: &gp_Vec,
            final_tangent: &gp_Vec,
            scale: bool,
        );

        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut GeomAPI_Interpolate>);

        pub fn GeomAPI_Interpolate_Curve(
            interpolate: &GeomAPI_Interpolate,
        ) -> UniquePtr<HandleGeomBSplineCurve>;

        // Geometry Querying
        type GeomAPI_ProjectPointOnSurf;

        #[cxx_name = "construct_unique"]
        pub fn GeomAPI_ProjectPointOnSurf_ctor(
            origin: &gp_Pnt,
            surface: &HandleGeomSurface,
        ) -> UniquePtr<GeomAPI_ProjectPointOnSurf>;
        #[cxx_name = "LowerDistanceParameters"]
        pub fn lower_distance_parameters(
            self: &GeomAPI_ProjectPointOnSurf,
            u: &mut f64,
            v: &mut f64,
        );

        // Transforms
        type gp_Trsf;

        #[cxx_name = "construct_unique"]
        pub fn new_transform() -> UniquePtr<gp_Trsf>;

        #[rust_name = "set_mirror_axis"]
        #[cxx_name = "SetMirror"]
        pub fn set_mirror(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1);
        #[cxx_name = "SetRotation"]
        pub fn set_rotation(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1, angle: f64);
        #[cxx_name = "SetScale"]
        pub fn set_scale(self: Pin<&mut gp_Trsf>, point: &gp_Pnt, scale: f64);
        #[cxx_name = "SetTranslation"]
        pub fn set_translation(self: Pin<&mut gp_Trsf>, point1: &gp_Pnt, point2: &gp_Pnt);
        #[cxx_name = "Value"]
        pub fn value(self: &gp_Trsf, the_row: i32, the_col: i32) -> f64;

        #[cxx_name = "SetTranslationPart"]
        pub fn set_translation_vec(self: Pin<&mut gp_Trsf>, translation: &gp_Vec);

        type gp_GTrsf;
        #[cxx_name = "construct_unique"]
        pub fn new_gp_GTrsf() -> UniquePtr<gp_GTrsf>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut gp_GTrsf>, the_row: i32, the_col: i32, the_value: f64);
        #[cxx_name = "Value"]
        pub fn value(self: &gp_GTrsf, the_row: i32, the_col: i32) -> f64;

        type BRepBuilderAPI_MakeSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeSolid_ctor(
            shell: &TopoDS_Shell,
        ) -> UniquePtr<BRepBuilderAPI_MakeSolid>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_MakeSolid>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeSolid>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeSolid) -> bool;

        type BRepBuilderAPI_MakeShapeOnMesh;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeShapeOnMesh_ctor(
            mesh: &Handle_Poly_Triangulation,
        ) -> UniquePtr<BRepBuilderAPI_MakeShapeOnMesh>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_MakeShapeOnMesh>) -> &TopoDS_Shape;
        pub fn Build(
            self: Pin<&mut BRepBuilderAPI_MakeShapeOnMesh>,
            progress: &MessageProgressRange,
        );
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeShapeOnMesh) -> bool;

        type BRepBuilderAPI_Transform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_Transform_ctor(
            shape: &TopoDS_Shape,
            transform: &gp_Trsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPI_Transform>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_Transform>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_Transform>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_Transform) -> bool;

        type BRepBuilderAPI_GTransform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_GTransform_ctor(
            shape: &TopoDS_Shape,
            transform: &gp_GTrsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPI_GTransform>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_GTransform>) -> &TopoDS_Shape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_GTransform>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_GTransform) -> bool;

        // Topology Explorer
        type TopExp_Explorer;

        #[cxx_name = "TopAbs_ShapeEnum"]
        type TopAbsShapeEnum;

        #[cxx_name = "construct_unique"]
        pub fn TopExp_Explorer_ctor(
            shape: &TopoDS_Shape,
            to_find: TopAbsShapeEnum,
        ) -> UniquePtr<TopExp_Explorer>;

        #[cxx_name = "More"]
        pub fn more(self: &TopExp_Explorer) -> bool;
        #[cxx_name = "Next"]
        pub fn next(self: Pin<&mut TopExp_Explorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExp_Explorer) -> UniquePtr<TopoDS_Shape>;
        #[cxx_name = "Current"]
        pub fn current(self: &TopExp_Explorer) -> &TopoDS_Shape;

        pub fn TopExp_FirstVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        pub fn TopExp_LastVertex(edge: &TopoDS_Edge) -> UniquePtr<TopoDS_Vertex>;
        pub fn TopExp_EdgeVertices(
            edge: &TopoDS_Edge,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
        );
        pub fn TopExp_WireVertices(
            wire: &TopoDS_Wire,
            vertex_first: Pin<&mut TopoDS_Vertex>,
            vertex_last: Pin<&mut TopoDS_Vertex>,
        );
        pub fn TopExp_CommonVertex(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
            vertex: Pin<&mut TopoDS_Vertex>,
        ) -> bool;

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

        type BRepIntCurveSurface_Inter;

        #[cxx_name = "construct_unique"]
        pub fn BRepIntCurveSurface_Inter_ctor() -> UniquePtr<BRepIntCurveSurface_Inter>;
        pub fn Init(
            self: Pin<&mut BRepIntCurveSurface_Inter>,
            shape: &TopoDS_Shape,
            line: &gp_Lin,
            tolerance: f64,
        );
        #[cxx_name = "More"]
        pub fn more(self: &BRepIntCurveSurface_Inter) -> bool;
        #[cxx_name = "Next"]
        pub fn next(self: Pin<&mut BRepIntCurveSurface_Inter>);
        pub fn BRepIntCurveSurface_Inter_face(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<TopoDS_Face>;
        pub fn BRepIntCurveSurface_Inter_point(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "U"]
        pub fn u(self: &BRepIntCurveSurface_Inter) -> f64;
        #[cxx_name = "V"]
        pub fn v(self: &BRepIntCurveSurface_Inter) -> f64;
        #[cxx_name = "W"]
        pub fn w(self: &BRepIntCurveSurface_Inter) -> f64;

        // BRepFeat
        type BRepFeat_MakeCylindricalHole;
        pub fn BRepFeat_MakeCylindricalHole_ctor() -> UniquePtr<BRepFeat_MakeCylindricalHole>;
        pub fn Init(
            self: Pin<&mut BRepFeat_MakeCylindricalHole>,
            shape: &TopoDS_Shape,
            axis: &gp_Ax1,
        );
        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut BRepFeat_MakeCylindricalHole>, radius: f64);
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFeat_MakeCylindricalHole>);
        #[cxx_name = "Shape"]
        pub fn shape(self: &BRepFeat_MakeCylindricalHole) -> &TopoDS_Shape;

        // Data Import
        type STEPControl_Reader;

        #[cxx_name = "IFSelect_ReturnStatus"]
        type IFSelectReturnStatus;

        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Reader_ctor() -> UniquePtr<STEPControl_Reader>;

        pub fn read_step(
            reader: Pin<&mut STEPControl_Reader>,
            filename: String,
        ) -> IFSelectReturnStatus;
        pub fn TransferRoots(
            self: Pin<&mut STEPControl_Reader>,
            progress: &MessageProgressRange,
        ) -> i32;
        pub fn one_shape(reader: &STEPControl_Reader) -> UniquePtr<TopoDS_Shape>;

        // Data Export
        type STEPControl_Writer;

        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Writer_ctor() -> UniquePtr<STEPControl_Writer>;

        pub fn transfer_shape(
            writer: Pin<&mut STEPControl_Writer>,
            shape: &TopoDS_Shape,
        ) -> IFSelectReturnStatus;
        pub fn write_step(
            writer: Pin<&mut STEPControl_Writer>,
            filename: String,
        ) -> IFSelectReturnStatus;

        type StlAPI_Writer;

        #[cxx_name = "construct_unique"]
        pub fn StlAPI_Writer_ctor() -> UniquePtr<StlAPI_Writer>;

        #[cxx_name = "WriteStl"]
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

        #[cxx_name = "Shape"]
        pub fn shape(self: &BRepMesh_IncrementalMesh) -> &TopoDS_Shape;
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepMesh_IncrementalMesh) -> bool;

        type TopLoc_Location;
        #[cxx_name = "construct_unique"]
        pub fn TopLoc_Location_ctor() -> UniquePtr<TopLoc_Location>;

        #[cxx_name = "construct_unique"]
        pub fn TopLoc_Location_from_transform(transform: &gp_Trsf) -> UniquePtr<TopLoc_Location>;

        pub fn TopLoc_Location_Transformation(location: &TopLoc_Location) -> UniquePtr<gp_Trsf>;

        type Handle_Poly_Triangulation;

        pub fn Handle_Poly_Triangulation_ctor(
            triangulation: UniquePtr<Poly_Triangulation>,
        ) -> UniquePtr<Handle_Poly_Triangulation>;

        #[cxx_name = "IsNull"]
        pub fn is_null(self: &Handle_Poly_Triangulation) -> bool;
        #[cxx_name = "handle_try_deref"]
        pub fn Handle_Poly_Triangulation_Get(
            handle: &Handle_Poly_Triangulation,
        ) -> Result<&Poly_Triangulation>;

        type Poly_Triangulation;
        #[cxx_name = "construct_unique"]
        pub fn Poly_Triangulation_ctor(
            nb_nodes: i32,
            nb_triangles: i32,
            has_uv: bool,
            has_normals: bool,
        ) -> UniquePtr<Poly_Triangulation>;
        #[cxx_name = "NbNodes"]
        pub fn nb_nodes(self: &Poly_Triangulation) -> i32;
        #[cxx_name = "NbTriangles"]
        pub fn nb_triangles(self: &Poly_Triangulation) -> i32;
        #[cxx_name = "HasNormals"]
        pub fn has_normals(self: &Poly_Triangulation) -> bool;
        #[cxx_name = "HasUVNodes"]
        pub fn has_uv_nodes(self: &Poly_Triangulation) -> bool;
        #[cxx_name = "Triangle"]
        pub fn triangle(self: &Poly_Triangulation, index: i32) -> &Poly_Triangle;
        pub fn SetTriangle(
            self: Pin<&mut Poly_Triangulation>,
            index: i32,
            triangle: &Poly_Triangle,
        );
        #[cxx_name = "SetNode"]
        pub fn set_node(self: Pin<&mut Poly_Triangulation>, index: i32, node: &gp_Pnt);
        #[cxx_name = "SetNormal"]
        pub fn set_normal(self: Pin<&mut Poly_Triangulation>, index: i32, dir: &gp_Dir);
        #[cxx_name = "SetUVNode"]
        pub fn set_uv_node(self: Pin<&mut Poly_Triangulation>, index: i32, uv: &gp_Pnt2d);
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
        #[cxx_name = "construct_unique"]
        pub fn Poly_Triangle_ctor(node1: i32, node2: i32, node3: i32) -> UniquePtr<Poly_Triangle>;
        #[cxx_name = "Value"]
        pub fn value(self: &Poly_Triangle, index: i32) -> i32;

        type Poly_Connect;
        #[cxx_name = "construct_unique"]
        pub fn Poly_Connect_ctor(
            triangulation: &Handle_Poly_Triangulation,
        ) -> UniquePtr<Poly_Connect>;

        pub fn compute_normals(face: &TopoDS_Face, triangulation: &Handle_Poly_Triangulation);

        // Edge approximation
        type GCPnts_TangentialDeflection;

        #[cxx_name = "construct_unique"]
        pub fn GCPnts_TangentialDeflection_ctor(
            curve: &BRepAdaptor_Curve,
            angular_deflection: f64,
            curvature_deflection: f64,
        ) -> UniquePtr<GCPnts_TangentialDeflection>;
        #[cxx_name = "NbPoints"]
        pub fn nb_points(self: &GCPnts_TangentialDeflection) -> i32;
        pub fn GCPnts_TangentialDeflection_Value(
            approximator: &GCPnts_TangentialDeflection,
            index: i32,
        ) -> UniquePtr<gp_Pnt>;

        // Shape Properties
        type GProp_GProps;
        #[cxx_name = "construct_unique"]
        pub fn GProp_GProps_ctor() -> UniquePtr<GProp_GProps>;
        #[cxx_name = "Mass"]
        pub fn mass(self: &GProp_GProps) -> f64;
        #[cxx_name = "StaticMoments"]
        pub fn static_moments(self: &GProp_GProps, lx: &mut f64, ly: &mut f64, lz: &mut f64);
        #[cxx_name = "MomentOfInertia"]
        pub fn moment_of_inertia(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
        #[cxx_name = "RadiusOfGyration"]
        pub fn radius_of_gyration(self: &GProp_GProps, axis: &gp_Ax1) -> f64;
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
        #[cxx_name = "AllowInternalEdges"]
        pub fn allow_internal_edges(self: Pin<&mut ShapeUpgrade_UnifySameDomain>, allow: bool);
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut ShapeUpgrade_UnifySameDomain>);
        #[cxx_name = "Shape"]
        pub fn shape(self: &ShapeUpgrade_UnifySameDomain) -> &TopoDS_Shape;

        pub fn connect_edges_to_wires(
            edges: Pin<&mut Handle_TopToolsHSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut Handle_TopToolsHSequenceOfShape>,
        );
    }
}

// Gross, but is this okay?
unsafe impl Send for ffi::BRepBuilderAPI_MakeWire {}
unsafe impl Send for ffi::TopoDS_Edge {}
unsafe impl Send for ffi::TopoDS_Wire {}
unsafe impl Send for ffi::TopoDS_Face {}
unsafe impl Send for ffi::TopoDS_Shell {}
unsafe impl Send for ffi::TopoDS_Solid {}
unsafe impl Send for ffi::TopoDS_Compound {}
unsafe impl Send for ffi::TopoDS_Shape {}

unsafe impl Send for ffi::TopExp_Explorer {}
unsafe impl Send for ffi::BRepFilletAPI_MakeChamfer {}
