pub mod b_rep;
pub mod b_rep_adaptor;
pub mod b_rep_algo_api;
pub mod b_rep_bnd_lib;
pub mod b_rep_builder_api;
pub mod b_rep_feat;
pub mod b_rep_fillet_api;
pub mod b_rep_g_prop;
pub mod b_rep_mesh;
pub mod b_rep_offset_api;
pub mod b_rep_prim_api;
pub mod b_rep_tools;
pub mod bin_tools;
pub mod bnd;
pub mod g_prop;
pub mod gc;
pub mod gc_pnts;
pub mod geom;
pub mod geom2d;
pub mod geom_abs;
pub mod geom_api;
pub mod gp;
pub mod iges_control;
pub mod law;
pub mod poly;
pub mod shape_analysis;
pub mod shape_upgrade;
pub mod step_control;
pub mod stl_api;
pub mod t_col_gp;
pub mod top_exp;
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
        type GeomAbs_CurveType = crate::geom_abs::GeomAbs_CurveType;
        type GeomAbs_JoinType = crate::geom_abs::GeomAbs_JoinType;

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

        type TopoDS_Builder;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Compound_ctor() -> UniquePtr<TopoDS_Compound>;

        #[cxx_name = "construct_unique"]
        pub fn TopoDS_Shell_ctor() -> UniquePtr<TopoDS_Shell>;

        pub fn MakeCompound(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Compound>);
        pub fn MakeShell(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Shell>);
        pub fn Add(self: &TopoDS_Builder, shape: Pin<&mut TopoDS_Shape>, compound: &TopoDS_Shape);

        // BRepLib
        pub fn BRepLibBuildCurves3d(shape: &TopoDS_Shape) -> bool;

        type BOPAlgo_GlueEnum;

        // Topology Explorer
        type TopAbs_ShapeEnum;

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

        type IFSelect_ReturnStatus;

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
    impl UniquePtr<Handle_Geom_TrimmedCurve> {}
    impl UniquePtr<Handle_Geom2d_TrimmedCurve> {}
    impl UniquePtr<Handle_Geom_Surface> {}
    impl UniquePtr<Handle_Geom_Curve> {}
    impl UniquePtr<Handle_Poly_Triangulation> {}
}

// Gross, but is this okay?
unsafe impl Send for ffi::TopoDS_Edge {}
unsafe impl Send for ffi::TopoDS_Wire {}
unsafe impl Send for ffi::TopoDS_Face {}
unsafe impl Send for ffi::TopoDS_Shell {}
unsafe impl Send for ffi::TopoDS_Solid {}
unsafe impl Send for ffi::TopoDS_Compound {}
unsafe impl Send for ffi::TopoDS_Shape {}
