pub mod b_rep_bnd_lib;
pub mod b_rep_g_prop;
pub mod b_rep_mesh;
pub mod b_rep_tools;
pub mod bin_tools;
pub mod bnd;
pub mod g_prop;
pub mod gc_pnts;
pub mod geom;
pub mod geom2d;
pub mod geom_api;
pub mod gp;
pub mod law;
pub mod poly;
pub mod shape_analysis;
pub mod shape_upgrade;
pub mod stl_api;
pub mod t_col_gp;
pub mod top_loc;
pub mod top_tools;

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

    #[derive(Debug)]
    #[repr(u32)]
    pub enum IFSelect_ReturnStatus {
        IFSelect_RetVoid,
        IFSelect_RetDone,
        IFSelect_RetError,
        IFSelect_RetFail,
        IFSelect_RetStop,
    }

    #[derive(Debug)]
    #[repr(u32)]
    pub enum BOPAlgo_GlueEnum {
        BOPAlgo_GlueOff,
        BOPAlgo_GlueShift,
        BOPAlgo_GlueFull,
    }

    #[derive(Debug)]
    #[repr(u32)]
    pub enum GeomAbs_CurveType {
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
    pub enum GeomAbs_JoinType {
        GeomAbs_Arc,
        GeomAbs_Tangent,
        GeomAbs_Intersection,
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
        type Handle_Geom_Curve;
        type Handle_Geom_BSplineCurve;
        type Handle_Geom_BezierCurve;
        type Handle_Geom_TrimmedCurve;
        type Handle_Geom_Surface;
        type Handle_Geom_BezierSurface;
        type Handle_Geom_Plane;
        type Handle_Geom2d_Curve;
        type Handle_Geom2d_Ellipse;
        type Handle_Geom2d_TrimmedCurve;
        type Handle_Geom_CylindricalSurface;
        type Handle_TopTools_HSequenceOfShape;
        type Handle_Law_Function;

        type Handle_TColgp_HArray1OfPnt;
        pub fn new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt(
            array: UniquePtr<TColgp_HArray1OfPnt>,
        ) -> UniquePtr<Handle_TColgp_HArray1OfPnt>;

        pub fn DynamicType(surface: &Handle_Geom_Surface) -> &HandleStandardType;
        pub fn type_name(handle: &HandleStandardType) -> String;

        pub fn IsNull(self: &HandleStandardType) -> bool;
        pub fn IsNull(self: &Handle_Geom_Curve) -> bool;
        pub fn IsNull(self: &Handle_Geom_TrimmedCurve) -> bool;
        pub fn IsNull(self: &Handle_Geom_Surface) -> bool;
        pub fn IsNull(self: &Handle_Geom_BezierSurface) -> bool;
        pub fn IsNull(self: &Handle_Geom_Plane) -> bool;
        pub fn IsNull(self: &Handle_Geom2d_Curve) -> bool;
        pub fn IsNull(self: &Handle_Geom2d_Ellipse) -> bool;
        pub fn IsNull(self: &Handle_Geom2d_TrimmedCurve) -> bool;
        pub fn IsNull(self: &Handle_Geom_CylindricalSurface) -> bool;
        pub fn IsNull(self: &Handle_TopTools_HSequenceOfShape) -> bool;

        pub fn HandleGeomCurve_Value(curve: &Handle_Geom_Curve, u: f64) -> UniquePtr<gp_Pnt>;

        // Types from sub-modules
        type TColgp_Array1OfPnt2d = crate::t_col_gp::TColgp_Array1OfPnt2d;
        type TColgp_Array2OfPnt = crate::t_col_gp::TColgp_Array2OfPnt;
        type TColgp_HArray1OfPnt = crate::t_col_gp::TColgp_HArray1OfPnt;
        type TopTools_ListOfShape = crate::top_tools::TopTools_ListOfShape;
        type TopTools_IndexedMapOfShape = crate::top_tools::TopTools_IndexedMapOfShape;
        type TopTools_IndexedDataMapOfShapeListOfShape =
            crate::top_tools::TopTools_IndexedDataMapOfShapeListOfShape;
        type TopTools_HSequenceOfShape = crate::top_tools::TopTools_HSequenceOfShape;

        type gp_Pnt = crate::gp::gp_Pnt;
        type gp_Pnt2d = crate::gp::gp_Pnt2d;
        type gp_Vec = crate::gp::gp_Vec;
        type gp_Dir = crate::gp::gp_Dir;

        type gp_Ax1 = crate::gp::gp_Ax1;
        type gp_Ax2 = crate::gp::gp_Ax2;
        type gp_Ax3 = crate::gp::gp_Ax3;
        type gp_Dir2d = crate::gp::gp_Dir2d;
        type gp_Ax2d = crate::gp::gp_Ax2d;
        type gp_Trsf = crate::gp::gp_Trsf;
        type gp_GTrsf = crate::gp::gp_GTrsf;

        type gp_Circ = crate::gp::gp_Circ;
        type gp_Lin = crate::gp::gp_Lin;

        // Edge types
        type GeomAbs_CurveType;

        // Segments
        type GC_MakeSegment;
        type GCE2d_MakeSegment;

        #[cxx_name = "construct_unique"]
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;

        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<Handle_Geom_TrimmedCurve>;
        pub fn GCE2d_MakeSegment_point_point(
            p1: &gp_Pnt2d,
            p2: &gp_Pnt2d,
        ) -> UniquePtr<Handle_Geom2d_TrimmedCurve>;

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
        ) -> UniquePtr<Handle_Geom_TrimmedCurve>;

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
        pub fn MakeCompound(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Compound>);
        pub fn MakeShell(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Shell>);
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
            geom_curve_handle: &Handle_Geom_Curve,
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
            curve_handle: &Handle_Geom2d_Curve,
            surface_handle: &Handle_Geom_Surface,
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
        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeFace_surface(
            surface: &Handle_Geom_Surface,
            edge_tolerance: f64,
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
        pub fn GetType(self: &BRepAdaptor_Curve) -> GeomAbs_CurveType;

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
        pub fn Shape(self: Pin<&mut BRepFeat_MakeDPrism>) -> &TopoDS_Shape;

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
        pub fn BRepPrimAPI_MakeSphere_ctor(
            axis: &gp_Ax2,
            r: f64,
            angle_1: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeSphere>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeSphere>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeSphere>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeSphere) -> bool;

        type BRepPrimAPI_MakeCone;

        #[cxx_name = "construct_unique"]
        pub fn BRepPrimAPI_MakeCone_ctor(
            axis: &gp_Ax2,
            r1: f64,
            r2: f64,
            h: f64,
            angle: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeCone>;

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeCone>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeCone>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeCone) -> bool;

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

        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeTorus>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepPrimAPI_MakeTorus>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepPrimAPI_MakeTorus) -> bool;

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

        #[rust_name = "variable_add_edge"]
        pub fn Add(
            self: Pin<&mut BRepFilletAPI_MakeFillet>,
            radius_values: &TColgp_Array1OfPnt2d,
            edge: &TopoDS_Edge,
        );

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

        // Offset
        type BRepOffsetAPI_MakeOffset;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_face_ctor(
            face: &TopoDS_Face,
            join: GeomAbs_JoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakeOffset_wire_ctor(
            wire: &TopoDS_Wire,
            join: GeomAbs_JoinType,
        ) -> UniquePtr<BRepOffsetAPI_MakeOffset>;

        pub fn Perform(self: Pin<&mut BRepOffsetAPI_MakeOffset>, offset: f64, alt: f64);

        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakeOffset>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepOffsetAPI_MakeOffset>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepOffsetAPI_MakeOffset) -> bool;

        type GeomAbs_JoinType;

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

        // Sweeps
        type BRepOffsetAPI_MakePipe;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipe_ctor(
            spine: &TopoDS_Wire,
            profile: &TopoDS_Shape,
        ) -> UniquePtr<BRepOffsetAPI_MakePipe>;

        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakePipe>) -> &TopoDS_Shape;

        // Sweeps with a law function
        type BRepOffsetAPI_MakePipeShell;

        #[cxx_name = "construct_unique"]
        pub fn BRepOffsetAPI_MakePipeShell_ctor(
            spine: &TopoDS_Wire,
        ) -> UniquePtr<BRepOffsetAPI_MakePipeShell>;

        pub fn SetMode(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, is_frenet: bool);

        pub fn Add(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            with_contact: bool,
            with_correction: bool,
        );

        pub fn SetLaw(
            self: Pin<&mut BRepOffsetAPI_MakePipeShell>,
            profile: &TopoDS_Shape,
            law: &Handle_Law_Function,
            with_contact: bool,
            with_correction: bool,
        );

        pub fn Build(self: Pin<&mut BRepOffsetAPI_MakePipeShell>, progress: &Message_ProgressRange);
        pub fn MakeSolid(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> bool;
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakePipeShell>) -> &TopoDS_Shape;

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

        pub type BRepAlgoAPI_BuilderAlgo;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_BuilderAlgo>) -> &TopTools_ListOfShape;

        pub type BRepAlgoAPI_Fuse;
        type BOPAlgo_GlueEnum;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Fuse_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Fuse>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Fuse) -> bool;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopTools_ListOfShape;
        pub fn SetGlue(self: Pin<&mut BRepAlgoAPI_Fuse>, glue: BOPAlgo_GlueEnum);

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
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopTools_ListOfShape;

        type BRepAlgoAPI_Common;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Common_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Common>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Common>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Common) -> bool;
        pub fn SectionEdges(self: Pin<&mut BRepAlgoAPI_Common>) -> &TopTools_ListOfShape;

        type BRepAlgoAPI_Section;

        #[cxx_name = "construct_unique"]
        pub fn BRepAlgoAPI_Section_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Section>;

        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepAlgoAPI_Section>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepAlgoAPI_Section) -> bool;
        pub fn cast_section_to_builderalgo(
            section: UniquePtr<BRepAlgoAPI_Section>,
        ) -> UniquePtr<BRepAlgoAPI_BuilderAlgo>;

        type BRepBuilderAPI_MakeSolid;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeSolid_ctor(
            shell: &TopoDS_Shell,
        ) -> UniquePtr<BRepBuilderAPI_MakeSolid>;

        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeSolid>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_MakeSolid>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_MakeSolid) -> bool;

        type BRepBuilderAPI_MakeShapeOnMesh;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_MakeShapeOnMesh_ctor(
            mesh: &Handle_Poly_Triangulation,
        ) -> UniquePtr<BRepBuilderAPI_MakeShapeOnMesh>;

        pub fn Shape(self: Pin<&mut BRepBuilderAPI_MakeShapeOnMesh>) -> &TopoDS_Shape;
        pub fn Build(
            self: Pin<&mut BRepBuilderAPI_MakeShapeOnMesh>,
            progress: &Message_ProgressRange,
        );
        pub fn IsDone(self: &BRepBuilderAPI_MakeShapeOnMesh) -> bool;

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

        type BRepBuilderAPI_GTransform;

        #[cxx_name = "construct_unique"]
        pub fn BRepBuilderAPI_GTransform_ctor(
            shape: &TopoDS_Shape,
            transform: &gp_GTrsf,
            copy: bool,
        ) -> UniquePtr<BRepBuilderAPI_GTransform>;

        pub fn Shape(self: Pin<&mut BRepBuilderAPI_GTransform>) -> &TopoDS_Shape;
        pub fn Build(self: Pin<&mut BRepBuilderAPI_GTransform>, progress: &Message_ProgressRange);
        pub fn IsDone(self: &BRepBuilderAPI_GTransform) -> bool;

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

        pub fn BRep_Tool_Surface(face: &TopoDS_Face) -> UniquePtr<Handle_Geom_Surface>;
        pub fn BRep_Tool_Curve(
            edge: &TopoDS_Edge,
            first: &mut f64,
            last: &mut f64,
        ) -> UniquePtr<Handle_Geom_Curve>;
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
        pub fn More(self: &BRepIntCurveSurface_Inter) -> bool;
        pub fn Next(self: Pin<&mut BRepIntCurveSurface_Inter>);
        pub fn BRepIntCurveSurface_Inter_face(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<TopoDS_Face>;
        pub fn BRepIntCurveSurface_Inter_point(
            intersector: &BRepIntCurveSurface_Inter,
        ) -> UniquePtr<gp_Pnt>;
        pub fn U(self: &BRepIntCurveSurface_Inter) -> f64;
        pub fn V(self: &BRepIntCurveSurface_Inter) -> f64;
        pub fn W(self: &BRepIntCurveSurface_Inter) -> f64;

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

        // Data Import
        type STEPControl_Reader;
        type IGESControl_Reader;
        type IFSelect_ReturnStatus;

        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Reader_ctor() -> UniquePtr<STEPControl_Reader>;

        #[cxx_name = "construct_unique"]
        pub fn IGESControl_Reader_ctor() -> UniquePtr<IGESControl_Reader>;

        pub fn read_step(
            reader: Pin<&mut STEPControl_Reader>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
        pub fn read_iges(
            reader: Pin<&mut IGESControl_Reader>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
        pub fn TransferRoots(
            self: Pin<&mut STEPControl_Reader>,
            progress: &Message_ProgressRange,
        ) -> i32;
        pub fn TransferRoots(
            self: Pin<&mut IGESControl_Reader>,
            progress: &Message_ProgressRange,
        ) -> i32;
        pub fn one_shape_step(reader: &STEPControl_Reader) -> UniquePtr<TopoDS_Shape>;
        pub fn one_shape_iges(reader: &IGESControl_Reader) -> UniquePtr<TopoDS_Shape>;

        // Data Export
        type STEPControl_Writer;
        type IGESControl_Writer;

        #[cxx_name = "construct_unique"]
        pub fn STEPControl_Writer_ctor() -> UniquePtr<STEPControl_Writer>;

        #[cxx_name = "construct_unique"]
        pub fn IGESControl_Writer_ctor() -> UniquePtr<IGESControl_Writer>;

        pub fn transfer_shape(
            writer: Pin<&mut STEPControl_Writer>,
            shape: &TopoDS_Shape,
        ) -> IFSelect_ReturnStatus;
        pub fn add_shape(writer: Pin<&mut IGESControl_Writer>, shape: &TopoDS_Shape) -> bool;
        pub fn compute_model(writer: Pin<&mut IGESControl_Writer>);
        pub fn write_step(
            writer: Pin<&mut STEPControl_Writer>,
            filename: String,
        ) -> IFSelect_ReturnStatus;
        pub fn write_iges(writer: Pin<&mut IGESControl_Writer>, filename: String) -> bool;

        type TopLoc_Location = crate::top_loc::TopLoc_Location;

        type Poly_Triangulation = crate::poly::Poly_Triangulation;
        type Handle_Poly_Triangulation;

        pub fn IsNull(self: &Handle_Poly_Triangulation) -> bool;
        #[cxx_name = "handle_try_deref"]
        pub fn Handle_Poly_Triangulation_Get(
            handle: &Handle_Poly_Triangulation,
        ) -> Result<&Poly_Triangulation>;

        pub fn compute_normals(face: &TopoDS_Face, triangulation: &Handle_Poly_Triangulation);

        // This is dumb:
        // https://cxx.rs/extern-c++.html#explicit-shim-trait-impls
        #[cxx_name = "list_to_vector"]
        pub fn shape_list_to_vector(
            list: &TopTools_ListOfShape,
        ) -> UniquePtr<CxxVector<TopoDS_Shape>>;
    }

    impl UniquePtr<Handle_TopTools_HSequenceOfShape> {}
    impl UniquePtr<Handle_Law_Function> {}
    impl UniquePtr<Handle_Geom_CylindricalSurface> {}
    impl UniquePtr<Handle_Geom_BezierSurface> {}
    impl UniquePtr<Handle_Geom_BezierCurve> {}
    impl UniquePtr<Handle_Geom_Plane> {}
    impl UniquePtr<Handle_Geom2d_Ellipse> {}
    impl UniquePtr<Handle_Geom2d_Curve> {}
    impl UniquePtr<Handle_Geom_BSplineCurve> {}
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
