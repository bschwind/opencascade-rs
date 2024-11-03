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
        type HandleTopToolsHSequenceOfShape;
        type HandleLawFunction;
        type HandleTColgpHArray1OfPnt;
        pub fn new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(
            array: UniquePtr<TColgpHArray1OfPnt>,
        ) -> UniquePtr<HandleTColgpHArray1OfPnt>;

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
        pub fn is_null(self: &HandleTopToolsHSequenceOfShape) -> bool;

        pub fn HandleGeomCurve_Value(curve: &HandleGeomCurve, u: f64) -> UniquePtr<GpPoint>;

        // Collections
        #[cxx_name = "TopTools_ListOfShape"]
        type TopToolsListOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_list_of_shape() -> UniquePtr<TopToolsListOfShape>;
        pub fn shape_list_append_face(list: Pin<&mut TopToolsListOfShape>, face: &TopoDSFace);
        #[cxx_name = "Size"]
        pub fn size(self: &TopToolsListOfShape) -> i32;

        #[cxx_name = "list_to_vector"]
        pub fn shape_list_to_vector(
            list: &TopToolsListOfShape,
        ) -> UniquePtr<CxxVector<TopoDSShape>>;

        #[cxx_name = "TopTools_IndexedMapOfShape"]
        type TopToolsIndexedMapOfShape;

        #[cxx_name = "construct_unique"]
        pub fn new_indexed_map_of_shape() -> UniquePtr<TopToolsIndexedMapOfShape>;
        #[cxx_name = "Extent"]
        pub fn extent(self: &TopToolsIndexedMapOfShape) -> i32;
        #[cxx_name = "FindKey"]
        pub fn find_key(self: &TopToolsIndexedMapOfShape, index: i32) -> &TopoDSShape;

        pub fn map_shapes(
            shape: &TopoDSShape,
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
        ) -> &TopoDSShape;
        #[cxx_name = "FindFromIndex"]
        pub fn find_from_index(
            self: &TopToolsIndexedDataMapOfShapeListOfShape,
            index: i32,
        ) -> &TopToolsListOfShape;
        #[cxx_name = "FindIndex"]
        pub fn find_index(
            self: &TopToolsIndexedDataMapOfShapeListOfShape,
            shape: &TopoDSShape,
        ) -> i32;
        #[cxx_name = "FindFromKey"]
        pub fn find_from_key<'a>(
            self: &'a TopToolsIndexedDataMapOfShapeListOfShape,
            shape: &'a TopoDSShape,
        ) -> &'a TopToolsListOfShape;

        pub fn map_shapes_and_ancestors(
            shape: &TopoDSShape,
            parent_type: TopAbsShapeEnum,
            child_type: TopAbsShapeEnum,
            shape_data_map: Pin<&mut TopToolsIndexedDataMapOfShapeListOfShape>,
        );
        pub fn map_shapes_and_unique_ancestors(
            shape: &TopoDSShape,
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
        pub fn TColgpArray1OfDir_Value(array: &TColgpArray1OfDir, index: i32) -> UniquePtr<GpDir>;

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
        ) -> UniquePtr<GpPoint2d>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut TColgpArray1OfPnt2d>, index: i32, item: &GpPoint2d);

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
        pub fn set_value(self: Pin<&mut TColgpArray2OfPnt>, row: i32, column: i32, item: &GpPoint);

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
        ) -> UniquePtr<GpPoint>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut TColgpHArray1OfPnt>, index: i32, item: &GpPoint);

        #[cxx_name = "TopTools_HSequenceOfShape"]
        type TopToolsHSequenceOfShape;

        #[cxx_name = "Length"]
        pub fn length(self: &TopToolsHSequenceOfShape) -> i32;

        pub fn new_HandleTopToolsHSequenceOfShape() -> UniquePtr<HandleTopToolsHSequenceOfShape>;
        pub fn TopToolsHSequenceOfShape_append(
            handle: Pin<&mut HandleTopToolsHSequenceOfShape>,
            shape: &TopoDSShape,
        );

        pub fn TopToolsHSequenceOfShape_length(handle: &HandleTopToolsHSequenceOfShape) -> i32;
        pub fn TopToolsHSequenceOfShape_value(
            handle: &HandleTopToolsHSequenceOfShape,
            index: i32,
        ) -> &TopoDSShape;

        #[cxx_name = "handle_try_deref"]
        pub fn HandleTopToolsHSequenceOfShape_Get(
            handle: &HandleTopToolsHSequenceOfShape,
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

        pub fn handle_geom_plane_location(plane: &HandleGeomPlane) -> &GpPoint;

        pub fn GeomCylindricalSurface_ctor(
            axis: &GpAx3,
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
            axis: &GpAx2d,
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

        pub fn ellipse_value(ellipse: &HandleGeom2dEllipse, u: f64) -> UniquePtr<GpPoint2d>;

        // Points
        #[cxx_name = "gp_Pnt"]
        type GpPoint;
        #[cxx_name = "gp_Pnt2d"]
        type GpPoint2d;

        #[cxx_name = "construct_unique"]
        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<GpPoint>;

        #[cxx_name = "X"]
        pub fn x(self: &GpPoint) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &GpPoint) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &GpPoint) -> f64;
        #[cxx_name = "Distance"]
        pub fn distance(self: &GpPoint, other: &GpPoint) -> f64;
        #[cxx_name = "Transform"]
        pub fn transform(self: Pin<&mut GpPoint>, transform: &GpTransform);

        #[cxx_name = "construct_unique"]
        pub fn new_point_2d(x: f64, y: f64) -> UniquePtr<GpPoint2d>;

        #[cxx_name = "X"]
        pub fn x(self: &GpPoint2d) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &GpPoint2d) -> f64;
        #[cxx_name = "Distance"]
        pub fn distance(self: &GpPoint2d, other: &GpPoint2d) -> f64;

        #[cxx_name = "gp_Vec"]
        type GpVec;

        #[cxx_name = "construct_unique"]
        pub fn new_vec(x: f64, y: f64, z: f64) -> UniquePtr<GpVec>;

        #[cxx_name = "X"]
        pub fn x(self: &GpVec) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &GpVec) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &GpVec) -> f64;

        // Edge types
        #[cxx_name = "GeomAbs_CurveType"]
        type GeomAbsCurveType;

        // Segments
        #[cxx_name = "GC_MakeSegment"]
        type GCMakeSegment;
        #[cxx_name = "GCE2d_MakeSegment"]
        type GCE2dMakeSegment;

        #[cxx_name = "construct_unique"]
        pub fn GCMakeSegment_point_point(p1: &GpPoint, p2: &GpPoint) -> UniquePtr<GCMakeSegment>;

        pub fn GCMakeSegment_Value(arc: &GCMakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;
        pub fn GCE2dMakeSegment_point_point(
            p1: &GpPoint2d,
            p2: &GpPoint2d,
        ) -> UniquePtr<HandleGeom2dTrimmedCurve>;

        // Lines
        #[cxx_name = "gp_Lin"]
        type GpLine;

        #[cxx_name = "construct_unique"]
        pub fn GpLine_ctor(point: &GpPoint, dir: &GpDir) -> UniquePtr<GpLine>;

        // Arcs
        #[cxx_name = "GC_MakeArcOfCircle"]
        type GCMakeArcOfCircle;

        #[cxx_name = "construct_unique"]
        pub fn GCMakeArcOfCircle_point_point_point(
            p1: &GpPoint,
            p2: &GpPoint,
            p3: &GpPoint,
        ) -> UniquePtr<GCMakeArcOfCircle>;

        pub fn GCMakeArcOfCircle_Value(
            arc: &GCMakeArcOfCircle,
        ) -> UniquePtr<HandleGeomTrimmedCurve>;

        // Circles
        #[cxx_name = "gp_Circ"]
        type GpCircle;

        #[cxx_name = "construct_unique"]
        pub fn GpCircle_ctor(axis: &GpAx2, radius: f64) -> UniquePtr<GpCircle>;

        // Shapes
        #[cxx_name = "TopoDS_Vertex"]
        type TopoDSVertex;
        #[cxx_name = "TopoDS_Edge"]
        type TopoDSEdge;
        #[cxx_name = "TopoDS_Wire"]
        type TopoDSWire;
        #[cxx_name = "TopoDS_Face"]
        type TopoDSFace;
        #[cxx_name = "TopoDS_Shell"]
        type TopoDSShell;
        #[cxx_name = "TopoDS_Solid"]
        type TopoDSSolid;
        #[cxx_name = "TopoDS_Shape"]
        type TopoDSShape;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSFace_ctor() -> UniquePtr<TopoDSFace>;

        pub fn cast_vertex_to_shape(wire: &TopoDSVertex) -> &TopoDSShape;
        pub fn cast_edge_to_shape(wire: &TopoDSEdge) -> &TopoDSShape;
        pub fn cast_wire_to_shape(wire: &TopoDSWire) -> &TopoDSShape;
        pub fn cast_face_to_shape(wire: &TopoDSFace) -> &TopoDSShape;
        pub fn cast_shell_to_shape(wire: &TopoDSShell) -> &TopoDSShape;
        pub fn cast_solid_to_shape(wire: &TopoDSSolid) -> &TopoDSShape;
        pub fn cast_compound_to_shape(wire: &TopoDSCompound) -> &TopoDSShape;

        pub fn TopoDS_cast_to_vertex(shape: &TopoDSShape) -> &TopoDSVertex;
        pub fn TopoDS_cast_to_wire(shape: &TopoDSShape) -> &TopoDSWire;
        pub fn TopoDS_cast_to_edge(shape: &TopoDSShape) -> &TopoDSEdge;
        pub fn TopoDS_cast_to_face(shape: &TopoDSShape) -> &TopoDSFace;
        pub fn TopoDS_cast_to_shell(shape: &TopoDSShape) -> &TopoDSShell;
        pub fn TopoDS_cast_to_solid(shape: &TopoDSShape) -> &TopoDSSolid;
        pub fn TopoDS_cast_to_compound(shape: &TopoDSShape) -> &TopoDSCompound;

        #[cxx_name = "Move"]
        pub fn translate(
            self: Pin<&mut TopoDSShape>,
            position: &TopLocLocation,
            raise_exception: bool,
        );

        #[cxx_name = "Location"]
        pub fn set_global_translation(
            self: Pin<&mut TopoDSShape>,
            translation: &TopLocLocation,
            raise_exception: bool,
        );

        #[cxx_name = "construct_unique"]
        pub fn TopoDSVertex_to_owned(shape: &TopoDSVertex) -> UniquePtr<TopoDSVertex>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSEdge_to_owned(shape: &TopoDSEdge) -> UniquePtr<TopoDSEdge>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSWire_to_owned(shape: &TopoDSWire) -> UniquePtr<TopoDSWire>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSFace_to_owned(shape: &TopoDSFace) -> UniquePtr<TopoDSFace>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSShell_to_owned(shape: &TopoDSShell) -> UniquePtr<TopoDSShell>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSSolid_to_owned(shape: &TopoDSSolid) -> UniquePtr<TopoDSSolid>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSCompound_to_owned(shape: &TopoDSCompound) -> UniquePtr<TopoDSCompound>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSShape_to_owned(shape: &TopoDSShape) -> UniquePtr<TopoDSShape>;

        #[cxx_name = "IsNull"]
        pub fn is_null(self: &TopoDSShape) -> bool;
        #[cxx_name = "IsEqual"]
        pub fn is_equal(self: &TopoDSShape, other: &TopoDSShape) -> bool;
        #[cxx_name = "ShapeType"]
        pub fn shape_type(self: &TopoDSShape) -> TopAbsShapeEnum;

        #[cxx_name = "TopAbs_Orientation"]
        type TopAbsOrientation;
        #[cxx_name = "Orientation"]
        pub fn orientation(self: &TopoDSShape) -> TopAbsOrientation;
        #[cxx_name = "Orientation"]
        pub fn orientation(self: &TopoDSFace) -> TopAbsOrientation;

        // Compound Shapes
        #[cxx_name = "TopoDS_Compound"]
        type TopoDSCompound;
        pub fn TopoDSCompound_as_shape(
            compound: UniquePtr<TopoDSCompound>,
        ) -> UniquePtr<TopoDSShape>;

        pub fn TopoDSShell_as_shape(shell: UniquePtr<TopoDSShell>) -> UniquePtr<TopoDSShape>;

        #[cxx_name = "BRep_Builder"]
        type BRepBuilder;
        #[cxx_name = "TopoDS_Builder"]
        type TopoDSBuilder;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSCompound_ctor() -> UniquePtr<TopoDSCompound>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDSShell_ctor() -> UniquePtr<TopoDSShell>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilder_ctor() -> UniquePtr<BRepBuilder>;

        pub fn BRepBuilder_upcast_to_topodsbuilder(builder: &BRepBuilder) -> &TopoDSBuilder;
        #[cxx_name = "MakeCompound"]
        pub fn make_compound(self: &TopoDSBuilder, compound: Pin<&mut TopoDSCompound>);
        #[cxx_name = "MakeShell"]
        pub fn make_shell(self: &TopoDSBuilder, compound: Pin<&mut TopoDSShell>);
        #[cxx_name = "Add"]
        pub fn add(self: &TopoDSBuilder, shape: Pin<&mut TopoDSShape>, compound: &TopoDSShape);

        // BRepBuilder
        #[cxx_name = "BRepBuilderAPI_MakeVertex"]
        type BRepBuilderAPIMakeVertex;
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIMakeVertex_GpPoint(
            point: &GpPoint,
        ) -> UniquePtr<BRepBuilderAPIMakeVertex>;

        #[cxx_name = "Vertex"]
        pub fn vertex(self: Pin<&mut BRepBuilderAPIMakeVertex>) -> &TopoDSVertex;

        #[cxx_name = "BRepBuilderAPI_MakeEdge"]
        type BRepBuilderAPI_MakeEdge;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_circle(
            circle: &GpCircle,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_GpPoint_GpPoint(
            p1: &GpPoint,
            p2: &GpPoint,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeEdge_CurveSurface2d(
            curve_handle: &HandleGeom2dCurve,
            surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;

        #[cxx_name = "Vertex1"]
        pub fn vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDSVertex;
        #[cxx_name = "Edge"]
        pub fn edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDSEdge;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeEdge>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeEdge) -> bool;

        type BRepBuilderAPI_MakeWire;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_ctor() -> UniquePtr<BRepBuilderAPI_MakeWire>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_edge_edge(
            edge_1: &TopoDSEdge,
            edge_2: &TopoDSEdge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeWire_edge_edge_edge(
            edge_1: &TopoDSEdge,
            edge_2: &TopoDSEdge,
            edge_3: &TopoDSEdge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDSShape;
        #[cxx_name = "Wire"]
        pub fn wire(self: Pin<&mut BRepBuilderAPI_MakeWire>) -> &TopoDSWire;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPI_MakeWire>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPI_MakeWire) -> bool;

        #[cxx_name = "BRepBuilderAPI_MakeFace"]
        type BRepBuilderAPIMakeFace;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIMakeFace_wire(
            wire: &TopoDSWire,
            only_plane: bool,
        ) -> UniquePtr<BRepBuilderAPIMakeFace>;
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIMakeFace_surface(
            surface: &HandleGeomSurface,
            edge_tolerance: f64,
        ) -> UniquePtr<BRepBuilderAPIMakeFace>;

        #[cxx_name = "Face"]
        pub fn face(self: &BRepBuilderAPIMakeFace) -> &TopoDSFace;
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPIMakeFace>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPIMakeFace>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPIMakeFace) -> bool;

        // BRepAdaptor
        #[cxx_name = "BRepAdaptor_Curve"]
        type BRepAdaptorCurve;

        #[cxx_name = "construct_unique"]
        pub fn BRepAdaptorCurve_ctor(edge: &TopoDSEdge) -> UniquePtr<BRepAdaptorCurve>;
        #[cxx_name = "FirstParameter"]
        pub fn first_parameter(self: &BRepAdaptorCurve) -> f64;
        #[cxx_name = "LastParameter"]
        pub fn last_parameter(self: &BRepAdaptorCurve) -> f64;
        pub fn BRepAdaptorCurve_value(curve: &BRepAdaptorCurve, u: f64) -> UniquePtr<GpPoint>;
        #[cxx_name = "GetType"]
        pub fn get_type(self: &BRepAdaptorCurve) -> GeomAbsCurveType;

        // Primitives
        #[cxx_name = "BRepPrimAPI_MakePrism"]
        type BRepPrimAPIMakePrism;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPIMakePrism_ctor(
            shape: &TopoDSShape,
            vec: &GpVec,
            copy: bool,
            canonize: bool,
        ) -> UniquePtr<BRepPrimAPIMakePrism>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakePrism>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakePrism>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakePrism) -> bool;

        #[cxx_name = "BRepFeat_MakeDPrism"]
        type BRepFeatMakeDPrism;

        #[cxx_name = "construct_unique"]
        pub fn BRepFeatMakeDPrism_ctor(
            shape: &TopoDSShape,
            profile_base: &TopoDSFace,
            sketch_base: &TopoDSFace,
            angle: f64,
            fuse: i32, // 0 = subtractive, 1 = additive
            modify: bool,
        ) -> UniquePtr<BRepFeatMakeDPrism>;

        #[cxx_name = "Perform"]
        pub fn perform_until_face(self: Pin<&mut BRepFeatMakeDPrism>, until: &TopoDSShape);

        #[cxx_name = "Perform"]
        pub fn perform_with_height(self: Pin<&mut BRepFeatMakeDPrism>, height: f64);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFeatMakeDPrism>) -> &TopoDSShape;

        type BRepPrimAPI_MakeRevol;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeRevol_ctor(
            shape: &TopoDSShape,
            axis: &GpAx1,
            angle: f64,
            copy: bool,
        ) -> UniquePtr<BRepPrimAPI_MakeRevol>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPI_MakeRevol>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPI_MakeRevol>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPI_MakeRevol) -> bool;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepBuilderAPI_MakeWire>, edge: &TopoDSEdge);

        #[rust_name = "add_wire"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepBuilderAPI_MakeWire>, wire: &TopoDSWire);

        #[cxx_name = "BRepPrimAPI_MakeCylinder"]
        type BRepPrimAPIMakeCylinder;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPIMakeCylinder_ctor(
            coord_system: &GpAx2,
            radius: f64,
            height: f64,
        ) -> UniquePtr<BRepPrimAPIMakeCylinder>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakeCylinder>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakeCylinder>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakeCylinder) -> bool;

        #[cxx_name = "BRepPrimAPI_MakeBox"]
        type BRepPrimAPIMakeBox;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPIMakeBox_ctor(
            point: &GpPoint,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<BRepPrimAPIMakeBox>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakeBox>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakeBox>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakeBox) -> bool;

        #[cxx_name = "BRepPrimAPI_MakeSphere"]
        type BRepPrimAPIMakeSphere;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeSphere_ctor(
            axis: &GpAx2,
            r: f64,
            angle_1: f64,
        ) -> UniquePtr<BRepPrimAPIMakeSphere>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakeSphere>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakeSphere>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakeSphere) -> bool;

        #[cxx_name = "BRepPrimAPI_MakeCone"]
        type BRepPrimAPIMakeCone;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPIMakeCone_ctor(
            axis: &GpAx2,
            r1: f64,
            r2: f64,
            h: f64,
            angle: f64,
        ) -> UniquePtr<BRepPrimAPIMakeCone>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakeCone>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakeCone>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakeCone) -> bool;

        #[cxx_name = "BRepPrimAPI_MakeTorus"]
        type BRepPrimAPIMakeTorus;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPIMakeTorus_ctor(
            axis: &GpAx2,
            r1: f64,
            r2: f64,
            angle_1: f64,
            angle_2: f64,
            angle_3: f64,
        ) -> UniquePtr<BRepPrimAPIMakeTorus>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepPrimAPIMakeTorus>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepPrimAPIMakeTorus>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepPrimAPIMakeTorus) -> bool;

        // BRepLib
        pub fn BRepLibBuildCurves3d(shape: &TopoDSShape) -> bool;

        // Fillets
        #[cxx_name = "BRepFilletAPI_MakeFillet"]
        type BRepFilletAPIMakeFillet;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPIMakeFillet_ctor(
            shape: &TopoDSShape,
        ) -> UniquePtr<BRepFilletAPIMakeFillet>;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepFilletAPIMakeFillet>, radius: f64, edge: &TopoDSEdge);

        #[rust_name = "variable_add_edge"]
        pub fn Add(
            self: Pin<&mut BRepFilletAPIMakeFillet>,
            radius_values: &TColgpArray1OfPnt2d,
            edge: &TopoDSEdge,
        );

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPIMakeFillet>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPIMakeFillet>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPIMakeFillet) -> bool;

        #[cxx_name = "BRepFilletAPI_MakeFillet2d"]
        type BRepFilletAPIMakeFillet2d;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPIMakeFillet2d_ctor(
            face: &TopoDSFace,
        ) -> UniquePtr<BRepFilletAPIMakeFillet2d>;

        pub fn BRepFilletAPIMakeFillet2d_add_fillet(
            make_fillet: Pin<&mut BRepFilletAPIMakeFillet2d>,
            vertex: &TopoDSVertex,
            radius: f64,
        ) -> UniquePtr<TopoDSEdge>;
        pub fn BRepFilletAPIMakeFillet2d_add_chamfer(
            make_fillet: Pin<&mut BRepFilletAPIMakeFillet2d>,
            edge1: &TopoDSEdge,
            edge2: &TopoDSEdge,
            distance1: f64,
            distance2: f64,
        ) -> UniquePtr<TopoDSEdge>;
        pub fn BRepFilletAPIMakeFillet2d_add_chamfer_angle(
            make_fillet: Pin<&mut BRepFilletAPIMakeFillet2d>,
            edge: &TopoDSEdge,
            vertex: &TopoDSVertex,
            distance: f64,
            angle: f64,
        ) -> UniquePtr<TopoDSEdge>;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPIMakeFillet2d>, progress: &MessageProgressRange);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPIMakeFillet2d>) -> &TopoDSShape;
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPIMakeFillet2d) -> bool;

        // Chamfers
        #[cxx_name = "BRepFilletAPI_MakeChamfer"]
        type BRepFilletAPIMakeChamfer;

        #[cxx_name = "construct_unique"]
        pub fn BRepFilletAPI_MakeChamfer_ctor(
            shape: &TopoDSShape,
        ) -> UniquePtr<BRepFilletAPIMakeChamfer>;

        #[rust_name = "add_edge"]
        #[cxx_name = "Add"]
        pub fn add(self: Pin<&mut BRepFilletAPIMakeChamfer>, distance: f64, edge: &TopoDSEdge);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepFilletAPIMakeChamfer>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFilletAPIMakeChamfer>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepFilletAPIMakeChamfer) -> bool;

        // Offset
        #[cxx_name = "BRepOffsetAPI_MakeOffset"]
        type BRepOffsetAPIMakeOffset;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIMakeOffset_face_ctor(
            face: &TopoDSFace,
            join: GeomAbsJoinType,
        ) -> UniquePtr<BRepOffsetAPIMakeOffset>;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIMakeOffset_wire_ctor(
            wire: &TopoDSWire,
            join: GeomAbsJoinType,
        ) -> UniquePtr<BRepOffsetAPIMakeOffset>;

        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut BRepOffsetAPIMakeOffset>, offset: f64, alt: f64);

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPIMakeOffset>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPIMakeOffset>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPIMakeOffset) -> bool;

        #[cxx_name = "GeomAbs_JoinType"]
        type GeomAbsJoinType;

        // Solids
        #[cxx_name = "BRepOffsetAPI_MakeThickSolid"]
        type BRepOffsetAPIMakeThickSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIMakeThickSolid_ctor() -> UniquePtr<BRepOffsetAPIMakeThickSolid>;

        pub fn MakeThickSolidByJoin(
            make_thick_solid: Pin<&mut BRepOffsetAPIMakeThickSolid>,
            shape: &TopoDSShape,
            closing_faces: &TopToolsListOfShape,
            offset: f64,
            tolerance: f64,
        );
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPIMakeThickSolid>) -> &TopoDSShape;
        pub fn Build(self: Pin<&mut BRepOffsetAPIMakeThickSolid>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPIMakeThickSolid) -> bool;

        // Sweeps
        #[cxx_name = "BRepOffsetAPI_MakePipe"]
        type BRepOffsetAPIMakePipe;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIMakePipe_ctor(
            spine: &TopoDSWire,
            profile: &TopoDSShape,
        ) -> UniquePtr<BRepOffsetAPIMakePipe>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPIMakePipe>) -> &TopoDSShape;

        // Sweeps with a law function
        #[cxx_name = "BRepOffsetAPI_MakePipeShell"]
        type BRepOffsetAPIMakePipeShell;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIMakePipeShell_ctor(
            spine: &TopoDSWire,
        ) -> UniquePtr<BRepOffsetAPIMakePipeShell>;

        #[cxx_name = "SetMode"]
        pub fn set_mode(self: Pin<&mut BRepOffsetAPIMakePipeShell>, is_frenet: bool);

        pub fn Add(
            self: Pin<&mut BRepOffsetAPIMakePipeShell>,
            profile: &TopoDSShape,
            with_contact: bool,
            with_correction: bool,
        );

        pub fn SetLaw(
            self: Pin<&mut BRepOffsetAPIMakePipeShell>,
            profile: &TopoDSShape,
            law: &HandleLawFunction,
            with_contact: bool,
            with_correction: bool,
        );

        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPIMakePipeShell>, progress: &MessageProgressRange);
        #[cxx_name = "MakeSolid"]
        pub fn make_solid(self: Pin<&mut BRepOffsetAPIMakePipeShell>) -> bool;
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPIMakePipeShell>) -> &TopoDSShape;

        // Lofting
        #[cxx_name = "BRepOffsetAPI_ThruSections"]
        type BRepOffsetAPIThruSections;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPIThruSections_ctor(
            is_solid: bool,
        ) -> UniquePtr<BRepOffsetAPIThruSections>;

        #[cxx_name = "AddWire"]
        pub fn add_wire(self: Pin<&mut BRepOffsetAPIThruSections>, wire: &TopoDSWire);
        #[cxx_name = "CheckCompatibility"]
        pub fn check_compatibility(self: Pin<&mut BRepOffsetAPIThruSections>, check: bool);
        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepOffsetAPIThruSections>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepOffsetAPIThruSections>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepOffsetAPIThruSections) -> bool;

        // Boolean Operations
        #[cxx_name = "BRepAlgoAPI_Fuse"]
        type BRepAlgoAPIFuse;
        #[cxx_name = "BOPAlgo_GlueEnum"]
        type BOPAlgoGlueEnum;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPIFuse_ctor(
            shape_1: &TopoDSShape,
            shape_2: &TopoDSShape,
        ) -> UniquePtr<BRepAlgoAPIFuse>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPIFuse>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPIFuse>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPIFuse) -> bool;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPIFuse>) -> &TopToolsListOfShape;
        #[cxx_name = "SetGlue"]
        pub fn set_glue(self: Pin<&mut BRepAlgoAPIFuse>, glue: BOPAlgoGlueEnum);

        #[cxx_name = "BRepAlgoAPI_Cut"]
        type BRepAlgoAPICut;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPICut_ctor(
            shape_1: &TopoDSShape,
            shape_2: &TopoDSShape,
        ) -> UniquePtr<BRepAlgoAPICut>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPICut>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPICut>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPICut) -> bool;
        pub fn Generated<'a>(
            self: Pin<&'a mut BRepAlgoAPICut>,
            shape: &'a TopoDSShape,
        ) -> &'a TopToolsListOfShape;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPICut>) -> &TopToolsListOfShape;

        #[cxx_name = "BRepAlgoAPI_Common"]
        type BRepAlgoAPICommon;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPICommon_ctor(
            shape_1: &TopoDSShape,
            shape_2: &TopoDSShape,
        ) -> UniquePtr<BRepAlgoAPICommon>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPICommon>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPICommon>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPICommon) -> bool;
        #[cxx_name = "SectionEdges"]
        pub fn section_edges(self: Pin<&mut BRepAlgoAPICommon>) -> &TopToolsListOfShape;

        #[cxx_name = "BRepAlgoAPI_Section"]
        type BRepAlgoAPISection;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPISection_ctor(
            shape_1: &TopoDSShape,
            shape_2: &TopoDSShape,
        ) -> UniquePtr<BRepAlgoAPISection>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepAlgoAPISection>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepAlgoAPISection>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepAlgoAPISection) -> bool;

        // Geometric processor
        #[cxx_name = "gp_Ax1"]
        type GpAx1;
        #[cxx_name = "gp_Ax2"]
        type GpAx2;
        #[cxx_name = "gp_Ax3"]
        type GpAx3;
        #[cxx_name = "gp_Dir"]
        type GpDir;
        #[cxx_name = "gp_Dir2d"]
        type GpDir2d;
        #[cxx_name = "gp_Ax2d"]
        type GpAx2d;
        pub fn gp_OX() -> &'static GpAx1;
        pub fn gp_OY() -> &'static GpAx1;
        pub fn gp_OZ() -> &'static GpAx1;
        pub fn gp_DZ() -> &'static GpDir;

        #[cxx_name = "X"]
        pub fn x(self: &GpDir) -> f64;
        #[cxx_name = "Y"]
        pub fn y(self: &GpDir) -> f64;
        #[cxx_name = "Z"]
        pub fn z(self: &GpDir) -> f64;

        #[cxx_name = "construct_unique"]
        pub fn GpAx1_ctor(origin: &GpPoint, main_dir: &GpDir) -> UniquePtr<GpAx1>;

        #[cxx_name = "construct_unique"]
        pub fn GpAx2_ctor(origin: &GpPoint, main_dir: &GpDir) -> UniquePtr<GpAx2>;

        #[cxx_name = "construct_unique"]
        pub fn GpAx3_from_GpAx2(axis: &GpAx2) -> UniquePtr<GpAx3>;

        #[cxx_name = "construct_unique"]
        pub fn GpDir_ctor(x: f64, y: f64, z: f64) -> UniquePtr<GpDir>;

        #[cxx_name = "construct_unique"]
        pub fn GpDir2d_ctor(x: f64, y: f64) -> UniquePtr<GpDir2d>;

        #[cxx_name = "construct_unique"]
        pub fn GpAx2d_ctor(point: &GpPoint2d, dir: &GpDir2d) -> UniquePtr<GpAx2d>;

        // Geometry Interpolation
        #[cxx_name = "GeomAPI_Interpolate"]
        type GeomAPIInterpolate;

        #[cxx_name = "construct_unique"]
        pub fn GeomAPIInterpolate_ctor(
            points: &HandleTColgpHArray1OfPnt,
            periodic: bool,
            tolerance: f64,
        ) -> UniquePtr<GeomAPIInterpolate>;

        #[cxx_name = "Load"]
        pub fn load(
            self: Pin<&mut GeomAPIInterpolate>,
            initial_tangent: &GpVec,
            final_tangent: &GpVec,
            scale: bool,
        );

        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut GeomAPIInterpolate>);

        pub fn GeomAPIInterpolate_Curve(
            interpolate: &GeomAPIInterpolate,
        ) -> UniquePtr<HandleGeomBSplineCurve>;

        // Geometry Querying
        #[cxx_name = "GeomAPI_ProjectPointOnSurf"]
        type GeomAPIProjectPointOnSurf;

        #[cxx_name = "construct_unique"]
        pub fn GeomAPIProjectPointOnSurf_ctor(
            origin: &GpPoint,
            surface: &HandleGeomSurface,
        ) -> UniquePtr<GeomAPIProjectPointOnSurf>;
        #[cxx_name = "LowerDistanceParameters"]
        pub fn lower_distance_parameters(
            self: &GeomAPIProjectPointOnSurf,
            u: &mut f64,
            v: &mut f64,
        );

        // Transforms
        #[cxx_name = "gp_Trsf"]
        type GpTransform;

        #[cxx_name = "construct_unique"]
        pub fn new_transform() -> UniquePtr<GpTransform>;

        #[rust_name = "set_mirror_axis"]
        #[cxx_name = "SetMirror"]
        pub fn set_mirror(self: Pin<&mut GpTransform>, axis: &GpAx1);
        #[cxx_name = "SetRotation"]
        pub fn set_rotation(self: Pin<&mut GpTransform>, axis: &GpAx1, angle: f64);
        #[cxx_name = "SetScale"]
        pub fn set_scale(self: Pin<&mut GpTransform>, point: &GpPoint, scale: f64);
        #[cxx_name = "SetTranslation"]
        pub fn set_translation(self: Pin<&mut GpTransform>, point1: &GpPoint, point2: &GpPoint);
        #[cxx_name = "Value"]
        pub fn value(self: &GpTransform, the_row: i32, the_col: i32) -> f64;

        #[cxx_name = "SetTranslationPart"]
        pub fn set_translation_vec(self: Pin<&mut GpTransform>, translation: &GpVec);

        #[cxx_name = "gp_GTrsf"]
        type GpGTrsf;
        #[cxx_name = "construct_unique"]
        pub fn new_GpGTrsf() -> UniquePtr<GpGTrsf>;
        #[cxx_name = "SetValue"]
        pub fn set_value(self: Pin<&mut GpGTrsf>, the_row: i32, the_col: i32, the_value: f64);
        #[cxx_name = "Value"]
        pub fn value(self: &GpGTrsf, the_row: i32, the_col: i32) -> f64;

        #[cxx_name = "BRepBuilderAPI_MakeSolid"]
        type BRepBuilderAPIMakeSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIMakeSolid_ctor(
            shell: &TopoDSShell,
        ) -> UniquePtr<BRepBuilderAPIMakeSolid>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPIMakeSolid>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPIMakeSolid>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPIMakeSolid) -> bool;

        #[cxx_name = "BRepBuilderAPI_MakeShapeOnMesh"]
        type BRepBuilderAPIMakeShapeOnMesh;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIMakeShapeOnMesh_ctor(
            mesh: &HandlePolyTriangulation,
        ) -> UniquePtr<BRepBuilderAPIMakeShapeOnMesh>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPIMakeShapeOnMesh>) -> &TopoDSShape;
        pub fn Build(
            self: Pin<&mut BRepBuilderAPIMakeShapeOnMesh>,
            progress: &MessageProgressRange,
        );
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPIMakeShapeOnMesh) -> bool;

        #[cxx_name = "BRepBuilderAPI_Transform"]
        type BRepBuilderAPITransform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPITransform_ctor(
            shape: &TopoDSShape,
            transform: &GpTransform,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPITransform>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPITransform>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPITransform>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPITransform) -> bool;

        #[cxx_name = "BRepBuilderAPI_GTransform"]
        type BRepBuilderAPIGTransform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPIGTransform_ctor(
            shape: &TopoDSShape,
            transform: &GpGTrsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPIGTransform>;

        #[cxx_name = "Shape"]
        pub fn shape(self: Pin<&mut BRepBuilderAPIGTransform>) -> &TopoDSShape;
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepBuilderAPIGTransform>, progress: &MessageProgressRange);
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepBuilderAPIGTransform) -> bool;

        // Topology Explorer
        #[cxx_name = "TopExp_Explorer"]
        type TopExpExplorer;

        #[cxx_name = "TopAbs_ShapeEnum"]
        type TopAbsShapeEnum;

        #[cxx_name = "construct_unique"]
        pub fn TopExpExplorer_ctor(
            shape: &TopoDSShape,
            to_find: TopAbsShapeEnum,
        ) -> UniquePtr<TopExpExplorer>;

        #[cxx_name = "More"]
        pub fn more(self: &TopExpExplorer) -> bool;
        #[cxx_name = "Next"]
        pub fn next(self: Pin<&mut TopExpExplorer>);
        pub fn ExplorerCurrentShape(explorer: &TopExpExplorer) -> UniquePtr<TopoDSShape>;
        #[cxx_name = "Current"]
        pub fn current(self: &TopExpExplorer) -> &TopoDSShape;

        pub fn TopExp_FirstVertex(edge: &TopoDSEdge) -> UniquePtr<TopoDSVertex>;
        pub fn TopExp_LastVertex(edge: &TopoDSEdge) -> UniquePtr<TopoDSVertex>;
        pub fn TopExp_EdgeVertices(
            edge: &TopoDSEdge,
            vertex_first: Pin<&mut TopoDSVertex>,
            vertex_last: Pin<&mut TopoDSVertex>,
        );
        pub fn TopExp_WireVertices(
            wire: &TopoDSWire,
            vertex_first: Pin<&mut TopoDSVertex>,
            vertex_last: Pin<&mut TopoDSVertex>,
        );
        pub fn TopExp_CommonVertex(
            edge_1: &TopoDSEdge,
            edge_2: &TopoDSEdge,
            vertex: Pin<&mut TopoDSVertex>,
        ) -> bool;

        pub fn BRep_Tool_Surface(face: &TopoDSFace) -> UniquePtr<HandleGeomSurface>;
        pub fn BRep_Tool_Curve(
            edge: &TopoDSEdge,
            first: &mut f64,
            last: &mut f64,
        ) -> UniquePtr<HandleGeomCurve>;
        pub fn BRep_Tool_Pnt(vertex: &TopoDSVertex) -> UniquePtr<GpPoint>;
        pub fn BRep_Tool_Triangulation(
            face: &TopoDSFace,
            location: Pin<&mut TopLocLocation>,
        ) -> UniquePtr<HandlePolyTriangulation>;

        #[cxx_name = "BRepIntCurveSurface_Inter"]
        type BRepIntCurveSurfaceInter;

        #[cxx_name = "construct_unique"]
        pub fn BRepIntCurveSurfaceInter_ctor() -> UniquePtr<BRepIntCurveSurfaceInter>;
        pub fn Init(
            self: Pin<&mut BRepIntCurveSurfaceInter>,
            shape: &TopoDSShape,
            line: &GpLine,
            tolerance: f64,
        );
        #[cxx_name = "More"]
        pub fn more(self: &BRepIntCurveSurfaceInter) -> bool;
        #[cxx_name = "Next"]
        pub fn next(self: Pin<&mut BRepIntCurveSurfaceInter>);
        pub fn BRepIntCurveSurfaceInter_face(
            intersector: &BRepIntCurveSurfaceInter,
        ) -> UniquePtr<TopoDSFace>;
        pub fn BRepIntCurveSurfaceInter_point(
            intersector: &BRepIntCurveSurfaceInter,
        ) -> UniquePtr<GpPoint>;
        #[cxx_name = "U"]
        pub fn u(self: &BRepIntCurveSurfaceInter) -> f64;
        #[cxx_name = "V"]
        pub fn v(self: &BRepIntCurveSurfaceInter) -> f64;
        #[cxx_name = "W"]
        pub fn w(self: &BRepIntCurveSurfaceInter) -> f64;

        // BRepFeat
        #[cxx_name = "BRepFeat_MakeCylindricalHole"]
        type BRepFeatMakeCylindricalHole;
        #[cxx_name = "construct_unique"]
        pub fn BRepFeatMakeCylindricalHole_ctor() -> UniquePtr<BRepFeatMakeCylindricalHole>;
        pub fn Init(self: Pin<&mut BRepFeatMakeCylindricalHole>, shape: &TopoDSShape, axis: &GpAx1);
        #[cxx_name = "Perform"]
        pub fn perform(self: Pin<&mut BRepFeatMakeCylindricalHole>, radius: f64);
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut BRepFeatMakeCylindricalHole>);
        #[cxx_name = "Shape"]
        pub fn shape(self: &BRepFeatMakeCylindricalHole) -> &TopoDSShape;

        // Data Import
        #[cxx_name = "STEPControl_Reader"]
        type STEPControlReader;

        #[cxx_name = "IFSelect_ReturnStatus"]
        type IFSelectReturnStatus;

        #[cxx_name = "construct_unique"]
        pub fn STEPControlReader_ctor() -> UniquePtr<STEPControlReader>;

        pub fn read_step(
            reader: Pin<&mut STEPControlReader>,
            filename: String,
        ) -> IFSelectReturnStatus;
        pub fn TransferRoots(
            self: Pin<&mut STEPControlReader>,
            progress: &MessageProgressRange,
        ) -> i32;
        pub fn one_shape(reader: &STEPControlReader) -> UniquePtr<TopoDSShape>;

        // Data Export
        #[cxx_name = "STEPControl_Writer"]
        type STEPControlWriter;

        #[cxx_name = "construct_unique"]
        pub fn STEPControlWriter_ctor() -> UniquePtr<STEPControlWriter>;

        pub fn transfer_shape(
            writer: Pin<&mut STEPControlWriter>,
            shape: &TopoDSShape,
        ) -> IFSelectReturnStatus;
        pub fn write_step(
            writer: Pin<&mut STEPControlWriter>,
            filename: String,
        ) -> IFSelectReturnStatus;

        #[cxx_name = "StlAPI_Writer"]
        type StlAPIWriter;

        #[cxx_name = "construct_unique"]
        pub fn StlAPIWriter_ctor() -> UniquePtr<StlAPIWriter>;

        #[cxx_name = "WriteStl"]
        pub fn write_stl(
            writer: Pin<&mut StlAPIWriter>,
            shape: &TopoDSShape,
            filename: String,
        ) -> bool;

        // Triangulation
        #[cxx_name = "BRepMesh_IncrementalMesh"]
        type BRepMeshIncrementalMesh;

        #[cxx_name = "construct_unique"]
        pub fn BRepMeshIncrementalMesh_ctor(
            shape: &TopoDSShape,
            deflection: f64,
        ) -> UniquePtr<BRepMeshIncrementalMesh>;

        #[cxx_name = "Shape"]
        pub fn shape(self: &BRepMeshIncrementalMesh) -> &TopoDSShape;
        #[cxx_name = "IsDone"]
        pub fn is_done(self: &BRepMeshIncrementalMesh) -> bool;

        #[cxx_name = "TopLoc_Location"]
        type TopLocLocation;
        #[cxx_name = "construct_unique"]
        pub fn TopLocLocation_ctor() -> UniquePtr<TopLocLocation>;

        #[cxx_name = "construct_unique"]
        pub fn TopLocLocation_from_transform(transform: &GpTransform) -> UniquePtr<TopLocLocation>;

        pub fn TopLocLocation_Transformation(location: &TopLocLocation) -> UniquePtr<GpTransform>;

        #[cxx_name = "Handle_Poly_Triangulation"]
        type HandlePolyTriangulation;

        pub fn HandlePolyTriangulation_ctor(
            triangulation: UniquePtr<PolyTriangulation>,
        ) -> UniquePtr<HandlePolyTriangulation>;

        #[cxx_name = "IsNull"]
        pub fn is_null(self: &HandlePolyTriangulation) -> bool;
        #[cxx_name = "handle_try_deref"]
        pub fn HandlePolyTriangulation_Get(
            handle: &HandlePolyTriangulation,
        ) -> Result<&PolyTriangulation>;

        #[cxx_name = "Poly_Triangulation"]
        type PolyTriangulation;
        #[cxx_name = "construct_unique"]
        pub fn PolyTriangulation_ctor(
            nb_nodes: i32,
            nb_triangles: i32,
            has_uv: bool,
            has_normals: bool,
        ) -> UniquePtr<PolyTriangulation>;
        #[cxx_name = "NbNodes"]
        pub fn nb_nodes(self: &PolyTriangulation) -> i32;
        #[cxx_name = "NbTriangles"]
        pub fn nb_triangles(self: &PolyTriangulation) -> i32;
        #[cxx_name = "HasNormals"]
        pub fn has_normals(self: &PolyTriangulation) -> bool;
        #[cxx_name = "HasUVNodes"]
        pub fn has_uv_nodes(self: &PolyTriangulation) -> bool;
        #[cxx_name = "Triangle"]
        pub fn triangle(self: &PolyTriangulation, index: i32) -> &PolyTriangle;
        pub fn SetTriangle(self: Pin<&mut PolyTriangulation>, index: i32, triangle: &PolyTriangle);
        #[cxx_name = "SetNode"]
        pub fn set_node(self: Pin<&mut PolyTriangulation>, index: i32, node: &GpPoint);
        #[cxx_name = "SetNormal"]
        pub fn set_normal(self: Pin<&mut PolyTriangulation>, index: i32, dir: &GpDir);
        #[cxx_name = "SetUVNode"]
        pub fn set_uv_node(self: Pin<&mut PolyTriangulation>, index: i32, uv: &GpPoint2d);
        pub fn PolyTriangulation_Normal(
            triangulation: &PolyTriangulation,
            index: i32,
        ) -> UniquePtr<GpDir>;
        pub fn PolyTriangulation_Node(
            triangulation: &PolyTriangulation,
            index: i32,
        ) -> UniquePtr<GpPoint>;
        pub fn PolyTriangulation_UV(
            triangulation: &PolyTriangulation,
            index: i32,
        ) -> UniquePtr<GpPoint2d>;

        #[cxx_name = "Poly_Triangle"]
        type PolyTriangle;
        #[cxx_name = "construct_unique"]
        pub fn PolyTriangle_ctor(node1: i32, node2: i32, node3: i32) -> UniquePtr<PolyTriangle>;
        #[cxx_name = "Value"]
        pub fn value(self: &PolyTriangle, index: i32) -> i32;

        #[cxx_name = "Poly_Connect"]
        type PolyConnect;
        #[cxx_name = "construct_unique"]
        pub fn PolyConnect_ctor(triangulation: &HandlePolyTriangulation) -> UniquePtr<PolyConnect>;

        pub fn compute_normals(face: &TopoDSFace, triangulation: &HandlePolyTriangulation);

        // Edge approximation
        #[cxx_name = "GCPnts_TangentialDeflection"]
        type GCPntsTangentialDeflection;

        #[cxx_name = "construct_unique"]
        pub fn GCPntsTangentialDeflection_ctor(
            curve: &BRepAdaptorCurve,
            angular_deflection: f64,
            curvature_deflection: f64,
        ) -> UniquePtr<GCPntsTangentialDeflection>;
        #[cxx_name = "NbPoints"]
        pub fn nb_points(self: &GCPntsTangentialDeflection) -> i32;
        pub fn GCPntsTangentialDeflection_Value(
            approximator: &GCPntsTangentialDeflection,
            index: i32,
        ) -> UniquePtr<GpPoint>;

        // Shape Properties
        #[cxx_name = "GProp_GProps"]
        type GPropGProps;
        #[cxx_name = "construct_unique"]
        pub fn GPropGProps_ctor() -> UniquePtr<GPropGProps>;
        #[cxx_name = "Mass"]
        pub fn mass(self: &GPropGProps) -> f64;
        #[cxx_name = "StaticMoments"]
        pub fn static_moments(self: &GPropGProps, lx: &mut f64, ly: &mut f64, lz: &mut f64);
        #[cxx_name = "MomentOfInertia"]
        pub fn moment_of_inertia(self: &GPropGProps, axis: &GpAx1) -> f64;
        #[cxx_name = "RadiusOfGyration"]
        pub fn radius_of_gyration(self: &GPropGProps, axis: &GpAx1) -> f64;
        pub fn GPropGProps_CentreOfMass(props: &GPropGProps) -> UniquePtr<GpPoint>;

        pub fn BRepGProp_LinearProperties(shape: &TopoDSShape, props: Pin<&mut GPropGProps>);
        pub fn BRepGProp_SurfaceProperties(shape: &TopoDSShape, props: Pin<&mut GPropGProps>);
        pub fn BRepGProp_VolumeProperties(shape: &TopoDSShape, props: Pin<&mut GPropGProps>);

        #[cxx_name = "BRepGProp_Face"]
        type BRepGPropFace;

        #[cxx_name = "construct_unique"]
        pub fn BRepGPropFace_ctor(face: &TopoDSFace) -> UniquePtr<BRepGPropFace>;
        pub fn Normal(
            self: &BRepGPropFace,
            u: f64,
            v: f64,
            point: Pin<&mut GpPoint>,
            normal: Pin<&mut GpVec>,
        );

        // BRepTools
        pub fn outer_wire(face: &TopoDSFace) -> UniquePtr<TopoDSWire>;

        // Cleaning
        #[cxx_name = "ShapeUpgrade_UnifySameDomain"]
        type ShapeUpgradeUnifySameDomain;

        #[cxx_name = "construct_unique"]
        pub fn ShapeUpgradeUnifySameDomain_ctor(
            shape: &TopoDSShape,
            unify_edges: bool,
            unify_faces: bool,
            concat_b_splines: bool,
        ) -> UniquePtr<ShapeUpgradeUnifySameDomain>;
        #[cxx_name = "AllowInternalEdges"]
        pub fn allow_internal_edges(self: Pin<&mut ShapeUpgradeUnifySameDomain>, allow: bool);
        #[cxx_name = "Build"]
        pub fn build(self: Pin<&mut ShapeUpgradeUnifySameDomain>);
        #[cxx_name = "Shape"]
        pub fn shape(self: &ShapeUpgradeUnifySameDomain) -> &TopoDSShape;

        pub fn connect_edges_to_wires(
            edges: Pin<&mut HandleTopToolsHSequenceOfShape>,
            tolerance: f64,
            shared: bool,
            wires: Pin<&mut HandleTopToolsHSequenceOfShape>,
        );
    }
}

// Gross, but is this okay?
unsafe impl Send for ffi::BRepBuilderAPI_MakeWire {}
unsafe impl Send for ffi::TopoDSEdge {}
unsafe impl Send for ffi::TopoDSWire {}
unsafe impl Send for ffi::TopoDSFace {}
unsafe impl Send for ffi::TopoDSShell {}
unsafe impl Send for ffi::TopoDSSolid {}
unsafe impl Send for ffi::TopoDSCompound {}
unsafe impl Send for ffi::TopoDSShape {}

unsafe impl Send for ffi::TopExpExplorer {}
unsafe impl Send for ffi::BRepFilletAPIMakeChamfer {}
