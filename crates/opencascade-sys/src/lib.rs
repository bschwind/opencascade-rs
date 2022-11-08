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
        type HandleGeom2d_Curve;
        type HandleGeom2d_Ellipse;
        type HandleGeom2d_TrimmedCurve;
        type HandleGeom_CylindricalSurface;

        pub fn DynamicType(surface: &HandleGeomSurface) -> &HandleStandardType;
        pub fn type_name(handle: &HandleStandardType) -> String;

        pub fn new_HandleGeomCurve_from_HandleGeom_TrimmedCurve(
            trimmed_curve_handle: &HandleGeomTrimmedCurve,
        ) -> UniquePtr<HandleGeomCurve>;

        pub fn new_HandleGeomPlane_from_HandleGeomSurface(
            geom_surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<HandleGeomPlane>;

        // General Shape Stuff
        /// Takes a shape reference (typically one returned from a shape builder API) and puts
        /// it in a unique_ptr.
        pub fn new_shape(shape: &TopoDS_Shape) -> UniquePtr<TopoDS_Shape>;

        // Collections
        type TopTools_ListOfShape;
        pub fn new_list_of_shape() -> UniquePtr<TopTools_ListOfShape>;
        pub fn shape_list_append_face(list: Pin<&mut TopTools_ListOfShape>, face: &TopoDS_Face);

        // Geometry
        type Geom_TrimmedCurve;
        type Geom_CylindricalSurface;
        type Geom2d_Ellipse;
        type Geom2d_Curve;
        type Geom2d_TrimmedCurve;

        pub fn handle_geom_plane_location(plane: &HandleGeomPlane) -> &gp_Pnt;

        // TODO - return a Handle?
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

        pub fn new_point(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;
        pub fn X(self: &gp_Pnt) -> f64;
        pub fn Y(self: &gp_Pnt) -> f64;
        pub fn Z(self: &gp_Pnt) -> f64;
        pub fn Distance(self: &gp_Pnt, other: &gp_Pnt) -> f64;

        pub fn new_point_2d(x: f64, y: f64) -> UniquePtr<gp_Pnt2d>;

        type gp_Vec;
        pub fn new_vec(x: f64, y: f64, z: f64) -> UniquePtr<gp_Vec>;

        // Segments
        type GC_MakeSegment;
        type GCE2d_MakeSegment;
        pub fn GC_MakeSegment_point_point(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<GC_MakeSegment>;
        pub fn GC_MakeSegment_Value(arc: &GC_MakeSegment) -> UniquePtr<HandleGeomTrimmedCurve>;
        pub fn GCE2d_MakeSegment_point_point(
            p1: &gp_Pnt2d,
            p2: &gp_Pnt2d,
        ) -> UniquePtr<HandleGeom2d_TrimmedCurve>;

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
        pub fn TopoDS_cast_to_face(shape: &TopoDS_Shape) -> UniquePtr<TopoDS_Face>;

        pub fn IsNull(self: &TopoDS_Shape) -> bool;
        pub fn IsEqual(self: &TopoDS_Shape, other: &TopoDS_Shape) -> bool;

        // Compound Shapes
        type TopoDS_Compound;
        pub fn TopoDS_Compound_as_shape(
            compound: UniquePtr<TopoDS_Compound>,
        ) -> UniquePtr<TopoDS_Shape>;

        type BRep_Builder;
        type TopoDS_Builder;
        pub fn TopoDS_Compound_ctor() -> UniquePtr<TopoDS_Compound>;
        pub fn BRep_Builder_ctor() -> UniquePtr<BRep_Builder>;
        pub fn BRep_Builder_upcast_to_topods_builder(builder: &BRep_Builder) -> &TopoDS_Builder;
        pub fn MakeCompound(self: &TopoDS_Builder, compound: Pin<&mut TopoDS_Compound>);
        pub fn Add(self: &TopoDS_Builder, shape: Pin<&mut TopoDS_Shape>, compound: &TopoDS_Shape);

        // BRepBuilder
        type BRepBuilderAPI_MakeEdge;
        type TopoDS_Vertex;
        pub fn BRepBuilderAPI_MakeEdge_HandleGeomCurve(
            geom_curve_handle: &HandleGeomCurve,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;
        pub fn BRepBuilderAPI_MakeEdge_CurveSurface2d(
            curve_handle: &HandleGeom2d_Curve,
            surface_handle: &HandleGeomSurface,
        ) -> UniquePtr<BRepBuilderAPI_MakeEdge>;
        pub fn Vertex1(self: &BRepBuilderAPI_MakeEdge) -> &TopoDS_Vertex;
        pub fn Edge(self: Pin<&mut BRepBuilderAPI_MakeEdge>) -> &TopoDS_Edge;

        type BRepBuilderAPI_MakeWire;
        pub fn BRepBuilderAPI_MakeWire_ctor() -> UniquePtr<BRepBuilderAPI_MakeWire>;
        pub fn BRepBuilderAPI_MakeWire_edge_edge(
            edge_1: &TopoDS_Edge,
            edge_2: &TopoDS_Edge,
        ) -> UniquePtr<BRepBuilderAPI_MakeWire>;
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

        type BRepPrimAPI_MakeRevol;
        pub fn BRepPrimAPI_MakeRevol_ctor(
            shape: &TopoDS_Shape,
            axis: &gp_Ax1,
            angle: f64,
            copy: bool,
        ) -> UniquePtr<BRepPrimAPI_MakeRevol>;
        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeRevol>) -> &TopoDS_Shape;
        pub fn IsDone(self: &BRepPrimAPI_MakeRevol) -> bool;

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

        type BRepPrimAPI_MakeBox;
        pub fn BRepPrimAPI_MakeBox_ctor(
            point: &gp_Pnt,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<BRepPrimAPI_MakeBox>;
        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeBox>) -> &TopoDS_Shape;

        type BRepPrimAPI_MakeSphere;
        pub fn BRepPrimAPI_MakeSphere_ctor(r: f64) -> UniquePtr<BRepPrimAPI_MakeSphere>;
        pub fn Shape(self: Pin<&mut BRepPrimAPI_MakeSphere>) -> &TopoDS_Shape;

        // BRepLib
        pub fn BRepLibBuildCurves3d(shape: &TopoDS_Shape) -> bool;

        // Fillets
        type BRepFilletAPI_MakeFillet;
        pub fn BRepFilletAPI_MakeFillet_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeFillet>;
        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepFilletAPI_MakeFillet>, radius: f64, edge: &TopoDS_Edge);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeFillet>) -> &TopoDS_Shape;

        // Chamfers
        type BRepFilletAPI_MakeChamfer;
        pub fn BRepFilletAPI_MakeChamfer_ctor(
            shape: &TopoDS_Shape,
        ) -> UniquePtr<BRepFilletAPI_MakeChamfer>;
        #[rust_name = "add_edge"]
        pub fn Add(self: Pin<&mut BRepFilletAPI_MakeChamfer>, distance: f64, edge: &TopoDS_Edge);
        pub fn Shape(self: Pin<&mut BRepFilletAPI_MakeChamfer>) -> &TopoDS_Shape;

        // Solids
        type BRepOffsetAPI_MakeThickSolid;
        pub fn BRepOffsetAPI_MakeThickSolid_ctor() -> UniquePtr<BRepOffsetAPI_MakeThickSolid>;
        pub fn MakeThickSolidByJoin(
            make_thick_solid: Pin<&mut BRepOffsetAPI_MakeThickSolid>,
            shape: &TopoDS_Shape,
            closing_faces: &TopTools_ListOfShape,
            offset: f64,
            tolerance: f64,
        );
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_MakeThickSolid>) -> &TopoDS_Shape;

        // Lofting
        type BRepOffsetAPI_ThruSections;
        pub fn BRepOffsetAPI_ThruSections_ctor(
            is_solid: bool,
        ) -> UniquePtr<BRepOffsetAPI_ThruSections>;
        pub fn AddWire(self: Pin<&mut BRepOffsetAPI_ThruSections>, wire: &TopoDS_Wire);
        pub fn CheckCompatibility(self: Pin<&mut BRepOffsetAPI_ThruSections>, check: bool);
        pub fn Shape(self: Pin<&mut BRepOffsetAPI_ThruSections>) -> &TopoDS_Shape;

        // Boolean Operations
        type BRepAlgoAPI_Fuse;
        pub fn BRepAlgoAPI_Fuse_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Fuse>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Fuse>) -> &TopoDS_Shape;

        type BRepAlgoAPI_Cut;
        pub fn BRepAlgoAPI_Cut_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Cut>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Cut>) -> &TopoDS_Shape;

        type BRepAlgoAPI_Section;
        pub fn BRepAlgoAPI_Section_ctor(
            shape_1: &TopoDS_Shape,
            shape_2: &TopoDS_Shape,
        ) -> UniquePtr<BRepAlgoAPI_Section>;
        pub fn Shape(self: Pin<&mut BRepAlgoAPI_Section>) -> &TopoDS_Shape;

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

        pub fn gp_Ax2_ctor(origin: &gp_Pnt, main_dir: &gp_Dir) -> UniquePtr<gp_Ax2>;
        pub fn gp_Ax3_from_gp_Ax2(axis: &gp_Ax2) -> UniquePtr<gp_Ax3>;
        pub fn gp_Dir2d_ctor(x: f64, y: f64) -> UniquePtr<gp_Dir2d>;
        pub fn gp_Ax2d_ctor(point: &gp_Pnt2d, dir: &gp_Dir2d) -> UniquePtr<gp_Ax2d>;

        // Transforms
        type gp_Trsf;
        pub fn new_transform() -> UniquePtr<gp_Trsf>;

        #[rust_name = "set_mirror_axis"]
        pub fn SetMirror(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1);
        pub fn SetRotation(self: Pin<&mut gp_Trsf>, axis: &gp_Ax1, angle: f64);
        pub fn SetScale(self: Pin<&mut gp_Trsf>, point: &gp_Pnt, scale: f64);
        pub fn SetTranslation(self: Pin<&mut gp_Trsf>, point1: &gp_Pnt, point2: &gp_Pnt);

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
